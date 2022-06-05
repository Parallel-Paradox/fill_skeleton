mod parser;
mod util;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::env::args;
use lalrpop_util::lalrpop_mod;
use crate::util::*;
lalrpop_mod!(pub splitter);

fn main() -> Result<(), String> {
    // read the input file
    let mut args = args();
    args.next();    // ignore the first arg

    let mut arg_map: HashMap<&str, String> = HashMap::new();

    loop {
        // check the Nil option
        let arg_box = args.next();
        let arg: String;
        if arg_box != None { arg = arg_box.unwrap(); } else { break; }
        match &arg as &str {
            // -i: Set input path, required.
            "-i" => { arg_map.insert("-i", args.next().unwrap()); },
            // -o: Set the output path, print in console if unset.
            "-o" => { arg_map.insert("-o", args.next().unwrap()); },
            // -rs: reserve according to reserve.json
            // -rm: remove according to remove.json
            // json path is set in "list"
            "-rs" | "-rm" => {
                arg_map.insert("mode", arg);
                arg_map.insert("list", args.next().unwrap());
            },
            // -rp: replace according to replace.json
            "-rp" => { arg_map.insert("-rp", args.next().unwrap()); },
            _ => { return Err(format!("Can't resolve arg [ {} ]", arg)); }, // Unresolved parameter
        }
    }

    let input = read_to_string(&arg_map["-i"]).unwrap();
    let mut split = splitter::ProgramParser::new().parse(&input).unwrap();

    dump_section(&arg_map, &mut split)
}
