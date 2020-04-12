use std::io::stdin;

pub struct CommandLine {}

impl CommandLine {
    pub fn get_input(&self) -> Option<String> {
        let mut input = String::new();

        stdin().read_line(&mut input).ok().map(|_| input)
    }

    pub fn print_output(&self, output: String) {
        println!("{}", output);
    }
}
