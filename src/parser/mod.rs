use pest::error::Error;
use pest::Parser;
use std::fs;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

pub enum ProgramStatement {
    ImportFileName(String),
    VariableAssignment {
        name: String, value: // how to substitution here?
    }
}

pub struct Program {
    imports: Vec<ImportStatement>, 
    requests
}

pub struct ImportStatement {
    filename: String
}

pub struct VariableAssignment {

}

pub struct RequestDefinition {
    name: 
}




#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct HttppParser;

fn parse_(input: &Pair) {
    println!("{:?}", input)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn parse_file<'b>(file_name: &str) -> Result<HTTTPProgram, Error<Rule>> {
    let unparsed_file = fs::read_to_string(file_name).expect("cannot read file");

    let mut results: Vec<HTTPPTokens<'b>> = Vec::new();

    let mut parsed = HttppParser::parse(Rule::program, file)?.next().unwrap();

    let input = parsed.next().unwrap();

    match input.as_rule() {
        Rule::method => asdw(&input),
        _ => unreachable!(),
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_method_parse() {}
}
