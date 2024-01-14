mod common;

use rand::distributions::{Alphanumeric, DistString};

#[test]
fn test_decode_random_string() {
    let word = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
    let encoded_word = format!("{}:{}", word.len(), word);
    let expected_word = format!("\"{}\"\n", word);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_word]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_word.as_bytes());
}

#[test]
fn test_decode_tracker_url_string() {
    let word = "http://bittorrent-test-tracker.codecrafters.io/announce";
    let encoded_word = format!("{}:{}", word.len(), word);
    let expected_word = format!("\"{}\"\n", word);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_word]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_word.as_bytes());
}
