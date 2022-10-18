use indexmap::IndexMap;

use prettytable::{Table, Row, Cell, row};

pub fn visualise(expr: Vec<(IndexMap<String, bool>, bool)>, input: &str) {
    if expr.is_empty() {
        return;
    }

    let mut table = Table::new();

    let mut r = Row::empty();
    for c in expr[0].0.keys() {
        r.add_cell(Cell::new(&c.to_string()));
    }
    r.add_cell(Cell::new(input));
    table.add_row(r);

    for (map, result) in expr {
        let mut r = Row::empty();
        for c in map.values() {
            r.add_cell(Cell::new(&c.to_string()));
        }
        r.add_cell(Cell::new(&result.to_string()));

        table.add_row(r);
    }

    table.printstd();
}