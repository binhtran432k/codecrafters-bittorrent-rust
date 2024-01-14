mod common;

use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};

#[test]
fn test_decode_empty_list() {
    let encoded_list = "le";
    let expected_list = "[]\n";
    let output = common::get_exec_output(Vec::from(["decode", &encoded_list]));
    assert!(output.status.success());
    assert_eq!(&output.stdout[..], expected_list.as_bytes());
}

#[test]
fn test_decode_list_with_random_word_and_number() {
    let word = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
    let encoded_word = format!("{}:{}", word.len(), word);
    let number = rand::thread_rng().gen_range(0..1000).to_string();
    let encoded_number = format!("i{}e", number);
    let encoded_list = format!("l{}{}e", encoded_word, encoded_number);
    let expected_lists = Vec::from([
        format!("[\"{}\",{}]\n", word, number),
        format!("[\"{}\", {}]\n", word, number),
    ]);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_list]));
    assert!(output.status.success());
    assert!(expected_lists
        .iter()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>()
        .contains(&&output.stdout[..]));
}

#[test]
fn test_decode_nested_list() {
    let word = Alphanumeric.sample_string(&mut rand::thread_rng(), 20);
    let encoded_word = format!("{}:{}", word.len(), word);
    let number = rand::thread_rng().gen_range(0..1000).to_string();
    let encoded_number = format!("i{}e", number);
    let encoded_list = format!("ll{}{}ee", encoded_number, encoded_word);
    let expected_lists = Vec::from([
        format!("[[{},\"{}\"]]\n", number, word),
        format!("[[{}, \"{}\"]]\n", number, word),
    ]);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_list]));
    assert!(output.status.success());
    assert!(expected_lists
        .iter()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>()
        .contains(&&output.stdout[..]));
}

#[test]
fn test_decode_mixed_nested_list() {
    let encoded_list = "lli4eei5ee";
    let expected_lists = Vec::from(["[[4],5]\n", "[[4], 5]\n"]);
    let output = common::get_exec_output(Vec::from(["decode", &encoded_list]));
    assert!(output.status.success());
    assert!(expected_lists
        .iter()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>()
        .contains(&&output.stdout[..]));
}
