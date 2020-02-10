extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
extern crate test_case;

use std::fs;
mod parser;

fn main() {
    parser::parse_file("src/parser/test.httpp");
}
