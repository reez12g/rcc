use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: &usize = &args.len();
    const NUM_OF_ARGC: usize = 2;
    if argc != &NUM_OF_ARGC {
        print!("Incorrect number of arguments.\n");
    }

    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");
    print!("  mov rax, {}\n", &args[1]);
    print!("  ret\n");
}
