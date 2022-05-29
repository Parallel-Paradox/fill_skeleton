mod parser;

use std::fs::read_to_string;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub splitter);

fn main() {
    // read the input file
    let input = read_to_string("./sample_input/hello.cpp").unwrap();
    let split = splitter::ProgramParser::new().parse(&input).unwrap();

    println!("{:#?}", split);
    println!("Hello, world!");
}
