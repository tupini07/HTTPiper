use std::fs;
use pest_consume::Parser;

use pest_consume::Error;
type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

#[pest_consume::parser]
impl IdentParser {
    fn ident(input: Node) -> Result<()> {
        println!("{:?}", input);
        Ok(())
    }
    fn alpha(input: Node) -> Result<()> {
        println!("{:?}", input);
        Ok(())
    }
    fn digit(input: Node) -> Result<()> {
        println!("{:?}", input);
        Ok(())
    }

    fn ident_list(input: Node) -> Result<()> {
        println!("{:?}", input.as_rule());
        for c in input.children() {
            println!("child: {}", c)
        }
        Ok(())
    }
}

fn parse_csv(input_str: &str) -> Result<()> {
    // Parse the input into `Nodes`
    let inputs = IdentParser::parse(Rule::ident_list, input_str)?;
    // There should be a single root node in the parsed tree
    let input = inputs.single()?;
    // Consume the `Node` recursively into the final value
    IdentParser::ident_list(input);
    Ok(())
}

fn main() {
    match parse_csv("a1 b2") {
        Ok(_) => println!("ok"), 
        Err(e) => println!("error: {}", e)
    };
}
