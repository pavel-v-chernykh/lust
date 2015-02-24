use std::collections::HashMap;
use ast::Expr;

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError,
    IncorrectSpecialForm,
    IncorrectNumberOfArguments,
    IncorrectTypeOfArgument
}

pub struct Context {
    env: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context {
        let mut ctx = Context {
            env: HashMap::new(),
        };
        ctx.env.insert("nil".to_string(), Expr::Symbol("nil".to_string()));
        ctx.env.insert("true".to_string(), Expr::Bool(true));
        ctx.env.insert("false".to_string(), Expr::Bool(false));
        ctx
    }

    pub fn eval(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        match *s {
            Expr::Number(_) => {
                Ok(s.clone())
            },
            Expr::Bool(_) => {
                Ok(s.clone())
            },
            Expr::Symbol(ref name) => {
                if let Some(s) = self.env.get(name) {
                    Ok(s.clone())
                } else {
                    Err(EvalError::EvalError)
                }
            },
            Expr::String(ref s) => {
                Ok(Expr::String(s.clone()))
            },
            Expr::List(ref l) => {
                if let Expr::Symbol(ref n) = *l.first().unwrap() {
                    match &n[..] {
                        "def" => {
                            self.eval_def(s)
                        },
                        "+" => {
                            self.eval_plus(s)
                        },
                        "-" => {
                            self.eval_minus(s)
                        },
                        "/" => {
                            self.eval_div(s)
                        },
                        "*" => {
                            self.eval_mul(s)
                        },
                        _ => {
                            Err(EvalError::EvalError)
                        }
                    }
                } else {
                    Err(EvalError::EvalError)
                }
            }
        }
    }

    fn eval_def(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = *l.first().unwrap() {
                match &n[..] {
                    "def" => {
                        if l.len() == 3 {
                            if let Expr::Symbol(ref n) = l[1] {
                                self.eval(&l[2]).and_then(|a| {
                                    self.env.insert(n.clone(), a.clone());
                                    Ok(a)
                                })
                            } else {
                                Err(EvalError::EvalError)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_plus(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = *l.first().unwrap() {
                match &n[..] {
                    "+" => {
                        if l.len() > 1 {
                            let mut a = 0_f64;
                            for i in l.iter().skip(1) {
                                match self.eval(i) {
                                    Ok(Expr::Number(n)) => {
                                        a += n
                                    },
                                    _ => {
                                        return Err(EvalError::IncorrectTypeOfArgument)
                                    }
                                }
                            }
                            Ok(Expr::Number(a))
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_minus(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = *l.first().unwrap() {
                match &n[..] {
                    "-" => {
                        if l.len() > 1 {
                            if let Expr::Number(n) = l[1] {
                                let mut a = n;
                                for i in l.iter().skip(2) {
                                    match self.eval(i) {
                                        Ok(Expr::Number(n)) => {
                                            a -= n
                                        },
                                        _ => {
                                            return Err(EvalError::IncorrectTypeOfArgument)
                                        }
                                    }
                                }
                                Ok(Expr::Number(a))
                            } else {
                                Err(EvalError::IncorrectTypeOfArgument)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_div(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = *l.first().unwrap() {
                match &n[..] {
                    "/" => {
                        if l.len() > 1 {
                            if let Expr::Number(n) = l[1] {
                                let mut a = n;
                                for i in l.iter().skip(2) {
                                    match self.eval(i) {
                                        Ok(Expr::Number(n)) => {
                                            a /= n
                                        },
                                        _ => {
                                            return Err(EvalError::IncorrectTypeOfArgument)
                                        }
                                    }
                                }
                                Ok(Expr::Number(a))
                            } else {
                                Err(EvalError::IncorrectTypeOfArgument)
                            }
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }

    fn eval_mul(&mut self, s: &Expr) -> Result<Expr, EvalError> {
        if let Expr::List(ref l) = *s {
            if let Expr::Symbol(ref n) = *l.first().unwrap() {
                match &n[..] {
                    "*" => {
                        if l.len() > 1 {
                            let mut a = 1_f64;
                            for i in l.iter().skip(1) {
                                match self.eval(i) {
                                    Ok(Expr::Number(n)) => {
                                        a *= n
                                    },
                                    _ => {
                                        return Err(EvalError::IncorrectTypeOfArgument)
                                    }
                                }
                            }
                            Ok(Expr::Number(a))
                        } else {
                            Err(EvalError::IncorrectNumberOfArguments)
                        }
                    },
                    _ => {
                        Err(EvalError::IncorrectSpecialForm)
                    }
                }
            } else {
                Err(EvalError::EvalError)
            }
        } else {
            Err(EvalError::EvalError)
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::Expr;
    use super::Context;
    use super::EvalError::EvalError;

    #[test]
    fn test_eval_number_to_itself() {
        let num = 10_f64;
        let mut ctx = Context::new();
        let expected_result = Expr::Number(num);
        let actual_result = ctx.eval(&Expr::Number(num));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_string_to_itself() {
        let s = "rust is awesome";
        let mut ctx = Context::new();
        let expected_result = Expr::String(s.to_string());
        let actual_result = ctx.eval(&Expr::String(s.to_string()));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_undefined_symbol_to_error() {
        let mut ctx = Context::new();
        let expected_result = EvalError;
        let actual_result = ctx.eval(&Expr::Symbol("a".to_string()));
        assert_eq!(expected_result, actual_result.err().unwrap());
    }

    #[test]
    fn test_eval_true_to_matching_bool() {
        let mut ctx = Context::new();
        let expected_result = Expr::Bool(true);
        let actual_result = ctx.eval(&Expr::Symbol("true".to_string()));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_false_to_matching_bool() {
        let mut ctx = Context::new();
        let expected_result = Expr::Bool(false);
        let actual_result = ctx.eval(&Expr::Symbol("false".to_string()));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nil_to_itself() {
        let mut ctx = Context::new();
        let expected_result = Expr::Symbol("nil".to_string());
        let actual_result = ctx.eval(&Expr::Symbol("nil".to_string()));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_def_special_form() {
        let num = 1_f64;
        let mut ctx = Context::new();
        let expected_result = Expr::Number(num);
        let actual_input = Expr::List(vec![Expr::Symbol("def".to_string()),
                                           Expr::Symbol("a".to_string()),
                                           Expr::Number(num)]);
        let actual_result = ctx.eval(&actual_input);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_nested_plus_special_form() {
        let mut ctx = Context::new();
        let actual_input = Expr::List(vec![Expr::Symbol("+".to_string()),
                                           Expr::List(vec![Expr::Symbol("+".to_string()),
                                                           Expr::Number(1_f64),
                                                           Expr::Number(2_f64)]),
                                           Expr::Number(3_f64)]);
        let actual_result = ctx.eval(&actual_input);
        let expected_result = Expr::Number(6_f64);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_minus_special_form() {
        let mut ctx = Context::new();
        let actual_input = Expr::List(vec![Expr::Symbol("-".to_string()),
                                           Expr::Number(3_f64),
                                           Expr::Number(2_f64)]);
        let actual_result = ctx.eval(&actual_input);
        let expected_result = Expr::Number(1_f64);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_div_special_form() {
        let mut ctx = Context::new();
        let actual_input = Expr::List(vec![Expr::Symbol("/".to_string()),
                                           Expr::Number(3_f64),
                                           Expr::Number(2_f64)]);
        let actual_result = ctx.eval(&actual_input);
        let expected_result = Expr::Number(1.5);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_mul_special_form() {
        let mut ctx = Context::new();
        let actual_input = Expr::List(vec![Expr::Symbol("*".to_string()),
                                           Expr::Number(3.5),
                                           Expr::Number(2_f64)]);
        let actual_result = ctx.eval(&actual_input);
        let expected_result = Expr::Number(7_f64);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_plus_special_form_using_defined_symbols() {
        let mut ctx = Context::new();
        ctx.eval(&Expr::List(vec![Expr::Symbol("def".to_string()),
                                  Expr::Symbol("a".to_string()),
                                  Expr::Number(1_f64)])).ok().unwrap();
        ctx.eval(&Expr::List(vec![Expr::Symbol("def".to_string()),
                                  Expr::Symbol("b".to_string()),
                                  Expr::Number(2_f64)])).ok().unwrap();
        let expected_result = Expr::Number(3_f64);
        let actual_result = ctx.eval(&Expr::List(vec![Expr::Symbol("+".to_string()),
                                                      Expr::Symbol("a".to_string()),
                                                      Expr::Symbol("b".to_string())]));
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }
}