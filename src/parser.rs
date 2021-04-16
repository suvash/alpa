use pest::error::Error as PError;
use pest::iterators::Pair;
use pest::Parser;

use crate::sankhya::Sankhya;
use crate::types::{Expr, Operation, Symbol};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct AlpaParser;

pub fn parse(source: &str) -> Option<Expr> {
    match parse_result(source) {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn parse_result(source: &str) -> Result<Expr, PError<Rule>> {
    let parse_tree_pair = AlpaParser::parse(Rule::alpa, source)?.next().unwrap();

    let parsed_expr = match parse_tree_pair.as_rule() {
        Rule::program => parse_s_expression(parse_tree_pair),
        _ => unreachable!(),
    };

    Ok(parsed_expr)
}

fn parse_s_expression(pair: Pair<Rule>) -> Expr {
    Expr::SExpr(parse_expressions(pair))
}

fn parse_expressions(pair: Pair<Rule>) -> Vec<Box<Expr>> {
    let mut exprs: Vec<Box<Expr>> = vec![];

    let pairs = pair.into_inner();
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                exprs.push(Box::new(parse_expression(pair)));
            }
            _ => unreachable!(),
        }
    }

    exprs
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
    let sankhya = pair.as_str().parse::<Sankhya>().unwrap();
    Expr::Int(sankhya.0)
}

fn parse_symbol(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let symbol = match pair.as_rule() {
        Rule::add => Symbol::Operation(Operation::Add),
        Rule::subtract => Symbol::Operation(Operation::Subtract),
        Rule::multiply => Symbol::Operation(Operation::Multiply),
        Rule::divide => Symbol::Operation(Operation::Divide),
        _ => unreachable!(),
    };

    Expr::Sym(symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let input = "+ +२ -५ (* ४ ५) (/ -१० २) (- -१ ५)";
        let expected = Expr::SExpr(vec![
            Box::new(Expr::Sym(Symbol::Operation(Operation::Add))),
            Box::new(Expr::Int(2)),
            Box::new(Expr::Int(-5)),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Multiply))),
                Box::new(Expr::Int(4)),
                Box::new(Expr::Int(5)),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Divide))),
                Box::new(Expr::Int(-10)),
                Box::new(Expr::Int(2)),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Subtract))),
                Box::new(Expr::Int(-1)),
                Box::new(Expr::Int(5)),
            ])),
        ]);
        assert_eq!(parse(input), Some(expected));
    }

    #[test]
    fn test_parse_failure() {
        let input = "+ 2 5 (* 4 5)";
        assert_eq!(parse(input), None);
    }
}
