/// Adapted from trouble-host, modified to use a custom MTU
use core::cell::RefCell;
use embassy_sync::blocking_mutex::{
    Mutex,
    raw::{CriticalSectionRawMutex, RawMutex},
};
use trouble_host::{Packet, PacketPool};

use super::config;
struct PacketBuf<const MTU: usize> {
    buf: [u8; MTU],
    free: bool,
}

impl<const MTU: usize> PacketBuf<MTU> {
    const NEW: PacketBuf<MTU> = PacketBuf::new();

    pub(crate) const fn new() -> Self {
        Self {
            buf: [0; MTU],
            free: true,
        }
    }
}

struct State<const MTU: usize, const N: usize> {
    packets: [PacketBuf<MTU>; N],
}

impl<const MTU: usize, const N: usize> State<MTU, N> {
    pub(crate) const fn new() -> Self {
        Self {
            packets: [PacketBuf::NEW; N],
        }
    }

    fn alloc(&mut self) -> Option<PacketRef<MTU>> {
        for (idx, packet) in self.packets.iter_mut().enumerate() {
            if packet.free {
                // info!("[{}] alloc {}", id.0, idx);
                packet.free = false;
                packet.buf.iter_mut().for_each(|b| *b = 0);
                return Some(PacketRef {
                    idx,
                    buf: packet.buf.as_mut_ptr(),
                });
            }
        }
        None
    }

    fn free(&mut self, p_ref: &PacketRef<MTU>) {
        // info!("[{}] free {}", id.0, p_ref.idx);
        self.packets[p_ref.idx].free = true;
    }
}

/// A packet pool holds a pool of packet buffers that can be dynamically allocated
/// and free'd.
pub struct StaticPacketPool<M: RawMutex, const MTU: usize, const N: usize> {
    state: Mutex<M, RefCell<State<MTU, N>>>,
}

impl<M: RawMutex, const MTU: usize, const N: usize> Default for StaticPacketPool<M, MTU, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: RawMutex, const MTU: usize, const N: usize> StaticPacketPool<M, MTU, N> {
    /// Create a new packet pool with the given QoS policy
    const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(State::new())),
        }
    }

    fn alloc(&self) -> Option<PacketRef<MTU>> {
        self.state.lock(|state| {
            let mut state = state.borrow_mut();
            state.alloc()
        })
    }

    fn free(&self, p_ref: &PacketRef<MTU>) {
        self.state.lock(|state| {
            let mut state = state.borrow_mut();
            state.free(p_ref);
        });
    }
}

/// Represents a reference to a packet.
#[repr(C)]
pub struct PacketRef<const MTU: usize> {
    idx: usize,
    buf: *mut u8,
}

/// Global default packet pool.
pub type BlePacketPool =
    StaticPacketPool<CriticalSectionRawMutex, { config::MTU }, { config::MAX_PACKETS }>;

static DEFAULT_POOL: StaticPacketPool<
    CriticalSectionRawMutex,
    { config::MTU },
    { config::MAX_PACKETS },
> = StaticPacketPool::new();

impl PacketPool for BlePacketPool {
    type Packet = DefaultPacket;
    const MTU: usize = { config::MTU };
    fn capacity() -> usize {
        config::MAX_PACKETS
    }

    fn allocate() -> Option<DefaultPacket> {
        DEFAULT_POOL.alloc().map(|p| DefaultPacket {
            p_ref: p,
            pool: &DEFAULT_POOL,
        })
    }
}

/// Type representing the packet from the default packet pool.
pub struct DefaultPacket {
    p_ref: PacketRef<{ config::MTU }>,
    pool: &'static BlePacketPool,
}

impl Packet for DefaultPacket {}
impl AsRef<[u8]> for DefaultPacket {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.p_ref.buf, config::MTU) }
    }
}

impl AsMut<[u8]> for DefaultPacket {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.p_ref.buf, config::MTU) }
    }
}

impl Drop for DefaultPacket {
    fn drop(&mut self) {
        self.pool.free(&self.p_ref);
    }
}
