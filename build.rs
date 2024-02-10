use dotenv::dotenv;
fn main() {
    // read the env vars from .env and set them as rustc env vars so the compiler can read them
    // and include them on compile time

    dotenv().ok();
    match std::env::var("WIFI_PASS") {
        Err(_) => panic!("Error Wifi Pass not set! Please add WIFI_PASS to .env"),
        Ok(pass) => {
            println!("cargo:rustc-env=WIFI_PASS={}", pass);
        }
    }
    match std::env::var("WIFI_SSID") {
        Err(_) => panic!("Error Wifi SSID not set! Please add WIFI_SSID to .env"),
        Ok(ssid) => {
            println!("cargo:rustc-env=WIFI_SSID={}", ssid);
        }
    }
    embuild::espidf::sysenv::output();
}
