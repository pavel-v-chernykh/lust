use super::{Lexer, LexerResult, LexerError};

#[test]
fn test_read_nil() {
    let sym_name = "nil";
    let t_sym = t_symbol!(sym_name, span!(1, 1, 1, 4));
    let mut lexer = Lexer::new(sym_name.chars());
    assert_eq!(Some(Ok(t_sym)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_integer_as_float() {
    let mut lexer = Lexer::new("64".chars());
    let t_num = t_number!(64_f64, span!(1, 1, 1, 3));
    assert_eq!(Some(Ok(t_num)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_float() {
    let mut lexer = Lexer::new("64.5".chars());
    let t_num = t_number!(64.5, span!(1, 1, 1, 5));
    assert_eq!(Some(Ok(t_num)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_string() {
    let s = "rust is beautiful";
    let actual_input = format!(r#""{}""#, s);
    let mut lexer = Lexer::new(actual_input.chars());
    let t_str = t_string!(s, span!(1, 1, 1, 20));
    assert_eq!(Some(Ok(t_str)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_symbol() {
    let sym_name = "my-symbol";
    let mut lexer = Lexer::new(sym_name.chars());
    let t_sym = t_symbol!(sym_name, span!(1, 1, 1, 10));
    assert_eq!(Some(Ok(t_sym)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_ns_qualified_symbol() {
    let ns_name = "my-ns";
    let sym_name = "my-symbol";
    let ns_quilified_sym_name = format!("{}/{}", ns_name, sym_name);
    let mut lexer = Lexer::new(ns_quilified_sym_name.chars());
    let t_sym = t_symbol!(ns_name, sym_name, span!(1, 1, 1, 16));
    assert_eq!(Some(Ok(t_sym)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_ns_qualified_div() {
    let ns_name = "my-ns";
    let sym_name = "/";
    let ns_quilified_sym_name = format!("{}/{}", ns_name, sym_name);
    let mut lexer = Lexer::new(ns_quilified_sym_name.chars());
    let t_sym = t_symbol!(ns_name, sym_name, span!(1, 1, 1, 8));
    assert_eq!(Some(Ok(t_sym)), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_incorrect_symbol_starting_with_digit() {
    let sym_name = "6-my-incorrect-symbol";
    let mut lexer = Lexer::new(sym_name.chars());
    let expected_result = Some(Err(LexerError::new(1, 2)));
    assert_eq!(expected_result, lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_keyword() {
    let mut lexer = Lexer::new(":my-keyword".chars());
    assert_eq!(Some(Ok(t_keyword!("my-keyword", span!(1, 1, 1, 12)))), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_ns_qualified_keyword() {
    let mut lexer = Lexer::new(":my-ns/my-keyword".chars());
    assert_eq!(Some(Ok(t_keyword!("my-ns", "my-keyword", span!(1, 1, 1, 18)))), lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_explicitly_positive_number() {
    let mut lexer = Lexer::new("+1".chars());
    let t_num = t_number!(1_f64, span!(1, 1, 1, 3));
    let expected_result = Some(Ok(t_num));
    assert_eq!(expected_result, lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_explicitly_negative_number() {
    let mut lexer = Lexer::new("-1".chars());
    let t_num = t_number!(-1_f64, span!(1, 1, 1, 3));
    let expected_result = Some(Ok(t_num));
    assert_eq!(expected_result, lexer.next());
    assert_eq!(None, lexer.next());
}

#[test]
fn test_read_dense_expression() {
    let lexer = Lexer::new("(def a 1)".chars());
    let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                               Ok(t_symbol!("def", span!(1, 2, 1, 5))),
                               Ok(t_symbol!("a", span!(1, 6, 1, 7))),
                               Ok(t_number!(1_f64, span!(1, 8, 1, 9))),
                               Ok(t_list_end!(span!(1, 9, 1, 10)))];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_sparse_expression_with_comments() {
    let lexer = Lexer::new(" ( ;; comments\n \
                            def a ;; comments \n\
                            1) ;; comments \n\
                            ;; comments   \n".chars());
    let expected_result = vec![Ok(t_list_start!(span!(1, 2, 1, 3))),
                               Ok(t_symbol!("def", span!(2, 3, 2, 6))),
                               Ok(t_symbol!("a", span!(2, 7, 2, 8))),
                               Ok(t_number!(1_f64, span!(3, 2, 3, 3))),
                               Ok(t_list_end!(span!(3, 3, 3, 4)))];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_nested_list_expressions() {
    let lexer = Lexer::new("(def a (+ 1 1))".chars());
    let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                               Ok(t_symbol!("def", span!(1, 2, 1, 5))),
                               Ok(t_symbol!("a", span!(1, 6, 1, 7))),
                               Ok(t_list_start!(span!(1, 8, 1, 9))),
                               Ok(t_symbol!("+", span!(1, 9, 1, 10))),
                               Ok(t_number!(1_f64, span!(1, 11, 1, 12))),
                               Ok(t_number!(1_f64, span!(1, 13, 1, 14))),
                               Ok(t_list_end!(span!(1, 14, 1, 15))),
                               Ok(t_list_end!(span!(1, 15, 1, 16)))];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_vec_of_expressions() {
    let lexer = Lexer::new("[a 1 (1 [3]) +]".chars());
    let expected_result = vec![Ok(t_vec_start!(span!(1, 1, 1, 2))),
                               Ok(t_symbol!("a", span!(1, 2, 1,3))),
                               Ok(t_number!(1., span!(1, 4, 1, 5))),
                               Ok(t_list_start!(span!(1, 6, 1, 7))),
                               Ok(t_number!(1., span!(1, 7, 1, 8))),
                               Ok(t_vec_start!(span!(1, 9, 1, 10))),
                               Ok(t_number!(3., span!(1, 10, 1, 11))),
                               Ok(t_vec_end!(span!(1, 11, 1, 12))),
                               Ok(t_list_end!(span!(1, 12, 1, 13))),
                               Ok(t_symbol!("+", span!(1, 14, 1, 15))),
                               Ok(t_vec_end!(span!(1, 15, 1, 16)))];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_list_of_symbols() {
    let lexer = Lexer::new("(+ - / * % > < = a b c z A X Y Z)".chars());
    let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                               Ok(t_symbol!("+", span!(1, 2, 1, 3))),
                               Ok(t_symbol!("-", span!(1, 4, 1, 5))),
                               Ok(t_symbol!("/", span!(1, 6, 1, 7))),
                               Ok(t_symbol!("*", span!(1, 8, 1, 9))),
                               Ok(t_symbol!("%", span!(1, 10, 1, 11))),
                               Ok(t_symbol!(">", span!(1, 12, 1, 13))),
                               Ok(t_symbol!("<", span!(1, 14, 1, 15))),
                               Ok(t_symbol!("=", span!(1, 16, 1, 17))),
                               Ok(t_symbol!("a", span!(1, 18, 1, 19))),
                               Ok(t_symbol!("b", span!(1, 20, 1, 21))),
                               Ok(t_symbol!("c", span!(1, 22, 1, 23))),
                               Ok(t_symbol!("z", span!(1, 24, 1, 25))),
                               Ok(t_symbol!("A", span!(1, 26, 1, 27))),
                               Ok(t_symbol!("X", span!(1, 28, 1, 29))),
                               Ok(t_symbol!("Y", span!(1, 30, 1, 31))),
                               Ok(t_symbol!("Z", span!(1, 32, 1, 33))),
                               Ok(t_list_end!(span!(1, 33, 1, 34)))];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_quoted_symbol() {
    let lexer = Lexer::new("'a".chars());
    let expected_result = vec![Ok(t_quote![span![1, 1, 1, 2]]),
                               Ok(t_symbol!["a", span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_quoted_number() {
    let lexer = Lexer::new("'1".chars());
    let expected_result = vec![Ok(t_quote![span![1, 1, 1, 2]]),
                               Ok(t_number![1., span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_quoted_list() {
    let lexer = Lexer::new("'()".chars());
    let expected_result = vec![Ok(t_quote![span![1, 1, 1, 2]]),
                               Ok(t_list_start![span![1, 2, 1, 3]]),
                               Ok(t_list_end![span![1, 3, 1, 4]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_quoted_symbol_inside_list() {
    let lexer = Lexer::new("(+ 1 'a)".chars());
    let expected_result = vec![Ok(t_list_start![span![1, 1, 1, 2]]),
                               Ok(t_symbol!["+", span![1, 2, 1, 3]]),
                               Ok(t_number![1., span![1, 4, 1, 5]]),
                               Ok(t_quote![span![1, 6, 1, 7]]),
                               Ok(t_symbol!["a", span![1, 7, 1, 8]]),
                               Ok(t_list_end![span![1, 8, 1, 9]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_unquoted_symbol() {
    let lexer = Lexer::new("~a".chars());
    let expected_result = vec![Ok(t_unquote![span![1, 1, 1, 2]]),
                               Ok(t_symbol!["a", span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_unquoted_number() {
    let lexer = Lexer::new("~1".chars());
    let expected_result = vec![Ok(t_unquote![span![1, 1, 1, 2]]),
                               Ok(t_number![1., span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_unquoted_list() {
    let lexer = Lexer::new("~()".chars());
    let expected_result = vec![Ok(t_unquote![span![1, 1, 1, 2]]),
                               Ok(t_list_start![span![1, 2, 1, 3]]),
                               Ok(t_list_end![span![1, 3, 1, 4]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_unquoted_symbol_inside_list() {
    let lexer = Lexer::new("'(+ 1 ~a)".chars());
    let expected_result = vec![Ok(t_quote![span![1, 1, 1, 2]]),
                               Ok(t_list_start![span![1, 2, 1, 3]]),
                               Ok(t_symbol!["+", span![1, 3, 1, 4]]),
                               Ok(t_number![1., span![1, 5, 1, 6]]),
                               Ok(t_unquote![span![1, 7, 1, 8]]),
                               Ok(t_symbol!["a", span![1, 8, 1, 9]]),
                               Ok(t_list_end![span![1, 9, 1, 10]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_unquoted_splicing_list() {
    let lexer = Lexer::new("~@()".chars());
    let expected_result = vec![Ok(t_unquote_splicing![span![1, 1, 1, 3]]),
                               Ok(t_list_start![span![1, 3, 1, 4]]),
                               Ok(t_list_end![span![1, 4, 1, 5]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_syntax_quoted_symbol() {
    let lexer = Lexer::new("`a".chars());
    let expected_result = vec![Ok(t_syntax_quote![span![1, 1, 1, 2]]),
                               Ok(t_symbol!["a", span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_syntax_quoted_number() {
    let lexer = Lexer::new("`1".chars());
    let expected_result = vec![Ok(t_syntax_quote![span![1, 1, 1, 2]]),
                               Ok(t_number![1., span![1, 2, 1, 3]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}

#[test]
fn test_read_syntax_quoted_list() {
    let lexer = Lexer::new("`(+ a 1)".chars());
    let expected_result = vec![Ok(t_syntax_quote![span![1, 1, 1, 2]]),
                               Ok(t_list_start![span![1, 2, 1, 3]]),
                               Ok(t_symbol!["+", span![1, 3, 1, 4]]),
                               Ok(t_symbol!["a", span![1, 5, 1, 6]]),
                               Ok(t_number![1., span![1, 7, 1, 8]]),
                               Ok(t_list_end![span![1, 8, 1, 9]])];
    assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
}
