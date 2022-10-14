use indexmap::map::IndexMap;

use crate::bitcartesiann::BitCartesianN;

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn get_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self._get_vars_recursive(&mut vars);
        vars
    }

    fn _get_vars_recursive(&self, vars: &mut Vec<String>) {
        match self {
            Expr::Var(a) => vars.push(a.clone()),
            Expr::Not(a) => a._get_vars_recursive(vars),
            Expr::And(a, b) => {
                a._get_vars_recursive(vars);
                b._get_vars_recursive(vars);
            }
            Expr::Or(a, b) => {
                a._get_vars_recursive(vars);
                b._get_vars_recursive(vars);
            }
        }
    }

    fn evaluate(&self, vars: &IndexMap<String, bool>) -> Option<bool> {
        Some(match self {
            Expr::Var(a) => *(vars.get(a)?),
            Expr::Not(a) => !a.evaluate(vars)?,
            Expr::And(a, b) => a.evaluate(vars)? && b.evaluate(vars)?,
            Expr::Or(a, b) => a.evaluate(vars)? || b.evaluate(vars)?,
        })
    }

    fn evaluate_all(&self) -> Option<Vec<(IndexMap<String, bool>, bool)>> {
        let vars = self.get_vars();
        let vars_len = vars.len() as u32;

        let mut result = Vec::new();

        for combi in BitCartesianN::new(vars_len) {
            let var_map: IndexMap<_, _> = vars
                .iter()
                .enumerate()
                .map(|(i, v)| (v.clone(), combi[i]))
                .collect();

            let eval = self.evaluate(&var_map)?;
            result.push((var_map, eval));
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    #[test]
    fn test_ast_get_vars() {
        let expr = And(
            Box::new(Expr::Var("A".to_string())),
            Box::new(Expr::Or(
                Box::new(Expr::Var("B".to_string())),
                Box::new(Expr::Var("C".to_string())),
            )),
        );

        assert_eq!(expr.get_vars(), vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_ast_evaluate_all() {
        use Expr::*;
        let expr = And(
            Box::new(Var("A".to_string())),
            Box::new(Or(
                Box::new(Var("B".to_string())),
                Box::new(
                    Not(Box::new(Var("C".to_string())))
                ),
            )),
        );
        let result = expr.evaluate_all().unwrap();
        for test_r in result {
            let correct_r = expr.evaluate(&test_r.0).unwrap();
            assert_eq!(correct_r, test_r.1);
        }
    }
}
