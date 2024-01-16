use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::OriginDimensions;
use epd_waveshare::prelude::Display;
use epd_waveshare::prelude::DisplayRotation;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::Gpio10;
use esp_idf_hal::gpio::Gpio17;
use esp_idf_hal::gpio::Gpio18;
use esp_idf_hal::gpio::Gpio21;
use esp_idf_hal::gpio::Gpio38;
use esp_idf_hal::gpio::Gpio48;
use esp_idf_hal::spi::SPI2;
use esp_idf_hal::units::Hertz;

use epd_waveshare::epd2in9_v2::Epd2in9;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::Input;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::spi;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;

use esp_idf_hal::delay;

use epd_waveshare::{prelude::WaveshareDisplay, *};
// trying to avoid stack overflows maybe :()
pub struct DisplayBoxed(Box<epd2in9_v2::Display2in9>);

impl DrawTarget for DisplayBoxed {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.0.clear(color)
    }
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.0.draw_iter(pixels)
    }
    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        self.0.fill_solid(area, color)
    }
    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.0.fill_contiguous(area, colors)
    }
}
impl DisplayBoxed {
    pub fn set_rotation(&mut self, rot: DisplayRotation) {
        self.0.set_rotation(rot)
    }
    pub fn clear(&mut self, color: BinaryColor) -> anyhow::Result<()> {
        self.0.clear(color).map_err(|err| anyhow::Error::from(err))
    }
    pub fn buffer(&self) -> &[u8] {
        &self.0.buffer()
    }
}
impl OriginDimensions for DisplayBoxed {
    fn size(&self) -> embedded_graphics::prelude::Size {
        self.0.size()
    }
}

pub fn init_display<'a>(
    spi2: SPI2,
    gpio48: Gpio48,
    gpio38: Gpio38,
    gpio21: Gpio21,
    gpio10: Gpio10,
    gpio18: Gpio18,
    gpio17: Gpio17,
) -> anyhow::Result<(
    DisplayBoxed,
    Epd2in9<
        SpiDeviceDriver<'a, SpiDriver<'a>>,
        PinDriver<'a, Gpio21, Output>,
        PinDriver<'a, Gpio10, Input>,
        PinDriver<'a, Gpio18, Output>,
        PinDriver<'a, Gpio17, Output>,
        Ets,
    >,
    SpiDeviceDriver<'a, SpiDriver<'a>>,
)> {
    let mut driver = spi::SpiDeviceDriver::new_single(
        spi2,
        gpio48,
        gpio38,
        Option::<gpio::AnyIOPin>::None,
        Option::<gpio::AnyOutputPin>::None,
        &spi::SpiDriverConfig::new().dma(spi::Dma::Disabled),
        &spi::SpiConfig::new().baudrate(Hertz::from(26)),
    )?;

    let cs = gpio::PinDriver::output(gpio21)?;

    let busy = gpio::PinDriver::input(gpio10)?;

    let dc = gpio::PinDriver::output(gpio18)?;

    let rst = gpio::PinDriver::output(gpio17)?;

    let epd = match epd2in9_v2::Epd2in9::new(&mut driver, cs, busy, dc, rst, &mut delay::Ets) {
        std::result::Result::Ok(epd) => epd,
        Err(e) => return Err(anyhow::Error::new(e)),
    };

    let display = Box::new(epd2in9_v2::Display2in9::default());
    let dis_boxed = DisplayBoxed { 0: display };
    return Ok((dis_boxed, epd, driver));
}
