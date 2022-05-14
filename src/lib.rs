use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

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

    // FIX ME
    let ls = Command::new("ls").output().expect("Failed.");
    let status = Command::new("/usr/bin/cc -o /Users/reez12g/development/rcc/tmp /Users/reez12g/development/rcc/tmp.s")
                                                .status();
    match status {
        Ok(v) => println!("status {}", v),
        Err(e) => println!("error: {}", e)
    }
    println!("value: {}", std::str::from_utf8(&ls.stdout).unwrap())
}

#[cfg(test)]
mod test {
    // FIX ME
    use std::process::Command;
    use crate::compile;

    #[test]
    fn can_compile() {
        let args: Vec<String> = vec!["test.rc", "123"]
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect();
        compile(&args);
        Command::new("cc")
                .arg("-o")
                .arg("tmp tmp.s");
        Command::new("./tmp");

        let output = {
            Command::new("echo")
                    .arg("$?")
                    .output()
        };
        match output {
            Ok(v) => println!("value = {}", v.stdout[0]),
            Err(e) => println!("err value = {}", e)
        };
    }
}