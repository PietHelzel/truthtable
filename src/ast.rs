use indexmap::map::IndexMap;

use std::fmt::Display;

use crate::bitcartesiann::BitCartesianN;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Var(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Implication(Box<Expr>, Box<Expr>),
    Biconditional(Box<Expr>, Box<Expr>),
    XOR(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(ident) => write!(f, "{ident}"),
            Expr::Not(a) => write!(f, "(!{})", a),
            Expr::And(a, b) => write!(f, "({}&{})", a, b),
            Expr::Or(a, b) => write!(f, "({}|{})", a, b),
            Expr::Implication(a, b) => write!(f, "({}=>{})", a, b),
            Expr::Biconditional(a, b) => write!(f, "({}<=>{})", a, b),
            Expr::XOR(a, b) => write!(f, "({}<!=>{})", a, b),
        }
    }
}

impl Expr {
    pub fn get_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self._get_vars_recursive(&mut vars);
        
        let mut vars_dedup = Vec::new();
        for i in vars {
            if !vars_dedup.contains(&i) {
                vars_dedup.push(i);
            }
        }

        vars_dedup
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
            },
            Expr::Implication(a, b) => {
                a._get_vars_recursive(vars);
                b._get_vars_recursive(vars);
            },
            Expr::Biconditional(a, b) => {
                a._get_vars_recursive(vars);
                b._get_vars_recursive(vars);
            },
            Expr::XOR(a, b) => {
                a._get_vars_recursive(vars);
                b._get_vars_recursive(vars);
            },
        }
    }
    
    pub fn get_steps(&self) -> Vec<Expr> {
        let mut steps = Vec::new();
        self._get_steps_recursive(&mut steps);
        steps
    }
    
    fn _get_steps_recursive(&self, steps: &mut Vec<Expr>) {
        match self {
            Expr::Var(_) => {},
            Expr::Not(a) => {
                a._get_steps_recursive(steps);
                steps.push(Expr::Not(a.clone()));
            },
            Expr::And(a, b) => {
                a._get_steps_recursive(steps);
                b._get_steps_recursive(steps);
                steps.push(Expr::And(a.clone(), b.clone()))
            },
            Expr::Or(a, b) => {
                a._get_steps_recursive(steps);
                b._get_steps_recursive(steps);
                steps.push(Expr::Or(a.clone(), b.clone()))
            },
            Expr::Implication(a, b) => {
                a._get_steps_recursive(steps);
                b._get_steps_recursive(steps);
                steps.push(Expr::Implication(a.clone(), b.clone()))
            },
            Expr::Biconditional(a, b) => {
                a._get_steps_recursive(steps);
                b._get_steps_recursive(steps);
                steps.push(Expr::Biconditional(a.clone(), b.clone()))
            },
            Expr::XOR(a, b) => {
                a._get_steps_recursive(steps);
                b._get_steps_recursive(steps);
                steps.push(Expr::XOR(a.clone(), b.clone()))
            }
        }
    }

    pub fn evaluate(&self, vars: &IndexMap<String, bool>) -> Option<bool> {
        Some(match self {
            Expr::Var(a) => *(vars.get(a)?),
            Expr::Not(a) => !a.evaluate(vars)?,
            Expr::And(a, b) => a.evaluate(vars)? && b.evaluate(vars)?,
            Expr::Or(a, b) => a.evaluate(vars)? || b.evaluate(vars)?,
            Expr::Implication(a, b) => {
                let a_eval = a.evaluate(vars)?;
                let b_eval = b.evaluate(vars)?;
                if a_eval && !b_eval {
                    false
                } else {
                    true
                }
            },
            Expr::Biconditional(a, b) => a.evaluate(vars) == b.evaluate(vars),
            Expr::XOR(a, b) => a.evaluate(vars) != b.evaluate(vars),
        })
    }

    pub fn evaluate_all(&self) -> Option<Vec<(IndexMap<String, bool>, bool)>> {
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
    
    #[test]
    fn test_ast_get_steps() {
        let expr = And(
            Box::new(Var("A".to_string())),
            Box::new(Or(
                Box::new(Var("B".to_string())),
                Box::new(
                    Not(Box::new(Var("C".to_string())))
                ),
            )),
        );
        let result: Vec<_> = expr.get_steps().iter().map(|s| format!("{}", s)).collect();
        
        assert_eq!(result, vec![
            "(!C)".to_string(),
            "(B|(!C))".to_string(),
            "(A&(B|(!C)))".to_string(),
        ])
    }
}
