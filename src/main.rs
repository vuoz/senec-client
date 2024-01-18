pub mod client;
pub mod display;
pub mod types;
pub mod wifi;

use client::{convert_connect_error, create_tcp_conn_and_client, create_ws_client};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::{Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_websocket::framer::{Framer, ReadResult};
use epd_waveshare::prelude::WaveshareDisplay;
use epd_waveshare::prelude::*;
use esp_idf_hal::delay;

use anyhow::Result;
use esp_idf_hal::peripherals::Peripherals;

use crate::display::init_display;
use crate::wifi::connect_to_wifi;
fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    // connecting to wifi
    let _wifi = connect_to_wifi(peripherals.modem)?;

    // setting up display
    let (mut display, mut epd, mut driver) = init_display(
        peripherals.spi2,
        peripherals.pins.gpio48,
        peripherals.pins.gpio38,
        peripherals.pins.gpio21,
        peripherals.pins.gpio10,
        peripherals.pins.gpio18,
        peripherals.pins.gpio17,
    )?;
    let default_text_style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
        .text_color(BinaryColor::On)
        .build();
    let text_style_baseline = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .build();
    let mut read_cursor = 0;
    // we dont need this after the intial request was send since this is a write only websocket
    let mut write_buf = [0; 1000];
    let mut read_buf = [0; 1000];

    let mut frame_buf = [0; 1000];

    let (mut stream, options, mut client) = create_tcp_conn_and_client("192.168.0.148:4000")?;
    log::info!("tcp conn success");
    let mut framer = Framer::new(&mut read_buf, &mut read_cursor, &mut write_buf, &mut client);

    match framer.connect(&mut stream, &options) {
        Ok(_) => (),
        Err(e) => return Err(convert_connect_error(e)),
    };
    log::info!("Connected to websocket");
    display.clear(BinaryColor::Off)?;
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    while let Some(ReadResult::Text(s)) = framer.read(&mut stream, &mut frame_buf).ok() {
        if let Ok((json_values, _)) = serde_json_core::from_str::<types::UiData>(s) {
            display.clear(BinaryColor::Off)?;
            log::info!("Got message: {:?}", json_values);
            let _ = Text::with_text_style(
                &format!("CurrCharge: {:?} ", json_values.gui_bat_data_fuel_charge),
                Point::new(40, 10),
                default_text_style,
                text_style_baseline,
            )
            .draw(&mut display)?;
            let _ = Text::with_text_style(
                &format!("CurrGridPow: {:?}", json_values.gui_grid_pow),
                Point::new(40, 30),
                default_text_style,
                text_style_baseline,
            )
            .draw(&mut display)?;
            epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
            //epd.update_partial_new_frame(spi, buffer, x, y, width, height)
        } else {
            display.clear(BinaryColor::Off)?;
            let _ = Text::with_text_style(
                "Error occured retrieving data from server!",
                Point::new(50, 10),
                default_text_style,
                text_style_baseline,
            )
            .draw(&mut display)?;
            Text::with_text_style(
                "Please check the error-logs!",
                Point::new(70, 30),
                default_text_style,
                text_style_baseline,
            )
            .draw(&mut display)?;
            epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
        }
    }

    Ok(())
}
