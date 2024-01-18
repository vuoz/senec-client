use anyhow::anyhow;
use embedded_websocket::framer::Framer;
use embedded_websocket::framer::ReadResult;
use embedded_websocket::WebSocketOptions;
use embedded_websocket::WebSocketSendMessageType;
use std::any;
use std::net::TcpStream;

use embedded_websocket::{Client, WebSocketClient};
use esp_idf_svc::http::client::EspHttpConnection;

pub fn create_request_client(
) -> anyhow::Result<embedded_svc::http::client::Client<EspHttpConnection>> {
    let conn = esp_idf_svc::http::client::EspHttpConnection::new(&Default::default())?;
    let client = embedded_svc::http::client::Client::wrap(conn);
    return Ok(client);
}
pub fn send_request<'a>(
    mut client: embedded_svc::http::client::Client<EspHttpConnection>,
) -> anyhow::Result<String> {
    let headers = [("accept", "text/plain")];
    let req = client.request(
        embedded_svc::http::Method::Get,
        "http://192.168.0.133:4000/",
        &headers,
    )?;
    let mut response = req.submit()?;
    if response.status() != 200 {
        return Err(anyhow::Error::msg("Status error"));
    }
    let mut vec = Vec::with_capacity(100);
    // trying to avoid stack overflow
    // terribly inefficient :()
    loop {
        let mut buf = [0u8; 1];
        let n = match response.read(&mut buf) {
            Ok(n) => n,
            Err(e) => return Err(anyhow::Error::from(e)),
        };
        if n == 0 {
            break;
        }
        let val = buf[0];
        vec.push(val);
    }

    return Ok(String::from_utf8(vec)?);
}
pub fn create_ws_client() -> anyhow::Result<()> {
    let mut read_buf = [0; 4000];
    let mut read_cursor = 0;
    let mut write_buf = [0; 4000];
    let mut frame_buf = [0; 4000];

    let mut stream = TcpStream::connect("192.168.0.148:4000")?;
    let mut client = WebSocketClient::new_client(rand::thread_rng());
    let websocket_options = WebSocketOptions {
        path: "/subscribe",
        host: "",
        origin: "",
        sub_protocols: None,
        additional_headers: None,
    };
    let mut framer = Framer::new(&mut read_buf, &mut read_cursor, &mut write_buf, &mut client);
    match framer.connect(&mut stream, &websocket_options) {
        Ok(_) => (),
        Err(err) => {
            let err = match err {
                embedded_websocket::framer::FramerError::Io(e) => anyhow::Error::from(e),
                embedded_websocket::framer::FramerError::Utf8(e) => anyhow::Error::from(e),
                embedded_websocket::framer::FramerError::WebSocket(e) => anyhow!("{:?}", e),
                embedded_websocket::framer::FramerError::FrameTooLarge(_) => {
                    anyhow::Error::msg("Frame to large")
                }
                embedded_websocket::framer::FramerError::HttpHeader(e) => anyhow!("{:?}", e),
            };
            return Err(err);
        }
    }
    log::info!("Connected to websocket");
    while let Some(ReadResult::Text(s)) = framer.read(&mut stream, &mut frame_buf).ok() {
        log::info!("Got Message: {:?}", s);
    }

    Ok(())
}
