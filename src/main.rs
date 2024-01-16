pub mod client;
pub mod display;
pub mod wifi;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::{Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use epd_waveshare::graphics::VarDisplay;
use epd_waveshare::prelude::WaveshareDisplay;
use epd_waveshare::prelude::*;
use esp_idf_hal::delay;

use anyhow::{Ok, Result};
use esp_idf_hal::peripherals::Peripherals;

use crate::client::{create_request_client, send_request};
use crate::display::init_display;
use crate::wifi::connect_to_wifi;
fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let _wifi = connect_to_wifi(peripherals.modem)?;
    let client = create_request_client()?;
    let response = send_request(client)?;
    log::info!("response: {}", response);

    let (mut display, mut epd, mut driver) = init_display(
        peripherals.spi2,
        peripherals.pins.gpio48,
        peripherals.pins.gpio38,
        peripherals.pins.gpio21,
        peripherals.pins.gpio10,
        peripherals.pins.gpio18,
        peripherals.pins.gpio17,
    )?;
    display.set_rotation(DisplayRotation::Rotate90);
    display.clear(BinaryColor::Off)?;

    let _ = Text::with_text_style(
        "Hello Rust!",
        Point::new(90, 10),
        MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
            .text_color(BinaryColor::On)
            .build(),
        TextStyleBuilder::new()
            .baseline(embedded_graphics::text::Baseline::Top)
            .build(),
    )
    .draw(&mut display);

    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    Ok(())
}
