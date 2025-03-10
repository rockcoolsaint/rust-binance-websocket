use tungstenite::connect;
use url::Url;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
fn main() {
    let binance_url = format!("{}/ws", BINANCE_WS_API);
    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }
}
