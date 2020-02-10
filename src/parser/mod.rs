use pest::error::Error;
use pest::Parser;
use std::fs;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

type Program = Vec<ProgramStatements>;

pub enum RequestResponse {
    Request,
    Response,
}

pub enum RequestParts {
    Body,
    Headers,
}

pub enum SubstitutionParts {
    Empty,
    VariableReference(String),
    NoSubtitution(String),
    RequestReference {
        requestName: String,
        reqResp: RequestResponse,
        part: RequestParts,
    },
}

pub struct SubstitutionDetails {
    root: SubstitutionParts,
    commands: Vec<String>,
}

pub enum SubstitutionContentParts {
    NoSobstitution(String),
    Substitution(SubstitutionDetails),
}

type SubstitutionableContent = Vec<SubstitutionContentParts>;

pub enum ProgramStatements {
    ImportFileName(String),
    VariableAssignment {
        name: String,
        value: SubstitutionableContent,
    },
    RequestDefinition(RequestDefinition),
}

pub struct KeyPair {
    key: String,
    value: SubstitutionableContent,
}

pub struct RequestDefinition {
    requestName: String,
    method: String,
    url: SubstitutionableContent,
    headers: Vec<KeyPair>,
    body: Vec<KeyPair>,
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

pub fn parse_file<'b>(file_name: &str) -> Result<(), Error<Rule>> {
    let unparsed_file = fs::read_to_string(file_name).expect("cannot read file");

    let mut parsed: Pairs = HttppParser::parse(Rule::program, &unparsed_file)?;

    let inpu1 = parsed.next().unwrap();

    // match input.as_rule() {
    //     Rule::method => asdw(&input),
    //     _ => unreachable!(),
    // };
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_method_parse() {}
}
