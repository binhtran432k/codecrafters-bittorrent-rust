mod common;

use rand::Rng;

#[test]
fn test_decode_random_integer() {
    let number = rand::thread_rng().gen::<i32>().to_string();
    let encoded_number = format!("i{}e", number);
    let expected_number = format!("{}\n", number);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_number]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_number.as_bytes());
}

#[test]
fn test_decode_large_integer() {
    let number = "4294967300";
    let encoded_number = format!("i{}e", number);
    let expected_number = format!("{}\n", number);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_number]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_number.as_bytes());
}

#[test]
fn test_decode_negative_integer() {
    let number = "-52";
    let encoded_number = format!("i{}e", number);
    let expected_number = format!("{}\n", number);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_number]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_number.as_bytes());
}
