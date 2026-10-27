// embassy-rp doesn't support routing channels interrupts to `DMA_IRQ_1`, DMA channels then have
// to share the interrupt that is supported, DMA_IRQ_0.
// `bind_interrupts!` supports routing interrupts to multiple handlers, separated by a comma.

use embassy_rp::{bind_interrupts, peripherals};

#[cfg(feature = "_cyw43")]
#[expect(non_camel_case_types)]
type CYW43_DMA_CHANNEL = peripherals::DMA_CH0;

#[cfg(feature = "storage")]
#[expect(non_camel_case_types)]
pub(crate) type FLASH_DMA_CHANNEL = peripherals::DMA_CH1;

bind_interrupts!(pub(crate) struct DmaIrqs {
    DMA_IRQ_0 =>
    #[cfg(feature = "_cyw43")]
    embassy_rp::dma::InterruptHandler<CYW43_DMA_CHANNEL>,
    #[cfg(feature = "storage")]
    embassy_rp::dma::InterruptHandler<FLASH_DMA_CHANNEL>
    ;
});
