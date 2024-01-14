use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::{iter::Peekable, str::Chars};

// Available if you need it!
// use serde_bencode
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bencode {
    String(String),
    Integer(i64),
    List(Vec<Bencode>),
}

fn decode_string(chars: &mut Peekable<Chars>) -> Result<Bencode> {
    // Example: "5:hello" -> "hello"
    let mut number_token = String::from(chars.next().unwrap());
    loop {
        // loop until catch ':'
        match chars.next() {
            Some(':') => break,
            Some(c) => number_token.push(c),
            None => return Err(anyhow!("invalid string: ':' is not found")),
        }
    }
    let number = match number_token.parse::<i64>() {
        Ok(it) => it,
        Err(_) => return Err(anyhow!("invalid string: cannot parse length")),
    };
    let mut txt = String::new();
    for _ in 0..number {
        match chars.next() {
            Some(c) => txt.push(c),
            None => return Err(anyhow!("invalid string: string size is not enough")),
        }
    }
    Ok(Bencode::String(txt))
}

fn decode_integer(chars: &mut Peekable<Chars>) -> Result<Bencode> {
    // Example: "i52e" -> 52
    chars.next(); // skip 'i'
    let mut number_string = String::new();
    loop {
        // loop until catch 'e'
        match chars.next() {
            Some('e') => break,
            Some(c) => number_string.push(c),
            None => return Err(anyhow!("invalid integer: 'e' is not found")),
        }
    }
    let number = match number_string.parse::<i64>() {
        Ok(it) => it,
        Err(_) => return Err(anyhow!("invalid integer: cannot parse number")),
    };
    Ok(Bencode::Integer(number))
}

pub fn decode(chars: &mut Peekable<Chars>) -> Result<Bencode> {
    let err_msg = format!(
        "Unhandled encoded value: {}",
        chars.clone().collect::<String>()
    );
    match chars.peek() {
        Some(x) if x.is_ascii_digit() => decode_string(chars),
        Some('i') => decode_integer(chars),
        Some('l') => {
            // Example: "l5:helloi52ee" -> ["hello",52]
            chars.next();
            let mut rt: Vec<Bencode> = Vec::new();
            loop {
                if let Some('e') = chars.peek() {
                    chars.next();
                    break;
                }
                match decode(chars) {
                    Ok(x) => rt.push(x),
                    Err(_) => return Err(anyhow!(err_msg)),
                }
            }
            Ok(Bencode::List(rt))
        }
        _ => Err(anyhow!(
            "Unhandled encoded value: {}",
            chars.collect::<String>()
        )),
    }
}
