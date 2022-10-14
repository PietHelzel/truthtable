use crate::ast::Expr;

use nom::{
    IResult,
    character::{is_space, is_alphabetic},
    bytes::complete::{take_while, tag},
    branch::alt, sequence::tuple,
    sequence::delimited
};

// fn space(input: &str) -> IResult<&str, &str> {
//     take_while(|x| is_space(x as u8))(input)
// }

fn identifier(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, ident) = take_while(|x| is_alphabetic(x as u8))(input)?;
    let expr_ident = Box::new(Expr::Var(ident.to_string()));
    Ok((input, expr_ident))
}

fn operator_not(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, _) = tag("!")(input)?;
    let (input, ident) = identifier(input)?;
    let expr = Box::new(Expr::Not(ident));
    Ok((input, expr))
}

fn operator_and(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, op1) = identifier(input)?;
    let (input, _) = tag("&")(input)?;
    let (input, op2) = identifier(input)?;

    let expr = Box::new(Expr::And(op1, op2));
    Ok((input, expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parser_space() {
    //     assert_eq!(space("  abc  "), Ok(("abc  ", "  ")));
    // }

    #[test]
    fn test_parser_identifier() {
        assert_eq!(identifier("abc"), Ok(("", Box::new(Expr::Var("abc".to_string())))));
    }

    #[test]
    fn test_parser_operator_not() {
        assert_eq!(operator_not("!abc"), Ok(("", Box::new(
            Expr::Not(
                Box::new(Expr::Var("abc".to_string()))
            )
        ))));
    }

    #[test]
    fn test_parser_operator_and() {
        assert_eq!(operator_and("a&b"), Ok(("", Box::new(
            Expr::And(
                Box::new(Expr::Var("a".to_string())),
                Box::new(Expr::Var("b".to_string()))
            )
        ))));
    }
}