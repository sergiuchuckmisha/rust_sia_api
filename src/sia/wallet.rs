//*module contains functions to interact with sia wallet*/

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

use super::super::call_win_cmd_command::call_cmd_comand;

pub fn get_addresses() -> Vec<u8> {
    call_cmd_comand(&["/C", "curl -A Sia-Agent localhost:9980/wallet/addresses"])
}

pub fn get_addresses_2() -> Vec<u8> {
    call_cmd_comand(&["/C", &(super::SIA_PATH.to_owned() + "siac wallet addresses")])
}

/**
@return first address
todo what if no addresses is found?
idea is to get json and parse it
code is taken from
https://docs.rs/rustc-serialize/0.3.24/rustc_serialize/json/index.html
https://coderwall.com/p/xcs8xa/convert-a-slice-or-an-array-to-a-vec-in-rust
*/
pub fn get_first_address()
    -> Vec<u8>
{
    let json = Json::from_str(&String::from_utf8(get_addresses()).unwrap()).unwrap();

    let obj = json.as_object().unwrap();
    let foo = obj.get("addresses").unwrap();

    foo.as_array().unwrap()[0].as_string().unwrap().as_bytes().to_vec()
}