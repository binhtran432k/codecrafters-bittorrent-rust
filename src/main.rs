use std::env;

use serde_json::Number;

// Available if you need it!
// use serde_bencode

fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let err_msg = format!("Unhandled encoded value: {}", encoded_value);
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').expect(&err_msg);
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().expect(&err_msg);
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        serde_json::Value::String(string.to_string())
    } else if encoded_value.starts_with('i') && encoded_value.ends_with('e') {
        let number_string = &encoded_value[1..encoded_value.len() - 1];
        let number = number_string.parse::<i64>().expect(&err_msg);
        serde_json::Value::Number(Number::from(number))
    } else {
        panic!("{}", err_msg)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
