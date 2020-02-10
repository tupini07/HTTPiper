use crate::parser::*;
use crate::parser::entities as e;
use std::fs;

pub fn parse_import(input: Pair) -> e::ProgramStatement {
    let filename: Pair = input.into_inner().next().unwrap();
    e::ProgramStatement::ImportFileName(filename.as_str().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("asd.txt")]
    #[test_case("weoirj.e39")]
    #[test_case("32.e")]
    fn parsing_import(desired_fn: &str) {
        let import_statement = format!("import \"{}\"", desired_fn);

        let import: Pair = HttppParser::parse(Rule::import, &import_statement)
            .unwrap()
            .next()
            .unwrap();

        let processed: e::ProgramStatement = parse_import(import);
        if let e::ProgramStatement::ImportFileName(fname) = processed {
            assert_eq!(fname, desired_fn);
        } else {
            panic!("No file name was extracted from import statement!");
        }
    }
}
