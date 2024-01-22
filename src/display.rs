use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::OriginDimensions;
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::*;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

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
// trying to avoid stack overflows maybe :() so we use heap alloc
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
impl Display for DisplayBoxed {
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    fn rotation(&self) -> DisplayRotation {
        self.0.rotation()
    }
    fn draw_helper(
        &mut self,
        width: u32,
        height: u32,
        pixel: embedded_graphics::Pixel<BinaryColor>,
    ) -> Result<(), Self::Error> {
        self.0.draw_helper(width, height, pixel)
    }
    fn clear_buffer(&mut self, background_color: prelude::Color) {
        self.0.clear_buffer(background_color)
    }
    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.0.set_rotation(rotation)
    }
    fn get_mut_buffer(&mut self) -> &mut [u8] {
        self.0.get_mut_buffer()
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
    let mut dis_boxed = DisplayBoxed { 0: display };

    dis_boxed.set_rotation(DisplayRotation::Rotate90);
    dis_boxed.clear(BinaryColor::Off)?;
    return Ok((dis_boxed, epd, driver));
}
impl DisplayBoxed {
    pub fn draw_default_display<'a>(
        &mut self,
        style: MonoTextStyle<'a, BinaryColor>,
    ) -> anyhow::Result<()> {
        Circle::new(Point::new(55, 2), 40)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(2)
                    .build(),
            )
            .draw(self)?;
        Circle::new(Point::new(55, 86), 40)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(2)
                    .build(),
            )
            .draw(self)?;
        Circle::new(Point::new(13, 44), 40)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(2)
                    .stroke_color(BinaryColor::On)
                    .build(),
            )
            .draw(self)?;
        Circle::new(Point::new(97, 44), 40)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(2)
                    .stroke_color(BinaryColor::On)
                    .build(),
            )
            .draw(self)?;
        Line::new(Point::new(149, 0), Point::new(149, 128))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(2)
                    .build(),
            )
            .draw(self)?;
        Line::new(Point::new(104, 0), Point::new(104, 20))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(2)
                    .build(),
            )
            .draw(self)?;
        Line::new(Point::new(104, 20), Point::new(149, 20))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(BinaryColor::On)
                    .stroke_width(2)
                    .build(),
            )
            .draw(self)?;

        self.draw_text(style, "0.00", "0.00", "0.00", "0.00", "0:00PM")?;

        Ok(())
    }
    pub fn draw_text<'a>(
        &mut self,
        style: MonoTextStyle<'a, BinaryColor>,
        num1: &'a str,
        num2: &'a str,
        num3: &'a str,
        num4: &'a str,
        update: &'a str,
    ) -> anyhow::Result<()> {
        Text::new(num1, Point::new(65, 23), style).draw(self)?;
        Text::new(num2, Point::new(65, 107), style).draw(self)?;
        Text::new(num3, Point::new(22, 65), style).draw(self)?;
        Text::new(num4, Point::new(107, 65), style).draw(self)?;
        Text::new(update, Point::new(107, 10), style).draw(self)?;

        Ok(())
    }
    pub fn clear_text(&mut self) -> anyhow::Result<()> {
        self.fill_solid(
            &Rectangle::new(
                Point::new(65, 15),
                embedded_graphics::prelude::Size::new(25, 10),
            ),
            BinaryColor::Off,
        )?;
        self.fill_solid(
            &Rectangle::new(
                Point::new(65, 99),
                embedded_graphics::prelude::Size::new(25, 10),
            ),
            BinaryColor::Off,
        )?;
        self.fill_solid(
            &Rectangle::new(
                Point::new(22, 57),
                embedded_graphics::prelude::Size::new(25, 10),
            ),
            BinaryColor::Off,
        )?;
        self.fill_solid(
            &Rectangle::new(
                Point::new(107, 57),
                embedded_graphics::prelude::Size::new(25, 10),
            ),
            BinaryColor::Off,
        )?;
        self.fill_solid(
            &Rectangle::new(
                Point::new(105, 1),
                embedded_graphics::prelude::Size::new(42, 18),
            ),
            BinaryColor::Off,
        )?;
        return Ok(());
    }
    pub fn display_error_message<'a>(
        &mut self,
        message: &str,
        style: MonoTextStyle<'a, BinaryColor>,
    ) -> anyhow::Result<()> {
        Text::new(message, Point::new(58, 100), style).draw(self)?;
        Ok(())
    }
}
