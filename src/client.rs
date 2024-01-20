use anyhow::anyhow;
use embedded_websocket::framer::Framer;
use embedded_websocket::framer::ReadResult;
use embedded_websocket::WebSocketClient;
use embedded_websocket::WebSocketOptions;
use esp_idf_svc::http::client::EspHttpConnection;
use rand::rngs::ThreadRng;
use std::net::TcpStream;

use crate::types;

// just for testing
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

//where the real code begins
pub fn create_ws_client<'a>() -> anyhow::Result<()> {
    let mut read_cursor = 0;
    // we dont need this after the intial request was send since this is a write only websocket
    let mut write_buf = [0; 1000];
    let mut read_buf = [0; 1000];

    let mut frame_buf = [0; 1000];

    let (mut stream, options, mut client) = create_tcp_conn_and_client("192.168.0.50:4000")?;
    let mut framer = Framer::new(&mut read_buf, &mut read_cursor, &mut write_buf, &mut client);
    match framer.connect(&mut stream, &options) {
        Ok(_) => (),
        Err(e) => return Err(convert_connect_error(e)),
    }
    log::info!("Connected to websocket");

    while let Some(ReadResult::Text(s)) = framer.read(&mut stream, &mut frame_buf).ok() {
        if let Ok(json_values) = serde_json_core::from_str::<types::UiData>(s) {
            log::info!("Got Message: {:?}", json_values);
        } else {
            log::info!("Error deserializing data!");
        }
    }

    return Ok(());
}
pub fn convert_connect_error(
    err: embedded_websocket::framer::FramerError<std::io::Error>,
) -> anyhow::Error {
    let err_anyhow = match err {
        embedded_websocket::framer::FramerError::Io(e) => anyhow!("{:?}", e),
        embedded_websocket::framer::FramerError::Utf8(e) => anyhow::Error::from(e),
        embedded_websocket::framer::FramerError::WebSocket(e) => anyhow!("{:?}", e),
        embedded_websocket::framer::FramerError::FrameTooLarge(n) => {
            anyhow::Error::msg(format!("Frame to large: {}", n))
        }
        embedded_websocket::framer::FramerError::HttpHeader(e) => anyhow!("{:?}", e),
    };
    return err_anyhow;
}
pub fn create_tcp_conn_and_client(
    addr: &str,
) -> anyhow::Result<(TcpStream, WebSocketOptions, WebSocketClient<ThreadRng>)> {
    let stream = TcpStream::connect(addr)?;
    let client = WebSocketClient::new_client(rand::thread_rng());
    let websocket_options = WebSocketOptions {
        path: "/subscribe",
        host: "",
        origin: "",
        sub_protocols: None,
        additional_headers: None,
    };
    return Ok((stream, websocket_options, client));
}
