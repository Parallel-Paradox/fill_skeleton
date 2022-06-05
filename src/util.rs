use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::Write;
use serde_json::Value;
use crate::parser::*;

pub fn process_section(list_path: &str, program: &mut Vec<Section>, mode: &str)
                       -> Result<(), String> {
    let list_raw = read_to_string(list_path).unwrap();
    let list_value: Value = serde_json::from_str(&list_raw).unwrap();
    let list = match list_value {
        Value::Array(array) => {
            let mut list = Vec::new();
            for val in array {
                match val {
                    Value::String(name) => list.push(name),
                    _ => { return Err(format!("{:?} - is not a string!", val)); }
                }
            }
            list
        },
        _ => { return Err(format!("{} muse be a json array!", list_path)); },
    };

    for section in program {
        // ignore section
        mark(section, &list, mode, true);
    }

    Ok(())
}

fn mark(section: &mut Section, list: &Vec<String>, mode: &str, parent: bool) {
    // parent is false when parent section is removed.
    let contains = list.contains(&section.ident);
    // -rm: remove if in list, or parent is removed.
    // -rs: remove if not list, or parent is removed.
    if (contains && mode == "-rm")
        || (!contains && mode == "-rs") || !parent {
        section.ignore = true;
    }

    for item in &mut section.items {
        if let Item::SubSection(sub_section) = item {
            mark(sub_section, list, mode, !section.ignore);
        }
    }
}

pub fn dump_section(arg_map: &HashMap<&str, String>, program: &mut Vec<Section>)
                    -> Result<(), String> {
    let mut base = String::new();

    if arg_map.contains_key("mode") {
        process_section(&arg_map["list"], program, &arg_map["mode"]).unwrap();
    }
    get_content(&program[0], &mut base);

    println!("{}", base);

    if !arg_map.contains_key("-o") { println!("{:#?}", program); }
    else {
        let content = format!("{:#?}", program);
        let mut file = File::create(&arg_map["-o"]).unwrap();
        file.write_all(&content.as_bytes()).unwrap();
    }

    Ok(())
}

fn get_content<'a>(section: &Section, base_on: &'a mut String) -> &'a mut String {
    if section.ignore { return base_on; }
    for item in &section.items {
        match item {
            Item::Content(content) => { base_on.push_str(content); },
            Item::SubSection(sub_section) => { get_content(sub_section, base_on); },
        }
    }

    base_on
}
