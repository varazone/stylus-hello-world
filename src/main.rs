#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    // stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
    stylus_sdk::abi::export::print_abi::<stylus_hello_world::Counter>(
        "MIT-OR-APACHE-2.0",
        "pragma solidity ^0.8.23;",
    );
}
