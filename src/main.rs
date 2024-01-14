use std::{env, iter::Peekable, str::Chars};

use serde::{Deserialize, Serialize};

// Available if you need it!
// use serde_bencode
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Bencode {
    String(String),
    Integer(i64),
    List(Vec<Bencode>),
}

fn decode_bencoded_value(chars: &mut Peekable<Chars>) -> Result<Bencode, String> {
    let err_msg = format!("Unhandled encoded value: {}", chars.clone().collect::<String>());
    match chars.next() {
        Some(x) if x.is_ascii_digit() => {
            // Example: "5:hello" -> "hello"
            let mut number_token = x.to_string();
            let mut curr = chars.next();
            loop {
                match curr {
                    Some(':') => break,
                    Some(c) => number_token.push(c),
                    None => return Err(err_msg),
                }
                curr = chars.next();
            }
            let number = match number_token.parse::<i64>() {
                Ok(it) => it,
                Err(_) => return Err(err_msg),
            };
            let mut txt = String::new();
            for _ in 0..number {
                match chars.next() {
                    Some(c) => txt.push(c),
                    None => return Err(err_msg),
                }
            }
            Ok(Bencode::String(txt))
        }
        Some('i') => {
            // Example: "i52e" -> 52
            let mut number_string = String::new();
            let mut curr = chars.next();
            loop {
                match curr {
                    Some('e') => break,
                    Some(c) => number_string.push(c),
                    None => return Err(err_msg),
                }
                curr = chars.next();
            }
            let number = match number_string.parse::<i64>() {
                Ok(it) => it,
                Err(_) => return Err(err_msg),
            };
            Ok(Bencode::Integer(number))
        }
        Some('l') => {
            // Example: "l5:helloi52ee" -> ["hello",52]
            let mut rt: Vec<Bencode> = Vec::new();
            loop {
                if let Some('e') = chars.peek() {
                    chars.next();
                    break;
                }
                match decode_bencoded_value(chars) {
                    Ok(x) => rt.push(x),
                    Err(_) => return Err(err_msg),
                }
            }
            Ok(Bencode::List(rt))
        }
        _ => Err(err_msg),
    }
}

// Usage: bittorrent-rust decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        match decode_bencoded_value(&mut encoded_value.chars().peekable()) {
            Ok(decoded_value) => {
                println!("{}", serde_json::to_value(decoded_value).unwrap());
            }
            Err(err) => panic!("{}", err),
        };
    } else {
        println!("unknown command: {}", args[1])
    }
}
