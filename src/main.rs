use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font:: MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use epd_waveshare::prelude::*;
use epd_waveshare::{prelude::WaveshareDisplay, *};
use embedded_graphics::text::{Text, TextStyleBuilder};
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::spi;
use embedded_graphics::Drawable;

use anyhow::Result;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;
fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();


    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let mut driver = spi::SpiDeviceDriver::new_single(
        peripherals.spi2,
        peripherals.pins.gpio48,
        peripherals.pins.gpio38,
        Option::<gpio::AnyIOPin>::None,
        Option::<gpio::AnyOutputPin>::None,
        &spi::SpiDriverConfig::new().dma(spi::Dma::Disabled),
        &spi::SpiConfig::new().baudrate(Hertz::from(26)),
    )?;

    let cs = gpio::PinDriver::output(peripherals.pins.gpio21)?;

    let busy = gpio::PinDriver::input(peripherals.pins.gpio10)?;

    let dc = gpio::PinDriver::output(peripherals.pins.gpio18)?;

    let rst = gpio::PinDriver::output(peripherals.pins.gpio17)?;

    let mut epd = match epd2in9_v2::Epd2in9::new(&mut driver, cs, busy, dc, rst,&mut delay::Ets ) {
        Ok(epd) => epd,
        Err(e) => {
            log::error!("Error edp: {:?}", e.to_string());

            return Ok(());
        }
    };

 
    let mut display = epd2in9_v2::Display2in9::default();
    display.set_rotation(DisplayRotation::Rotate90);


    display.clear(BinaryColor::Off)?;
    let _ = Text::with_text_style("Hello Rust!!!!", Point::new(90,10), MonoTextStyleBuilder::new().font(&embedded_graphics::mono_font::ascii::FONT_10X20).text_color(BinaryColor::On).build(), TextStyleBuilder::new().baseline(embedded_graphics::text::Baseline::Top).build()).draw(&mut display);
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    Ok(())
}
