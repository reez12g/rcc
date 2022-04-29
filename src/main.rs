use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: &usize = &args.len();
    const NUM_OF_ARGC: usize = 2;
    if argc != &NUM_OF_ARGC {
        print!("Incorrect number of arguments.\n");
    }

    print!(".globl _main\n");
    print!("_main:\n");
    print!("  mov x0, {}\n", &args[1]);
    print!("  ret\n");
}
