use pest::error::Error as PError;
use pest::iterators::Pair;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct AlpaParser;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Int(i32),
    Sym(Symbol),
    SExpr(Vec<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivideByZero,
}

pub type AST = Vec<Expr>;

pub fn parse(source: &str) -> Option<AST> {
    match parse_result(source) {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn parse_result(source: &str) -> Result<AST, PError<Rule>> {
    let parse_tree_pair = AlpaParser::parse(Rule::alpa, source)?.next().unwrap();

    let parsed_ast = match parse_tree_pair.as_rule() {
        Rule::program => parse_expressions(parse_tree_pair),
        _ => unreachable!(),
    };

    Ok(parsed_ast)
}

fn parse_expressions(pair: Pair<Rule>) -> Vec<Expr> {
    let mut exprs: Vec<Expr> = vec![];

    let pairs = pair.into_inner();
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                exprs.push(parse_expression(pair));
            }
            _ => unreachable!(),
        }
    }

    exprs
}

fn parse_s_expression(pair: Pair<Rule>) -> Expr {
    Expr::SExpr(parse_expressions(pair))
}

fn parse_expression(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::number => parse_number(pair),
        Rule::symbol => parse_symbol(pair),
        Rule::sexpr => parse_s_expression(pair),
        _ => unreachable!(),
    }
}

fn parse_number(pair: Pair<Rule>) -> Expr {
    let sankhya = pair.as_str().parse::<i32>().unwrap();
    Expr::Int(sankhya)
}

fn parse_symbol(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let symbol = match pair.as_rule() {
        Rule::add => Symbol::Add,
        Rule::subtract => Symbol::Subtract,
        Rule::multiply => Symbol::Multiply,
        Rule::divide => Symbol::Divide,
        _ => unreachable!(),
    };

    Expr::Sym(symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let input = "+ 2 5 (* 4 5)";
        let expected = vec![
            Expr::Sym(Symbol::Add),
            Expr::Int(2),
            Expr::Int(5),
            Expr::SExpr(vec![
                Expr::Sym(Symbol::Multiply),
                Expr::Int(4),
                Expr::Int(5),
            ]),
        ];
        assert_eq!(parse(input), Some(expected));
    }

    #[test]
    fn test_parse_failure() {
        let input = "random stuff";
        assert_eq!(parse(input), None);
    }
}
