use std::env;
use rcc::compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    compile(&args);
}
