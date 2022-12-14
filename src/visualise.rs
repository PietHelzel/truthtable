use indexmap::IndexMap;

use prettytable::{Table, Row, Cell};
use prettytable::format::Alignment;

use crate::ast::Expr;

use colored::*;

pub fn visualise(expr: Vec<(IndexMap<String, bool>, bool)>, steps: Vec<Expr>, input: &str) {
    if expr.is_empty() {
        return;
    }

    let mut table = Table::new();

    let mut r = Row::empty();
    for c in expr[0].0.keys() {
        r.add_cell(Cell::new(&c.to_string()));
    }
    for s in &steps {
        r.add_cell(Cell::new(&s.to_string()));
    }
    r.add_cell(Cell::new(input));
    table.add_row(r);

    for (map, result) in expr {
        let mut r = Row::empty();
        for c in map.values() {
            r.add_cell(Cell::new_align(&colorise(*c), Alignment::CENTER));
        }
        
        for s in &steps {
            let step_result = s.evaluate(&map).expect("Could not evaluate step");
            r.add_cell(Cell::new_align(&colorise(step_result), Alignment::CENTER));
        }
        
        r.add_cell(Cell::new_align(&colorise(result), Alignment::CENTER));

        table.add_row(r);
    }

    table.printstd();
}

fn colorise(a: bool) -> String {
    if a {
        (a as u8).to_string().green().to_string()
    } else {
        (a as u8).to_string().red().to_string()
    }
}