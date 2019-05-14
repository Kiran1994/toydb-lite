use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

pub fn write(page_number: &String, data: &String) {
    let file_name = format!("./table/page-{}", page_number.to_string());

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    writeln!(file, "{}", data);
}

pub fn read(page_number: &String) -> String {
    let file_name = format!("./table/page-{}", page_number.to_string());
    //TODO: handle file read errors properly before unwrap
    return fs::read_to_string(file_name).unwrap();
}