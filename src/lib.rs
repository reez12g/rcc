use std::fs::File;
use std::io::{BufWriter, Write};

pub fn compile(args: &[String]) {
    let argc: &usize = &args.len();
    const NUM_OF_ARGC: usize = 2;
    if argc != &NUM_OF_ARGC {
        print!("Incorrect number of arguments.\n");
    }
    let mut assembly_file_writer = BufWriter::new(
                                                File::create("tmp.s")
                                                        .expect("Error encountered while creating file!"));

    let contents = ".globl _main\n_main:\n  mov x0, {}\n  ret\n".replace("{}", &args[1]);
    assembly_file_writer.write_all(
        &contents.as_bytes()
    ).unwrap();
}

#[cfg(test)]
mod test {
    // FIX ME
    use crate::compile;

    #[test]
    fn can_compile() {
        let args: Vec<String> = vec!["test.rc", "123"]
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect();
        compile(&args);
    }
}