use crate::parser::entities as e;
use crate::parser::*;
use std::fs;

pub fn parse_import(input: Pair) -> e::ProgramStatement {
    debug_assert_eq!(input.as_rule(), Rule::import);

    let filename: Pair = input.into_inner().next().unwrap();
    e::ProgramStatement::ImportFileName(filename.as_str().to_owned())
}

pub fn parse_reqAttribute(input: Pair) -> e::SubstitutionRoot {
    debug_assert_eq!(input.as_rule(), Rule::reqAttribute);

    let mut inner: Pairs = input.into_inner();

    let request_name_pair = inner.next().unwrap();
    debug_assert_eq!(request_name_pair.as_rule(), Rule::request_id);
    let request_name = request_name_pair.as_str().to_owned();

    let req_resp_pair = inner.next().unwrap();
    debug_assert_eq!(req_resp_pair.as_rule(), Rule::req_resp);
    let req_resp = req_resp_pair.as_str().to_owned();

    let req_part_pair = inner.next().unwrap();
    debug_assert_eq!(req_part_pair.as_rule(), Rule::body_headers);
    let req_part = req_part_pair.as_str().to_owned();

    e::SubstitutionRoot::RequestReference {
        name: request_name,
        req_resp: if req_resp == "Request" {
            e::RequestResponse::Request
        } else {
            e::RequestResponse::Response
        },
        part: if req_part == "Body" {
            e::RequestParts::Body
        } else {
            e::RequestParts::Headers
        },
    }
}

pub fn parse_substitution(input: Pair) -> e::SubstitutionDetails {
    debug_assert_eq!(input.as_rule(), Rule::substitution);

    let mut pairs = input.into_inner();

    let root_pair = pairs.next().unwrap();

    let root = match root_pair.as_rule() {
        Rule::variable_id => e::SubstitutionRoot::VariableReference(root_pair.as_str().to_owned()),
        Rule::empty => e::SubstitutionRoot::Empty,
        Rule::reqAttribute => parse_reqAttribute(root_pair),
        _ => unreachable!(),
    };

    let mut commands_vec: Vec<String> = vec![];

    if let Some(commands) = pairs.next() {
        let command_pairs: Pairs = commands.into_inner();

        commands_vec.append(
            command_pairs
                .map(|p: Pair| p.as_str().trim().to_owned())
                .collect::<Vec<String>>()
                .as_mut(),
        );
    }

    e::SubstitutionDetails {
        root: root,
        commands: commands_vec,
    }
}

pub fn parse_value(input: Pair) -> e::SubstitutionableContent {
    debug_assert_eq!(input.as_rule(), Rule::value);

    use e::SubstitutionContentParts::*;

    input
        .into_inner()
        .map(|p: Pair| match p.as_rule() {
            Rule::key | Rule::text => NoSobstitution(p.as_str().to_owned()),
            Rule::substitution => Substitution(parse_substitution(p)),
            _ => unreachable!(),
        })
        .collect()
}

pub fn parse_variable_assignment(input: Pair) -> e::ProgramStatement {
    debug_assert_eq!(input.as_rule(), Rule::var_assignment);

    let mut inner: Pairs = input.into_inner();

    let name = inner.next().unwrap().as_str().to_owned();
    let value = parse_value(inner.next().unwrap());

    e::ProgramStatement::VariableAssignment {
        name: name,
        value: value,
    }
}

pub fn parse_request(input: Pair) -> e::RequestDefinition {
    unimplemented!()
}

pub fn parse_request_headers(input: Pair) -> Vec<e::SingleHeader> {
    unimplemented!()
}

pub fn parse_request_body(input: Pait) -> e::BodyValues {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::entities::SubstitutionRoot;
    use test_case::test_case;

    fn parse<'a>(input: &'a str, rule: Rule) -> Pair<'a> {
        HttppParser::parse(rule, input).unwrap().next().unwrap()
    }

    #[test]
    fn parse_request() {
        unimplemented!()
    }

    #[test]
    fn parse_request_headers() {
        unimplemented!()
    }

    #[test]
    fn parse_request_body() {
        unimplemented!()
    }

    #[test]
    fn parse_value_with_no_substitution() {
        unimplemented!()
    }

    #[test]
    fn parse_value_with_plain_substitution_plain() {
        unimplemented!()
    }

    #[test]
    fn parse_value_with_substitution_no_commands() {
        unimplemented!()
    }

    #[test]
    fn parsing_value_with_substitution_and_three_commands() {
        let value_statement = format!("{}", "{{@fm > cat | grep -o | wc}}");
        let value_pair: Pair = parse(&value_statement, Rule::value);

        let processed = parse_value(value_pair);
        assert_eq!(
            processed.len(),
            1,
            "There should be exactly one part in this SubstitutionableContent"
        );

        match &processed[0] {
            e::SubstitutionContentParts::Substitution(details) => {
                assert_eq!(
                    details.root,
                    SubstitutionRoot::VariableReference("@fm".to_string())
                );

                assert_eq!(
                    details.commands.len(),
                    3,
                    "Expected example to have three commands"
                );
                assert_eq!(details.commands[0], "cat".to_string());
                assert_eq!(details.commands[1], "grep -o".to_string());
                assert_eq!(details.commands[2], "wc".to_string());
            }
            e::SubstitutionContentParts::NoSobstitution(_) => {
                panic!("This should actually be a substitution!")
            }
        };
    }

    #[test_case("@sdfRfrrr", "ewoirjwer")]
    #[test_case("@s", "ewsd{{ @ds > cat | mop }}ei")]
    #[test_case("@sfei", "{{ @ds > cat | mop }}")]
    fn parsing_var_assignment(var_name: &str, var_value: &str) {
        let var_assignment_statement = format!("{} = {}", var_name, var_value);

        let var_assignment: Pair = parse(&var_assignment_statement, Rule::var_assignment);

        let processed: e::ProgramStatement = parse_variable_assignment(var_assignment);
        match processed {
            e::ProgramStatement::VariableAssignment { name, value } => {
                assert_eq!(
                    name, var_name,
                    "Expected extracted variable name to be the same as the provided one"
                );
                assert!(
                    value.len() > 0,
                    "Expected the parser to extract at least one value."
                );
            }
            _ => panic!("No variable assignment was extracted!"),
        }
    }

    #[test_case("asd.txt")]
    #[test_case("weoirj.e39")]
    #[test_case("32.e")]
    fn parsing_import(desired_fn: &str) {
        let import_statement = format!("import \"{}\"", desired_fn);

        let import: Pair = parse(&import_statement, Rule::import);

        let processed: e::ProgramStatement = parse_import(import);
        if let e::ProgramStatement::ImportFileName(fname) = processed {
            assert_eq!(fname, desired_fn);
        } else {
            panic!("No file name was extracted from import statement!");
        }
    }
}
