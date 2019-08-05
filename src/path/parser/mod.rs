use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::path::Path;

#[derive(Parser)]
#[grammar = "path/parser/grammar.pest"]
struct PathParser;

fn remove_string_quotes(input: &str) -> String {
    match input.chars().next().expect("invalid grammar: no string quotes") {
        '\'' => input.replace('\'', "").to_string(),
        _ => unreachable!("invalid grammar"),
    }
}

fn parse_path(pair: Pair<Rule>) -> Result<Path, String> {
    let mut path = Path::new();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::identifier => path.push(p.as_str()),
            Rule::string => path.push(remove_string_quotes(p.as_str())),
            Rule::positive_integer => {
                let pint: usize = p
                    .as_str()
                    .parse()
                    .map_err(|_| format!("unable to parse '{}' as positive integer", p.as_str()))?;
                path.push(pint);
            }
            _ => unreachable!("invalid grammar"),
        };
    }

    Ok(path)
}
fn parse_content(pair: Pair<Rule>) -> Result<Path, String> {
    let inner = pair.into_inner().next().expect("invalid grammar");

    match inner.as_rule() {
        Rule::path => parse_path(inner),
        _ => unreachable!("invalid grammar"),
    }
}

pub(crate) fn parse(path: &str) -> Result<Path, String> {
    let mut pairs = PathParser::parse(Rule::content, path).map_err(|e| format!("unable to parse path: {}", e))?;
    let next = pairs.next().ok_or_else(|| format!("unable to parse path: {}", path))?;
    parse_content(next)
}
