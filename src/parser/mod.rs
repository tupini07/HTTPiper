pub mod entities;
mod process_rules;
use pest::error::Error;
use pest::Parser;
use std::fs;

use crate::parser::entities as e;
use crate::parser::process_rules as pr;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;
pub type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct HttppParser;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn parse_file<'b>(file_name: &str) -> Result<(), Error<Rule>> {
    let unparsed_file = fs::read_to_string(file_name).expect("cannot read file");
    let mut program: Pair = HttppParser::parse(Rule::program, &unparsed_file)?
        .next()
        .unwrap();
    // println!(
    //     "{:?}",
    //     program.into_inner().map(|p: Pair| match p.as_rule() {
    //         Rule::import => parse_import(p),
    //         _ => ProgramStatement::ImportFileName("-".to_owned())
    //     })
    // );

    for x in program.into_inner() {
        match x.as_rule() {
            Rule::import => (), // println!("{:?}", pr::parse_import(x)),
            Rule::var_assignment => println!("{:?}", pr::parse_variable_assignment(x)),
            _ => println!("----"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn can_parse_test_file() {
        let unparsed_file = fs::read_to_string("src/parser/test.httpp").expect("cannot read file");

        let mut program: Pair = HttppParser::parse(Rule::program, &unparsed_file)
            .unwrap()
            .next()
            .unwrap();

        let collected = program
            .into_inner()
            .map(|p: Pair| format!("{:?}", p.as_rule()))
            .collect::<Vec<String>>();
    }
}
