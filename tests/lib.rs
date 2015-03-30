#[macro_use]
extern crate lust;

use lust::{Parser, Scope};

#[test]
fn test_macro_expansion() {
    let ref mut scope = Scope::new_std();
    let input = "(def m (macro (a) '(+ 1 ~a)))";
    let expr = Parser::new(input.chars())
        .next().unwrap().ok().unwrap()
        .eval(scope).ok().unwrap();
    assert_eq!(expr, e_macro![[e_symbol!["a"]],
                              [e_call!["quote",
                                       e_list![e_symbol!["+"],
                                               e_number![1.],
                                               e_call!["unquote",
                                                       e_symbol!["a"]]]]]]);
    let input = "(def f (fn (b) (m b)))";
    let expr = Parser::new(input.chars())
        .next().unwrap().ok().unwrap()
        .eval(scope).ok().unwrap();
    assert_eq!(expr, e_fn![[e_symbol!["b"]],
                           [e_call!["+",
                                    e_number![1.],
                                    e_symbol!["b"]]]]);
}

#[test]
fn test_features() {
    let ref mut scope = Scope::new_std();
    let input = include_str!("./test.ls");
    for expr in Parser::new(input.chars()) {
        expr.unwrap_or_else(|e| panic!("{}", e))
            .eval(scope)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}
