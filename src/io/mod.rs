use std::io::{stdin, stdout, Write};

pub struct CommandLine {}

impl CommandLine {
    pub fn get_input(&self) -> Option<String> {
        let mut input = String::new();

        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).ok().map(|_| input)
    }

    pub fn print_output(&self, output: String) {
        println!("{}", output);
    }
}
