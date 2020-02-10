use pest_consume::Parser;
use pest_consume::Error;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct HttppParser;

#[pest_consume::parser]
impl HttppParser {
    pub fn request_signature(input: Node) -> Result<()> {
        Ok(())
    }

    pub fn method(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }
}




#[cfg(test)]
mod tests {
    use super::HttppParser;

    #[test]
    fn test_method_parse (){
        let want = "GET";
        let parsed = HttppParser::parse(Rule::method, "GET");
    }
}