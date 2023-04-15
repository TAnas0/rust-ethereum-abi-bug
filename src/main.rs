use std::fs::File;
use ethereum_abi::Abi;

fn main() {
    // Parse ABI JSON file
    let abi = {
        let file = File::open("./ABI.json").expect("failed to open ABI file");
        Abi::from_reader(file).expect("failed to parse ABI")
    };
}