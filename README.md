# rust_sia_api
basic api for sia. Assuming that platform is win and that sia instance is started on the machine.

## Prerequisites:
1. [nightly] rust version activated
2. [sia] is installed and running on 9980 port: 
 
 
## Running server

 * just navigate to `rust_sia_api` package and run `cargo run` and check for [3000 port on localhost]
 
 
 ## Implemented functionality
 
 1. [/addresses] and [/address] GET methods to check current address(es)
 
 
 [sia]: https://github.com/NebulousLabs/Sia
 [nightly]: https://doc.rust-lang.org/1.15.0/book/nightly-rust.html
 [3000 port on localhost]: http://127.0.0.1:3000/
 [/addresses]: http://127.0.0.1:3000/addresses
 [/address]: http://127.0.0.1:3000/address