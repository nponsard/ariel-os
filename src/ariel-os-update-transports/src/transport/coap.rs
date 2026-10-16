use core::net::SocketAddr;

use ariel_os_log::error;
use coap_request::Stack;
use coap_request_implementations::AsUriPath as _;
use embedded_nal_async::Dns;
use embedded_nal_coap::RequestingCoAPClient;

use nourl::UrlScheme;

use crate::Error;

const CONCURRENT_REQUESTS: usize = 3;

/// Return the Block option size exponent (szx) based on the block size.
///
/// See <https://www.rfc-editor.org/info/rfc7959/#section-2.2>.
///
/// # Errors
///
/// If `chunk_size` is not of the form `2 ** (4 + x)` with `0 <= x <= 6`.
pub(crate) const fn chunk_size_to_block2szx(chunk_size: usize) -> Result<u8, ()> {
    match chunk_size {
        16 => Ok(0),
        32 => Ok(1),
        64 => Ok(2),
        128 => Ok(3),
        256 => Ok(4),
        512 => Ok(5),
        1024 => Ok(6),
        _ => Err(()),
    }
}

pub struct NetworkTransportClient<'uri, const CHUNK_SIZE: usize> {
    pub(crate) client: RequestingCoAPClient<'static, CONCURRENT_REQUESTS>,
    pub(crate) path: &'uri str,
}

/// Create a new CoAP client to send requests to a given uri.
///
/// # Errors
///
/// * If the URI parsing fails.
/// * If the DNS resolution fails.
pub(crate) async fn new_client<'uri, DNS: Dns, const CHUNK_SIZE: usize>(
    dns_client: &DNS,
    uri: &'uri str,
) -> Result<NetworkTransportClient<'uri, CHUNK_SIZE>, Error> {
    const {
        assert!(
            chunk_size_to_block2szx(CHUNK_SIZE).is_ok(),
            "CoAP only supports chunk sizes of the form 2 **(x + 4) with 0<= x <= 6"
        );
    }

    let Ok(uri) = nourl::Url::parse(uri) else {
        return Err(Error::InvalidUri);
    };

    match uri.scheme() {
        UrlScheme::CoAP | UrlScheme::CoAPS => {}
        _ => return Err(Error::InvalidUri),
    }

    let peer_socket_addr = if let Some(socket_addr) = uri.host_socket_address() {
        socket_addr
    } else {
        // TODO: might want to restrict to one type of IP addresses.
        if let Ok(ip_addr) = dns_client
            .get_host_by_name(uri.host(), embedded_nal_async::AddrType::Either)
            .await
        {
            SocketAddr::new(ip_addr, uri.port_or_default())
        } else {
            // TODO: could introduce an error for DNS.
            return Err(Error::Transport);
        }
    };

    let client = ariel_os_coap::coap_client().await;
    let client = client.to(peer_socket_addr);

    ariel_os_log::trace!("Addr: {}, path: {}", &peer_socket_addr, uri.path());

    Ok(NetworkTransportClient {
        client,
        path: uri.path(),
    })
}

impl<const CHUNK_SIZE: usize> NetworkTransportClient<'_, CHUNK_SIZE> {
    /// Fetch and return a single chunk of data from the URI.
    ///
    /// # Errors
    ///
    /// * If the request fails or returns garbage data.
    pub async fn get<'buf, const N: usize>(
        &mut self,
        chunk_index: u32,
        buf: &'buf mut [u8; N],
    ) -> Result<(&'buf mut [u8], bool), Error> {
        const {
            assert!(N >= CHUNK_SIZE);
        }
        let request = SuitPayloadRequest::<CHUNK_SIZE>::new(self.path).set_blocknum(chunk_index);
        let Some((has_more, coap_chunk)) = self
            .client
            .request(request)
            .await
            .map_err(|_| Error::Transport)?
        else {
            return Err(Error::Transport);
        };

        // NOTE: len <= CHUNK_SIZE by construction
        let len = coap_chunk.len;
        buf[..len].copy_from_slice(&coap_chunk.bytes[..len]);
        Ok((&mut buf[..len], has_more))
    }
}

// Copied from
// <https://github.com/bergzand/Ariel-os/blob/0a44aaa8178083588f82b3f4187051ab4ec75365/examples/rustweek-demo/src/suit.rs#L41-L102>.
#[derive(Debug, Clone)]
struct SuitPayloadRequest<'a, const CHUNK_SIZE: usize> {
    block_num: u32,
    path: &'a str,
}

impl<'a, const CHUNK_SIZE: usize> SuitPayloadRequest<'a, CHUNK_SIZE> {
    fn new(path: &'a str) -> Self {
        Self { block_num: 0, path }
    }

    fn set_blocknum(&self, block_num: u32) -> Self {
        Self {
            block_num,
            path: self.path,
        }
    }
}

impl<const CHUNK_SIZE: usize>
    coap_request::Request<RequestingCoAPClient<'static, CONCURRENT_REQUESTS>>
    for SuitPayloadRequest<'_, CHUNK_SIZE>
{
    type Output = Option<(bool, CoapChunk<CHUNK_SIZE>)>;

    type Carry = ();

    fn build_request(
        &mut self,
        request: &mut <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<
            '_,
        >,
    ) -> impl Future<
        Output = Result<
            Self::Carry,
            <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestUnionError,
        >,
    > {
        use coap_message::MinimalWritableMessage;
        use coap_message::OptionNumber as _;

        type Request<'a> =
            <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<'a>;
        type OptionNum<'a> = <Request<'a> as MinimalWritableMessage>::OptionNumber;

        let block2szx = chunk_size_to_block2szx(CHUNK_SIZE).unwrap();

        request.set_code(coap_numbers::code::GET);
        for part in self.path.as_uri_path() {
            if let Err(e) = request.add_option(
                OptionNum::new(coap_numbers::option::URI_PATH).unwrap(),
                part.as_bytes(),
            ) {
                return core::future::ready(Err(e));
            }
        }
        if let Err(e) = request.add_option_uint(
            OptionNum::new(coap_numbers::option::BLOCK2).unwrap(),
            // NUM | M = 0 | szx (3 bits)
            self.block_num << 4 | u32::from(block2szx),
        ) {
            return core::future::ready(Err(e));
        }
        core::future::ready(Ok(()))
    }

    fn process_response(
        &mut self,
        response: &<RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::ResponseMessage<
            '_,
        >,
        _carry: Self::Carry,
    ) -> impl Future<Output = Self::Output> {
        use coap_message::{MessageOption as _, ReadableMessage as _};

        let Some(Some(block2)) = response.options().find_map(|o| {
            (o.number() == coap_numbers::option::BLOCK2).then(|| o.value_uint::<u32>())
        }) else {
            error!("No block number in response");
            return core::future::ready(None);
        };

        let (blocknum, more, size) = parse_block2(block2);

        if usize::from(size) != CHUNK_SIZE && blocknum != self.block_num {
            error!("Unexpected block {} of size {}", blocknum, size);
            return core::future::ready(None);
        }

        let payload = response.payload();

        // Should by all means be unreachable.
        if payload.len() > CHUNK_SIZE {
            error!("Unexpected payload of size {}", payload.len());
            return core::future::ready(None);
        }
        let mut bytes = [0u8; CHUNK_SIZE];
        bytes[..payload.len()].copy_from_slice(payload);
        let out = CoapChunk {
            bytes,
            len: payload.len(),
        };
        core::future::ready(Some((more, out)))
    }
}

#[derive(Debug)]
struct CoapChunk<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

/// See <https://www.rfc-editor.org/info/rfc7959/#section-2.2>.
fn parse_block2(block2: u32) -> (u32, bool, u16) {
    let szx = block2 & 0x7;

    let num = block2 >> 4;
    let more = (block2 & 0x8) != 0;
    let size = 1 << (4 + szx);
    (num, more, size)
}
