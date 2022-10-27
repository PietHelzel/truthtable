use crate::ast::Expr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    multi::many1,
    sequence::delimited,
    sequence::preceded,
    IResult,
};

//Type definition for the default return value of the parsers
type ExprRes<'a> = IResult<&'a str, Box<Expr>>;

enum ZipOp {
    And,
    Or,
    Implication,
    Biconditional,
    XOR,
}

fn zip(factors: Vec<Box<Expr>>, op: ZipOp) -> Option<Box<Expr>> {
    if factors.len() < 2 {
        return None;
    }

    let mut factors = factors.into_iter();

    let mut expr = match op {
        ZipOp::And => Expr::And(factors.nth(0)?, factors.nth(0)?),
        ZipOp::Or => Expr::Or(factors.nth(0)?, factors.nth(0)?),
        ZipOp::Implication => Expr::Implication(factors.nth(0)?, factors.nth(0)?),
        ZipOp::Biconditional => Expr::Biconditional(factors.nth(0)?, factors.nth(0)?),
        ZipOp::XOR => Expr::XOR(factors.nth(0)?, factors.nth(0)?),
    };

    for f in factors.into_iter() {
        expr = match op {
            ZipOp::And => Expr::And(Box::new(expr), f),
            ZipOp::Or => Expr::Or(Box::new(expr), f),
            ZipOp::Implication => Expr::Implication(Box::new(expr), f),
            ZipOp::Biconditional => Expr::Biconditional(Box::new(expr), f),
            ZipOp::XOR => Expr::XOR(Box::new(expr), f),
        };
    }

    Some(Box::new(expr))
}

//Detects a factor surrounded by parentheses
fn parens(input: &str) -> ExprRes {
    delimited(tag("("), expression, tag(")"))(input)
}

//Detects either an identifier or an expression surrounded by parentheses
fn factor(input: &str) -> ExprRes {
    alt((identifier, parens, operator_not))(input)
}

//Detects an identifier of a variable, which is an alphabetic word
fn identifier(input: &str) -> ExprRes {
    let (input, ident) = alphanumeric1(input)?;
    let expr_ident = Box::new(Expr::Var(ident.to_string()));
    Ok((input, expr_ident))
}

//Detects the "not" operator, with the syntax "!x"
fn operator_not(input: &str) -> ExprRes {
    let (input, term) = preceded(tag("!"), factor)(input)?;

    Ok((input, Box::new(Expr::Not(term))))
}

//Detects the "and" operator, with the syntax "x&y"
//Chains like a&b&c become (a&b)&c
fn operator_and(input: &str) -> ExprRes {
    let (input, factor1) = factor(input)?;

    let (input, mut factors) = many1(preceded(tag("&"), factor))(input)?;

    factors.insert(0, factor1);

    let expr = zip(factors, ZipOp::And)
        .expect("Wrong number of arguments for factor zipping. This should be impossible :O");
    Ok((input, expr))
}

//Detects the "or" operator, with the syntax "x|y"
fn operator_or(input: &str) -> ExprRes {
    let (input, factor1) = factor(input)?;

    let (input, mut factors) = many1(preceded(tag("|"), factor))(input)?;

    factors.insert(0, factor1);

    let expr = zip(factors, ZipOp::Or)
        .expect("Wrong number of arguments for factor zipping This should be impossible :O");
    Ok((input, expr))
}

//Detects the "implication" operator, with the syntax "x=>y"
fn operator_implication(input: &str) -> ExprRes {
    let (input, factor1) = factor(input)?;

    let (input, mut factors) = many1(preceded(tag("=>"), factor))(input)?;

    factors.insert(0, factor1);

    let expr = zip(factors, ZipOp::Implication)
        .expect("Wrong number of arguments for factor zipping This should be impossible :O");
    Ok((input, expr))
}

//Detects the "biconditional" operator, with the syntax "x<=>y"
fn operator_biconditional(input: &str) -> ExprRes {
    let (input, factor1) = factor(input)?;

    let (input, mut factors) = many1(preceded(tag("<=>"), factor))(input)?;

    factors.insert(0, factor1);

    let expr = zip(factors, ZipOp::Biconditional)
        .expect("Wrong number of arguments for factor zipping This should be impossible :O");
    Ok((input, expr))
}

//Detects the "XOR" operator, with the syntax "x<!=>y"
fn operator_xor(input: &str) -> ExprRes {
    let (input, factor1) = factor(input)?;

    let (input, mut factors) = many1(preceded(tag("<!=>"), factor))(input)?;

    factors.insert(0, factor1);

    let expr = zip(factors, ZipOp::XOR)
        .expect("Wrong number of arguments for factor zipping This should be impossible :O");
    Ok((input, expr))
}

//Detects all kinds of expressions, both operators and identifiers
fn expression(input: &str) -> ExprRes {
    alt((
        operator_and,
        operator_or,
        operator_implication,
        operator_biconditional,
        operator_xor,
        operator_not,
        identifier,
        parens,
    ))(input)
}

pub fn parse(input: &str) -> Option<Expr> {
    let input = input.replace(" ", "").replace("\n", "");
    match expression(&input) {
        Ok((remainder, result)) => {
            if remainder.len() == 0 {
                Some(*result)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_identifier() {
        assert_eq!(
            identifier("abc"),
            Ok(("", Box::new(Expr::Var("abc".to_string()))))
        );
    }

    #[test]
    fn test_parser_operator_not() {
        assert_eq!(
            operator_not("!abc"),
            Ok((
                "",
                Box::new(Expr::Not(Box::new(Expr::Var("abc".to_string()))))
            ))
        );
    }

    #[test]
    fn test_parser_operator_and() {
        assert_eq!(
            operator_and("a&b"),
            Ok((
                "",
                Box::new(Expr::And(
                    Box::new(Expr::Var("a".to_string())),
                    Box::new(Expr::Var("b".to_string()))
                ))
            ))
        );
    }

    #[test]
    fn test_parser_operator_or() {
        assert_eq!(
            operator_or("a|b"),
            Ok((
                "",
                Box::new(Expr::Or(
                    Box::new(Expr::Var("a".to_string())),
                    Box::new(Expr::Var("b".to_string()))
                ))
            ))
        );
    }

    #[test]
    fn test_parser_expression() {
        assert_eq!(
            expression("a&(!b)"),
            Ok((
                "",
                Box::new(Expr::And(
                    Box::new(Expr::Var("a".to_string())),
                    Box::new(Expr::Not(Box::new(Expr::Var("b".to_string()))))
                ))
            ))
        );
    }
}
