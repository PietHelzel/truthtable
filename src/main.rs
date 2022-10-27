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
    
    let mut steps = parsed_expr.get_steps();
    
    let steps_len = steps.len();
    if steps_len > 0 {
        steps = steps.into_iter().take(steps_len - 1).collect();
    }

    visualise(eval_expr, steps, i);
}
