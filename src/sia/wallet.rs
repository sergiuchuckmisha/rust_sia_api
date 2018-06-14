//*module contains functions to interact with sia wallet*/

use super::super::call_win_cmd_command::call_cmd_comand;

pub fn get_addresses() -> Vec<u8> {
    call_cmd_comand(&["/C", "curl -A Sia-Agent localhost:9980/wallet/addresses"])
}

pub fn get_addresses_2() -> Vec<u8> {
    call_cmd_comand(&["/C", &(super::SIA_PATH.to_owned() + "siac wallet addresses")])
}