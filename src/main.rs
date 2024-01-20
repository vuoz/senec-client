pub mod client;
pub mod display;
pub mod types;
pub mod wifi;

use embedded_graphics::prelude::Size;
use std::time::Duration;

use client::{convert_connect_error, create_tcp_conn_and_client, create_ws_client};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::primitives::{Line, PrimitiveStyleBuilder};
use embedded_graphics::primitives::{Primitive, Rectangle};
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
    log::info!("Got the display");
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

    log::info!("Starting tcp conn");
    let (mut stream, options, mut client) = create_tcp_conn_and_client("192.168.0.50:4000")?;
    log::info!("tcp conn success");
    let mut framer = Framer::new(&mut read_buf, &mut read_cursor, &mut write_buf, &mut client);

    match framer.connect(&mut stream, &options) {
        Ok(_) => (),
        Err(e) => return Err(convert_connect_error(e)),
    };

    let mut curr_time = std::time::SystemTime::now();

    log::info!("Connected to websocket");
    epd.clear_frame(&mut driver, &mut delay::Ets)?;

    display.clear(BinaryColor::Off)?;
    Line::new(Point::new(0, 40), Point::new(296, 40))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(10)
                .stroke_color(BinaryColor::On)
                .build(),
        )
        .draw(&mut display)?;
    // this should later be the ui and the icons etc
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    while let Some(ReadResult::Text(s)) = framer.read(&mut stream, &mut frame_buf).ok() {
        // every 2 mins the screen will be refreshed fully
        // to remove any remainders

        let time_now = std::time::SystemTime::now();
        let since = time_now.duration_since(curr_time)?;
        if since > Duration::from_secs(60) {
            curr_time = time_now;
            display.clear_buffer(Color::White);
            Line::new(Point::new(0, 40), Point::new(296, 40))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_width(10)
                        .stroke_color(BinaryColor::On)
                        .build(),
                )
                .draw(&mut display)?;

            epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
            epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
        }

        match serde_json_core::from_str::<types::UiData>(s) {
            Ok((json_values, _)) => {
                log::info!("Got message: {:?}", json_values);
                // remove the text from before from the buffer
                display.fill_solid(
                    &Rectangle::new(Point::new(45, 10), Size::new(110, 25)),
                    BinaryColor::Off,
                )?;

                Text::new(
                    json_values.gui_grid_pow,
                    Point::new(45, 30),
                    default_text_style,
                )
                .draw(&mut display)?;
                epd.update_new_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
                epd.display_new_frame(&mut driver, &mut delay::Ets)?;

                continue;
            }
            Err(e) => {
                log::info!("An error occured: {:?} Message: {:?}  ", e, s);
            }
        }
    }
    display.clear_buffer(Color::White);
    Text::new(
        "Disconnected from Websocket!",
        Point::new(5, 40),
        default_text_style,
    )
    .draw(&mut display)?;
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;

    Ok(())
}
