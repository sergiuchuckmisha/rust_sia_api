

use std::process::Command;
use std::ffi::OsStr;


pub fn call_cmd_comand<I, S>(args: I) -> Vec<u8>
    where I: IntoIterator<Item=S>, S: AsRef<OsStr>
{
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(args)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello from sh")
            .output()
            .expect("failed to execute process")
    };

    output.stdout
}