mod bencoded_decoder;

use std::env;

// Usage: bittorrent-rust decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        match bencoded_decoder::decode(&mut encoded_value.chars().peekable()) {
            Ok(decoded_value) => {
                println!("{}", serde_json::to_value(decoded_value).unwrap());
            }
            Err(err) => panic!("{}", err),
        };
    } else {
        println!("unknown command: {}", args[1])
    }
}
