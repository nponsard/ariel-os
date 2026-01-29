# coapcore

A CoAP security tool for embedded devices,
supporting OSCORE/EDHOC and managing credentials.

This crate is maintained as part of Ariel OS,
whose CoAP stack integrates it and manages server access policies.
Nothing in this crate depends on Ariel OS, but some examples may refer to it.

🚧 This crate is under active development;
breaking changes will be made as necessary.
It currently only handles the server side of CoAP exchanges.
At runtime, there is more copying of messages than is generally preferred;
those result from limitations of underlying tools and are being addressed there.
