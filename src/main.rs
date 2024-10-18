mod instruction;
mod error;
mod parser;

use std::io::Write; // bring trait into scope
use std::fs;

use parser::parse;

fn main() {
    let contents = fs::read_to_string("test.aasm")
        .expect("Should have been able to read the file");

    let parsed = parse(contents);
    if let Ok(val) = parsed {
        let instr_data = instruction::convert_instr(val);

        let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        // either use the ? operator or unwrap since it returns a Result
        .open("bin/value.ao").unwrap();

        let _ = file.write_all(&instr_data);
    } else if let Err(val) = parsed {
        println!("{:?}", val);
    }
}
