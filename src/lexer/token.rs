use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Pos {
    line: usize,
    col: usize,
}

impl Pos {
    fn new(line: usize, col: usize) -> Pos {
        Pos {
            line: line,
            col: col,
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    start: Pos,
    end: Pos,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
        Span {
            start: Pos::new(start_line, start_col),
            end: Pos::new(end_line, end_col),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number {
        span: Span,
        val: f64
    },
    String {
        span: Span,
        val: String,
    },
    Symbol {
        ns: Option<String>,
        name: String,
        span: Span,
    },
    Keyword {
        ns: Option<String>,
        name: String,
        span: Span,
    },
    ListStart {
        span: Span,
    },
    ListEnd {
        span: Span,
    },
    VecStart {
        span: Span,
    },
    VecEnd {
        span: Span,
    },
    Quote {
        span: Span,
    },
    Unquote {
        span: Span,
    },
    UnquoteSplicing {
        span: Span,
    },
    SyntaxQuote {
        span: Span,
    },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Number { ref val, ref span } => {
                write!(f, "'Number {}' at {}", val, span)
            },
            Token::String { ref val, ref span } => {
                write!(f, r#"'String "{}"' at {}"#, val, span)
            },
            Token::Symbol { ref ns, ref name, ref span } => {
                match *ns {
                    Some(ref ns) => {
                        write!(f, "'Symbol {}/{}' at {}", ns, name, span)
                    },
                    None => {
                        write!(f, "'Symbol {}' at {}", name, span)
                    }
                }
            },
            Token::Keyword { ref ns, ref name, ref span } => {
                match *ns {
                    Some(ref ns) => {
                        write!(f, "'Keyword :{}/{}' at {}", ns, name, span)
                    },
                    None => {
                        write!(f, "'Keyword :{}' at {}", name, span)
                    }
                }
            },
            Token::ListStart { ref span } => {
                write!(f, "'List Start' at {}", span)
            },
            Token::ListEnd { ref span } => {
                write!(f, "'List End' at {}", span)
            },
            Token::VecStart { ref span } => {
                write!(f, "'Vec Start' at {}", span)
            },
            Token::VecEnd { ref span } => {
                write!(f, "'Vec End' at {}", span)
            },
            Token::Quote { ref span } => {
                write!(f, "'Quote' at {}", span)
            },
            Token::Unquote { ref span } => {
                write!(f, "'Unquote' at {}", span)
            },
            Token::UnquoteSplicing { ref span } => {
                write!(f, "'Unquote Splicing' at {}", span)
            },
            Token::SyntaxQuote { ref span } => {
                write!(f, "'Syntax Quote' at {}", span)
            },
        }
    }
}
