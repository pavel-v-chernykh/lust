use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Atom {
    Number(f64),
    Symbol(String)
}

enum ParseAtomError {
    IncorrectSymbolName
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom::Number(n) => {
                write!(f, "{}", n)
            },
            Atom::Symbol(ref name) => {
                write!(f, "'{}", name)
            }
        }
    }
}

impl FromStr for Atom {
    type Err = ParseAtomError;

    fn from_str(s: &str) -> Result<Atom, ParseAtomError> {
        match s.parse::<f64>() {
            Ok(f) => {
                Ok(Atom::Number(f))
            },
            Err(..) => {
                match s.chars().next() {
                    Some(c) => {
                        if c.is_numeric() {
                            Err(ParseAtomError::IncorrectSymbolName)
                        } else {
                            Ok(Atom::Symbol(s.to_string()))
                        }
                    },
                    None => {
                        Err(ParseAtomError::IncorrectSymbolName)
                    }
                }
            }
        }
    }
}

fn tokenize(s: &str) -> Vec<String> {
    s.replace("("," ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .split(' ')
        .filter_map(|s| { if s.is_empty() { None } else { Some(s.to_string()) }})
        .collect()
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
}

#[cfg(test)]
mod test {
    use super::{Atom, tokenize};

    #[test]
    fn parse_integer() {
        assert_eq!(Atom::Number(64f64), "64".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn parse_float() {
        assert_eq!(Atom::Number(64.5), "64.5".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(Atom::Symbol("name".to_string()), "name".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn tokenize_dense_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize("(def a 1)"))
    }

    #[test]
    fn tokenize_sparse_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize(" ( \n def a\n1)   \n"))
    }
}
