use std::collections::HashMap;
use common::Atom::{Number, Symbol};
use common::Sexp::{self, Atom};

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError
}

fn eval(s: Sexp, e: &HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    match s {
        Atom(Number(_)) => {
            Ok(s)
        },
        Atom(Symbol(ref name)) => {
            if let Some(s) = e.get(name) {
                Ok(s.clone())
            } else {
                Err(EvalError::EvalError)
            }
        }
        _ => {
            Err(EvalError::EvalError)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use common::Atom::{Number, Symbol, Nil};
    use common::Sexp::Atom;
    use super::EvalError::EvalError;
    use super::eval;

    #[test]
    fn test_eval_atom_number() {
        let number = 10f64;
        let expected_result = Atom(Number(number));
        let actual_result = eval(Atom(Number(number)), &HashMap::new());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_number() {
        let num = 10f64;
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Number(num)));
        let expected_result = Atom(Number(num));
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_nil() {
        let mut env = HashMap::new();
        env.insert("a".to_string(), Atom(Nil));
        let expected_result = Atom(Nil);
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_non_value() {
        let env = HashMap::new();
        let expected_result = EvalError;
        let actual_result = eval(Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.err().unwrap());
    }
}