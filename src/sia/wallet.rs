
pub fn get_addresses() -> Vec<u8> {
    call_cmd_comand(&["/C", "curl -A Sia-Agent localhost:9980/wallet/addresses"])
}