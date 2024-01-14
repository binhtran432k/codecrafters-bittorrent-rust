use std::{path::PathBuf, process::Command};

pub fn get_exec_output(args: Vec<&str>) -> std::process::Output {
    let bin_path = PathBuf::from(env!("CARGO_BIN_EXE_bittorrent-rust"));
    Command::new(bin_path)
        .args(args)
        .output()
        .expect("Failed to execute a binnary")
}
