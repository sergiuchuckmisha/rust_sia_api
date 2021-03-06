//#![feature(proc_macro)]

//#[macro_use] extern crate stdweb;

extern crate sha1;

mod sia;
mod call_win_cmd_command;
mod servant;

use call_win_cmd_command::call_cmd_comand;
use servant::run_server;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("Hello, world!");

    let mut m = sha1::Sha1::new();
    m.update(b"Hello World!");
    println!("{}", m.digest().to_string());

    let hello2 = call_cmd_comand(vec!("/C", "echo hello"));

    println!("The value of output.stdout is: {:?}", String::from_utf8(hello2 ));
    println!("The value of output.stdout is: {:?}", String::from_utf8(call_cmd_comand(&["/C", "echo hello"]) ));
    println!("The value of output.stdout is: {:?}", String::from_utf8(call_cmd_comand(vec!("/C", "echo hello")) ));
    println!("The value of output.stdout is: {:?}", String::from_utf8(call_cmd_comand(&["echo hello"]) ));
    println!("The value of output.stdout is: {:?}", String::from_utf8(call_cmd_comand(&["/C", "echo hello"]) ).unwrap().trim());
    println!("The value of output.stdout is: {:?}", String::from_utf8(call_cmd_comand(&["/C", "curl -A Sia-Agent localhost:9980/wallet/addresses"]) ).unwrap().trim());
    run_server();
}


