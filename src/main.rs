pub mod client;
pub mod display;
pub mod types;
pub mod wifi;

use std::time::Duration;

use client::{convert_connect_error, create_tcp_conn_and_client};
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

    // get peripherals
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
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let _text_style_baseline = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .build();
    let mut read_cursor = 0;
    // we dont need this after the intial request was send since this is a write only websocket
    let mut write_buf = [0; 1000];
    let mut read_buf = [0; 1000];

    let mut frame_buf = [0; 10000];

    log::info!("Starting tcp conn");
    let (mut stream, options, mut client) = create_tcp_conn_and_client("192.168.0.131:4000")?;
    log::info!("tcp conn success");
    let mut framer = Framer::new(&mut read_buf, &mut read_cursor, &mut write_buf, &mut client);

    match framer.connect(&mut stream, &options) {
        Ok(_) => (),
        Err(e) => return Err(convert_connect_error(e)),
    };

    log::info!("Connected to websocket");

    // Clear the display from any remainders
    epd.clear_frame(&mut driver, &mut delay::Ets)?;
    display.clear(BinaryColor::Off)?;

    // draw the ui with default values
    display.draw_default_display(default_text_style)?;
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
    epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;

    // start time
    let mut curr_time = std::time::SystemTime::now();

    loop {
        match framer.read(&mut stream, &mut frame_buf) {
            Ok(read_res) => match read_res {
                ReadResult::Binary(_) => continue,
                ReadResult::Pong(_) => continue,
                ReadResult::Closed => continue,
                ReadResult::Text(t) => {
                    let time_now = std::time::SystemTime::now();
                    let since = time_now.duration_since(curr_time)?;
                    if since > Duration::from_secs(60) {
                        display.clear(BinaryColor::Off)?;
                        display.draw_default_display(default_text_style)?;
                        epd.update_and_display_frame(
                            &mut driver,
                            display.buffer(),
                            &mut delay::Ets,
                        )?;
                        epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
                        curr_time = time_now;
                    }
                    match serde_json_core::from_str::<types::UiDataWithWeather>(t) {
                        Ok((json_values, _)) => {
                            log::info!("Got message: {:?}", json_values);
                            display.clear_text()?;
                            display.draw_text(
                                default_text_style,
                                json_values.gui_house_pow,
                                json_values.gui_bat_data_fuel_charge,
                                json_values.gui_inverter_power,
                                json_values.gui_grid_pow,
                                json_values.ts,
                            )?;

                            epd.update_new_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
                            epd.display_new_frame(&mut driver, &mut delay::Ets)?;
                            epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;

                            continue;
                        }
                        Err(e) => {
                            log::info!("An error occured: {:?} Message: {:?}  ", e, t);
                            display.clear(BinaryColor::Off)?;
                            display.display_error_message(
                                "Error decoding message!",
                                default_text_style,
                            )?;
                            epd.update_and_display_frame(
                                &mut driver,
                                display.buffer(),
                                &mut delay::Ets,
                            )?;
                            epd.update_old_frame(&mut driver, display.buffer(), &mut delay::Ets)?;
                            continue;
                        }
                    }
                }
            },
            Err(e) => {
                println!("Error :{:?}", e);
                break;
            }
        }
    }
    display.clear_buffer(Color::White);
    Text::new(
        "Disconnected from Websocket!",
        Point::new(60, 40),
        default_text_style,
    )
    .draw(&mut display)?;
    epd.update_and_display_frame(&mut driver, display.buffer(), &mut delay::Ets)?;

    Ok(())
}
