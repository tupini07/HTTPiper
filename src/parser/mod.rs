use pest::error::Error;
use pest::Parser;
use std::fs;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

type Program = Vec<ProgramStatement>;
type SubstitutionableContent = Vec<SubstitutionContentParts>;

#[derive(Debug)]
pub enum RequestResponse {
    Request,
    Response,
}

#[derive(Debug)]
pub enum RequestParts {
    Body,
    Headers,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct SubstitutionDetails {
    root: SubstitutionParts,
    commands: Vec<String>,
}

#[derive(Debug)]
pub enum SubstitutionContentParts {
    NoSobstitution(String),
    Substitution(SubstitutionDetails),
}

#[derive(Debug)]
pub enum ProgramStatement {
    ImportFileName(String),
    VariableAssignment {
        name: String,
        value: SubstitutionableContent,
    },
    RequestDefinition(RequestDefinition),
}

#[derive(Debug)]
pub struct SingleHeader {
    key: String,
    value: SubstitutionableContent,
}

#[derive(Debug)]
pub enum BodyValues {
    Array(Vec<BodyValues>),
    Object(Vec<(String, BodyValues)>),
    Terminal(SubstitutionableContent),
}

#[derive(Debug)]
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn parse_import(input: Pair) -> ProgramStatement {
    let filename: Pair = input.into_inner().next().unwrap();
    ProgramStatement::ImportFileName(filename.as_str().to_owned())
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
            Rule::import => println!("{:?}", parse_import(x)),
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

    #[test_case("asd.txt")]
    #[test_case("weoirj.e39")]
    #[test_case("32.e")]
    fn parsing_import(desired_fn: &str) {
        let import_statement = format!("import \"{}\"", desired_fn);

        let import: Pair = HttppParser::parse(Rule::import, &import_statement)
            .unwrap()
            .next()
            .unwrap();

        let processed: ProgramStatement = parse_import(import);
        if let ProgramStatement::ImportFileName(fname) = processed {
            assert_eq!(fname, desired_fn);
        } else {
            panic!("No file name was extracted from import statement!");
        }
    }
}
