use pest::error::Error as PError;
use pest::iterators::Pair;
use pest::Parser;

use crate::ntypes::Sankhya;
use crate::types::{Expr, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct AlpaParser;

pub fn parse(source: &str) -> Result<Expr, PError<Rule>> {
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

fn parse_q_expression(pair: Pair<Rule>) -> Expr {
    Expr::QExpr(parse_expressions(pair))
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
        Rule::qexpr => parse_q_expression(pair),
        _ => unreachable!(),
    }
}

fn parse_number(pair: Pair<Rule>) -> Expr {
    Expr::Num(pair.as_str().parse::<Sankhya>().unwrap())
}

fn parse_symbol(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::identifier => parse_identifier(pair),
        Rule::numbers_op => parse_numbers_op(pair),
        Rule::qexpr_op => parse_qexpr_op(pair),
        Rule::qexprs_op => parse_qexprs_op(pair),
        Rule::sexpr_op => parse_sexpr_op(pair),
        _ => unreachable!(),
    }
}

fn parse_identifier(pair: Pair<Rule>) -> Expr {
    Expr::Sym(Symbol::Identifier(pair.as_str().to_string()))
}

fn parse_numbers_op(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let op = match pair.as_rule() {
        Rule::add => NumOp::Add,
        Rule::subtract => NumOp::Subtract,
        Rule::multiply => NumOp::Multiply,
        Rule::divide => NumOp::Divide,
        _ => unreachable!(),
    };

    Expr::Sym(Symbol::NumOp(op))
}

fn parse_qexpr_op(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let op = match pair.as_rule() {
        Rule::first => QExprOp::First,
        Rule::rest => QExprOp::Rest,
        Rule::len => QExprOp::Len,
        Rule::eval => QExprOp::Eval,
        _ => unreachable!(),
    };

    Expr::Sym(Symbol::QExprOp(op))
}

fn parse_qexprs_op(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let op = match pair.as_rule() {
        Rule::join => QExprsOp::Join,
        Rule::cons => QExprsOp::Cons,
        _ => unreachable!(),
    };

    Expr::Sym(Symbol::QExprsOp(op))
}

fn parse_sexpr_op(pair: Pair<Rule>) -> Expr {
    let pair = pair.into_inner().next().unwrap();

    let op = match pair.as_rule() {
        Rule::quote => SExprOp::Quote,
        _ => unreachable!(),
    };

    Expr::Sym(Symbol::SExprOp(op))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let input = "+ +२ -५ () (* ४ ५) (/ -१० २) (- -१ ५)";
        let expected = Expr::SExpr(vec![
            Box::new(Expr::Sym(Symbol::NumOp(NumOp::Add))),
            Box::new(Expr::Num(Sankhya(2))),
            Box::new(Expr::Num(Sankhya(-5))),
            Box::new(Expr::SExpr(vec![])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Multiply))),
                Box::new(Expr::Num(Sankhya(4))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Divide))),
                Box::new(Expr::Num(Sankhya(-10))),
                Box::new(Expr::Num(Sankhya(2))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Subtract))),
                Box::new(Expr::Num(Sankhya(-1))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
        ]);
        assert_eq!(parse(input), Ok(expected));
    }
}
