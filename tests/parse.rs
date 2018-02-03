
extern crate minidom;
extern crate atdf;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse(path: &str) {
    let file = File::open(path).unwrap();
    let mut file = BufReader::new(file);
    let mut atdf = String::new();
    file.read_to_string(&mut atdf).unwrap();
    let root: minidom::Element = atdf.parse().unwrap();

    let devices = atdf::parse(&root).unwrap();
    assert!(!devices.is_empty());
}

#[allow(non_snake_case)]
mod suite {
    use super::parse;

    include!("data/suite.rs");
}
