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
