
extern crate atdf;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse(path: &str) {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let atdf = atdf::parse(file).unwrap();

    let device = atdf.devices().first().unwrap();
    let d = atdf::device(device.clone(), atdf.modules());
    panic!("{:#?}", d);
    // println!("{}", device.family);
}

#[allow(non_snake_case)]
mod suite {
    use super::parse;

    include!("data/suite.rs");
}
