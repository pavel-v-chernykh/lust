macro_rules! t_number {
    ($val:expr, $span:expr) => (::lexer::Token::Number { val: $val, span: $span, });
}

macro_rules! t_string {
    ($val:expr, $span:expr) => (::lexer::Token::String { val: $val.to_string(), span: $span, });
}

macro_rules! t_symbol {
    ($name:expr, $span:expr) => (::lexer::Token::Symbol {
        ns: None,
        name: $name.to_string(),
        span: $span,
    });
    ($ns:expr, $name:expr, $span:expr) => (::lexer::Token::Symbol {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        span: $span,
    });
}

macro_rules! t_keyword {
    ($name:expr, $span:expr) => (::lexer::Token::Keyword {
        ns: None,
        name: $name.to_string(),
        span: $span,
    });
    ($ns:expr, $name:expr, $span:expr) => (::lexer::Token::Keyword {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        span: $span,
    });
}

macro_rules! t_list_start {
    ($span:expr) => (::lexer::Token::ListStart { span: $span });
}

macro_rules! t_list_end {
    ($span:expr) => (::lexer::Token::ListEnd { span: $span });
}

macro_rules! t_vec_start {
    ($span:expr) => (::lexer::Token::VecStart { span: $span });
}

macro_rules! t_vec_end {
    ($span:expr) => (::lexer::Token::VecEnd { span: $span });
}

macro_rules! t_quote {
    ($span:expr) => (::lexer::Token::Quote { span: $span });
}

macro_rules! t_unquote {
    ($span:expr) => (::lexer::Token::Unquote { span: $span });
}

macro_rules! t_unquote_splicing {
    ($span:expr) => (::lexer::Token::UnquoteSplicing { span: $span });
}

macro_rules! t_syntax_quote {
    ($span:expr) => (::lexer::Token::SyntaxQuote { span: $span });
}

macro_rules! span {
    ($start_line:expr, $start_col:expr, $end_line:expr, $end_col:expr) => (
        ::lexer::Span::new($start_line, $start_col, $end_line, $end_col);
    )
}

macro_rules! e_number {
    ($e:expr) => (::ast::Node::Number($e))
}

macro_rules! e_bool {
    ($e:expr) => (::ast::Node::Bool($e))
}

macro_rules! e_string {
    ($e:expr) => (::ast::Node::String($e.to_string()))
}

macro_rules! e_symbol {
    ($name:expr) => (::ast::Node::Symbol {
        ns: None,
        name: $name.to_string(),
    });
    ($ns:expr, $name:expr) => ($crate::Node::Symbol {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
    });
}

macro_rules! e_keyword {
    ($name:expr) => (::ast::Node::Keyword {
        ns: None,
        name: $name.to_string(),
    });
    ($ns:expr, $name:expr) => (::ast::Node::Keyword {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
    });
}

macro_rules! e_list {
    ($($e:expr),*) => (::ast::Node::List(vec![$($e),*]))
}

macro_rules! e_vec {
    ($($e:expr),*) => (::ast::Node::Vec(vec![$($e),*]))
}

macro_rules! e_def {
    ($sym:expr, $e:expr) => (::ast::Node::Def {
        sym: $sym.to_string(),
        expr: Box::new($e),
    })
}

macro_rules! e_fn {
    ([$($params:expr),*], [$($e:expr),*]) => (::ast::Node::Fn {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_macro {
    ([$($params:expr),*], [$($e:expr),*]) => (::ast::Node::Macro {
        params: vec![$($params),*],
        body: vec![$($e),*],
    })
}

macro_rules! e_call {
    ($name:expr, $($arg:expr),*) => (::ast::Node::Call {
        ns: None,
        name: $name.to_string(),
        args: vec![$($arg),*],
    });
    ($ns:expr, $name:expr, $($arg:expr),*) => (::ast::Node::Call {
        ns: Some($ns.to_string()),
        name: $name.to_string(),
        args: vec![$($arg),*],
    });
}

macro_rules! e_let {
    ([$($s:expr, $e:expr),*], $($body:expr),*) => (::ast::Node::Let {
        bindings: vec![$($s, $e),*],
        body: vec![$($body),*],
    })
}
