mod ast;
mod bitcartesiann;
mod parser;
mod visualise;

use parser::*;
use visualise::visualise;

use std::env::args;

fn main() {
    let i = &args().nth(1).expect("No expression supplied via CLI");

    let parsed_expr = parse(i).expect("Malformed expression");

    let eval_expr = parsed_expr.evaluate_all().unwrap();

    visualise(eval_expr, i);
}
