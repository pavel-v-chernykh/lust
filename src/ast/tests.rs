#[test]
fn test_format_list_with_nested_list_and_atoms() {
    let actual_input = e_def!["a", e_call!["+", e_number!(1_f64), e_number!(2_f64)]];
    let actual_result = format!("{}", actual_input);
    let expected_result = "(def a (+ 1 2))";
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_format_call_expr_without_args() {
    assert_eq!(format!("{}", e_call!["+",]), "(+)");
}
