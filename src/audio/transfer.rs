use hal::dma;
use hal::gpio;
use hal::sai::{self, I2sUsers, SaiChannel, SaiI2sExt};
use stm32h7xx_hal as hal;

use hal::pac;

use super::{DMA_BUFFER_LENGTH, FS};

pub type Sai1Pins = (
    gpio::gpioe::PE2<gpio::Alternate<gpio::AF6>>, // MCLK_A
    gpio::gpioe::PE5<gpio::Alternate<gpio::AF6>>, // SCK_A
    gpio::gpioe::PE4<gpio::Alternate<gpio::AF6>>, // FS_A
    gpio::gpioe::PE6<gpio::Alternate<gpio::AF6>>, // SD_A
    Option<gpio::gpioe::PE3<gpio::Alternate<gpio::AF6>>>, // SD_B
);

type TransferDma1Str0 = dma::Transfer<
    dma::dma::Stream0<pac::DMA1>,
    pac::SAI1,
    dma::MemoryToPeripheral,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

type TransferDma1Str1 = dma::Transfer<
    dma::dma::Stream1<pac::DMA1>,
    pac::SAI1,
    dma::PeripheralToMemory,
    &'static mut [u32; DMA_BUFFER_LENGTH],
    dma::DBTransfer,
>;

pub struct Transfer {
    dma1_str0: TransferDma1Str0,
    dma1_str1: TransferDma1Str1,
    sai1: hal::sai::Sai<pac::SAI1, hal::sai::I2S>,
}

impl Transfer {
    pub fn init(
        clocks: &hal::rcc::CoreClocks,
        sai1_rec: hal::rcc::rec::Sai1,
        sai1_pins: Sai1Pins,
        dma1_rec: hal::rcc::rec::Dma1,
        tx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
        rx_buffer: &'static mut [u32; DMA_BUFFER_LENGTH],
    ) -> Self {
        let dma1_streams =
            dma::dma::StreamsTuple::new(unsafe { pac::Peripherals::steal().DMA1 }, dma1_rec);

        let dma_config = dma::dma::DmaConfig::default()
            .priority(dma::config::Priority::High)
            .memory_increment(true)
            .peripheral_increment(false)
            .circular_buffer(true)
            .fifo_enable(false);
        let dma1_str0: dma::Transfer<_, _, dma::MemoryToPeripheral, _, _> = dma::Transfer::init(
            dma1_streams.0,
            unsafe { pac::Peripherals::steal().SAI1 },
            tx_buffer,
            None,
            dma_config,
        );

        let dma_config = dma_config
            .transfer_complete_interrupt(true)
            .half_transfer_interrupt(true);
        let dma1_str1: dma::Transfer<_, _, dma::PeripheralToMemory, _, _> = dma::Transfer::init(
            dma1_streams.1,
            unsafe { pac::Peripherals::steal().SAI1 },
            rx_buffer,
            None,
            dma_config,
        );

        let sai1_a_config = sai::I2SChanConfig::new(sai::I2SDir::Tx)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Falling);
        let sai1_b_config = sai::I2SChanConfig::new(sai::I2SDir::Rx)
            .set_sync_type(sai::I2SSync::Internal)
            .set_frame_sync_active_high(true)
            .set_clock_strobe(sai::I2SClockStrobe::Rising);
        let sai1 = unsafe { pac::Peripherals::steal().SAI1 }.i2s_ch_a(
            sai1_pins,
            FS,
            sai::I2SDataSize::BITS_24,
            sai1_rec,
            clocks,
            I2sUsers::new(sai1_a_config).add_slave(sai1_b_config),
        );

        Self {
            dma1_str0,
            dma1_str1,
            sai1,
        }
    }

    pub fn start(&mut self) {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::DMA1_STR1);
        }

        let dma1_str0 = &mut self.dma1_str0;
        let dma1_str1 = &mut self.dma1_str1;
        let sai1 = &mut self.sai1;

        dma1_str1.start(|_sai1_rb| {
            sai1.enable_dma(SaiChannel::ChannelB);
        });

        dma1_str0.start(|sai1_rb| {
            sai1.enable_dma(SaiChannel::ChannelA);

            // wait until sai1's fifo starts to receive data
            while sai1_rb.cha.sr.read().flvl().is_empty() {}

            sai1.enable();

            use hal::traits::i2s::FullDuplex;
            sai1.try_send(0, 0).unwrap();
        });
    }

    pub fn examine_interrupt(&mut self) -> Result<State, ()> {
        if self.dma1_str1.get_half_transfer_flag() {
            self.dma1_str1.clear_half_transfer_interrupt();
            Ok(State::HalfSent)
        } else if self.dma1_str1.get_transfer_complete_flag() {
            self.dma1_str1.clear_transfer_complete_interrupt();
            Ok(State::FullSent)
        } else {
            Err(())
        }
    }
}

pub enum State {
    HalfSent,
    FullSent,
}