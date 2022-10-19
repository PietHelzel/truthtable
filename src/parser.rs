use crate::ast::Expr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::delimited,
    sequence::{preceded, separated_pair},
    IResult
};

//Type definition for the default return value of the parsers
type ExprRes<'a> = IResult<&'a str, Box<Expr>>;

//Detects a factor surrounded by parentheses
fn parens(input: &str) -> ExprRes {
    delimited(tag("("), expression, tag(")"))(input)
}

//Detects either an identifier or an expression surrounded by parentheses
fn factor(input: &str) -> ExprRes {
    alt((identifier, parens))(input)
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
fn operator_and(input: &str) -> ExprRes {
    let (input, (factor1, factor2)) = separated_pair(factor, tag("&"), factor)(input)?;

    let expr = Box::new(Expr::And(factor1, factor2));
    Ok((input, expr))
}

//Detects the "or" operator, with the syntax "x|y"
fn operator_or(input: &str) -> ExprRes {
    let (input, (factor1, factor2)) = separated_pair(factor, tag("|"), factor)(input)?;

    let expr = Box::new(Expr::Or(factor1, factor2));
    Ok((input, expr))
}

//Detects all kinds of expressions, both operators and identifiers
fn expression(input: &str) -> ExprRes {
    alt((operator_and, operator_or, operator_not, identifier, parens))(input)
}

pub fn parse(input: &str) -> Option<Expr> {
    let input = input.replace(" ", "").replace("\n", "");
    match expression(&input) {
        Ok((_, result)) => Some(*result),
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
