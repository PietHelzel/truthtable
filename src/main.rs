mod ast;
mod bitcartesiann;
mod parser;
mod visualise;

use parser::*;
use visualise::visualise;

fn main() {
    let i = "(a&b)|((!a)&(!b))";

    let parsed_expr = parse(i).unwrap();

    println!("{parsed_expr}");

    let eval_expr = parsed_expr.evaluate_all().unwrap();

    visualise(eval_expr, i);
}
