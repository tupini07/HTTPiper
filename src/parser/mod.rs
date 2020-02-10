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
        name: String,
        req_resp: RequestResponse,
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

pub struct SingleHeader {
    key: String,
    value: SubstitutionableContent,
}

pub enum BodyValues {
    Array(Vec<BodyValues>),
    Object(Vec<(String, BodyValues)>),
    Terminal(SubstitutionableContent),
}

pub struct RequestDefinition {
    name: String,
    method: String,
    url: SubstitutionableContent,
    headers: Vec<SingleHeader>,
    body: BodyValues,
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

    let mut program: Pair = HttppParser::parse(Rule::program, &unparsed_file)?
        .next()
        .unwrap();

    println!(
        "{:?}",
        program
            .into_inner()
            .map(|p: Pair| format!("{:?}", p.as_rule()))
            .collect::<Vec<String>>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
