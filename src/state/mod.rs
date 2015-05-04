mod error;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use ast::Node;
use self::error::EvalError::*;
use self::error::EvalError;

pub type EvalResult = Result<Node, EvalError>;

#[derive(Debug)]
struct Namespace {
    mappings: HashMap<String, Node>,
}

impl Namespace {
    fn new() -> Namespace {
        Namespace { mappings: HashMap::new(), }
    }

    fn get(&self, name: &String) -> Option<&Node> {
        self.mappings.get(name)
    }

    fn insert(&mut self, name: String, node: Node) -> Option<Node> {
        self.mappings.insert(name, node)
    }
}

#[derive(Debug)]
pub struct State<'s> {
    current: String,
    namespaces: HashMap<String, Namespace>,
    parent: Option<&'s State<'s>>,
    id: usize,
}

impl<'s> State<'s> {
    pub fn new(current: String) -> State<'s> {
        let mut current_ns = Namespace::new();
        current_ns.insert("nil".to_string(), e_list![]);
        current_ns.insert("true".to_string(), e_bool!(true));
        current_ns.insert("false".to_string(), e_bool!(false));

        let mut namespaces = HashMap::new();
        namespaces.insert(current.clone(), current_ns);

        State {
            current: current,
            namespaces: namespaces,
            parent: None,
            id: 0,
        }
    }

    pub fn eval(&mut self, node: &Node) -> EvalResult {
        match try!(self.expand(node)) {
            ref symbol_node @ Node::Symbol { .. } => {
                self.eval_symbol(symbol_node)
            },
            ref def_node @ Node::Def { .. } => {
                self.eval_def(def_node)
            },
            ref call_node @ Node::Call { .. } => {
                self.eval_call(call_node)
            },
            ref let_node @ Node::Let { .. } => {
                self.eval_let(let_node)
            },
            other_node => {
                Ok(other_node)
            },
        }
    }

    fn new_chained(parent: &'s State<'s>) -> State<'s> {
        let mut state = State::new(format!("{}_chained", parent.current));
        state.parent = Some(parent);
        state
    }

    fn insert(&mut self, name: String, node: Node) -> Option<Node> {
        self.namespaces
            .get_mut(&self.current)
            .and_then(|scope| scope.insert(name, node))
    }

    fn get(&self, ns: Option<&String>, name: &String) -> Option<&Node> {
        let mut state = self;
        loop {
            let v = state.namespaces
                         .get(ns.unwrap_or(&state.current))
                         .and_then(|scope| scope.get(name));
            if v.is_none() && state.parent.is_some() {
                state = state.parent.unwrap();
            } else {
                return v
            }
        }
    }

    fn get_current(&self) -> &String {
        &self.current
    }

    fn set_current(&mut self, current: String) -> String {
        if !self.namespaces.contains_key(&current) {
            self.namespaces.insert(current.clone(), Namespace::new());
        }
        let old_current = self.current.clone();
        self.current = current;
        old_current
    }

    fn next_id(&mut self) -> usize {
        let next = self.id;
        self.id += 1;
        next
    }

    fn eval_symbol(&mut self, node: &Node) -> EvalResult {
        if let Node::Symbol { ref ns, ref name, .. } = *node {
            self.get(ns.as_ref(), name)
                .map(|e| Ok(e.clone()))
                .unwrap_or(Err(ResolveError(name.clone())))
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::Symbol { .. } => {
                Ok(node.clone())
            },
            Node::List(ref l) => {
                let mut v = vec![];
                for e in l {
                    if e.is_call_of("unquote-splicing") {
                        if let Node::List(ref l) = try!(self.eval(&e)) {
                            for e in l {
                                v.push(e.clone())
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(e.clone()))
                        }
                    } else {
                        v.push(try!(self.eval_quoted(&e)))
                    }
                }
                Ok(Node::List(v))
            },
            _ => {
                self.eval(node)
            },
        }
    }

    fn eval_def(&mut self, node: &Node) -> EvalResult {
        if let Node::Def { ref sym, ref expr } = *node {
            let e = try!(self.eval(expr));
            self.insert(sym.clone(), e.clone());
            Ok(e)
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_let(&mut self, node: &Node) -> EvalResult {
        if let Node::Let { ref bindings, ref body } = *node {
            let ref mut let_state = State::new_chained(self);
            for c in bindings.chunks(2) {
                if let (Some(&Node::Symbol { ref name, .. }), Some(be)) = (c.first(), c.last()) {
                    let evaled_be = try!(let_state.eval(&be));
                    let_state.insert(name.clone(), evaled_be);
                }
            }
            let mut result = e_list![];
            for e in body {
                result = try!(let_state.eval(&e));
            }
            Ok(result)
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref name, .. } = *node {
            match &name[..] {
                "+" => {
                    self.eval_call_builtin_plus(node)
                },
                "-" => {
                    self.eval_call_builtin_minus(node)
                },
                "*" => {
                    self.eval_call_builtin_mul(node)
                },
                "/" => {
                    self.eval_call_builtin_div(node)
                },
                "<" => {
                    self.eval_call_builtin_lt(node)
                },
                ">" => {
                    self.eval_call_builtin_gt(node)
                },
                "=" => {
                    self.eval_call_builtin_eq(node)
                },
                "if" => {
                    self.eval_call_builtin_if(node)
                },
                "quote" => {
                    self.eval_call_builtin_quote(node)
                },
                "syntax-quote" => {
                    self.eval_call_builtin_syntax_quote(node)
                },
                "unquote" => {
                    self.eval_call_builtin_unquote(node)
                },
                "unquote-splicing" => {
                    self.eval_call_builtin_unquote_splicing(node)
                },
                "eval" => {
                    self.eval_call_builtin_eval(node)
                },
                "gensym" => {
                    self.eval_call_builtin_gensym(node)
                },
                "in-ns" => {
                    self.eval_call_builtin_in_ns(node)
                },
                _ => {
                    self.eval_call_custom(node)
                },
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_plus(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            let mut result = 0_f64;
            for a in args {
                if let Node::Number(n) = try!(self.eval(&a)) {
                    result += n;
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Node::Number(result))
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_minus(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut result = if args.len() == 1 { -n } else { n };
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            result -= n
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Node::Number(result))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_mul(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            let mut result = 1_f64;
            for a in args {
                if let Node::Number(n) = try!(self.eval(&a)) {
                    result *= n
                } else {
                    return Err(IncorrectTypeOfArgumentError(a.clone()))
                }
            }
            Ok(Node::Number(result))
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_div(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut result = if args.len() == 1 { 1. / n } else { n };
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            result /= n
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Node::Number(result))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_lt(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            if temp < n {
                                temp = n
                            } else {
                                return Ok(Node::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Node::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_gt(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            if temp > n {
                                temp = n
                            } else {
                                return Ok(Node::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Node::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_eq(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() >= 1 {
                if let Node::Number(n) = try!(self.eval(&args[0])) {
                    let mut temp = n;
                    for a in &args[1..] {
                        if let Node::Number(n) = try!(self.eval(&a)) {
                            if temp == n {
                                temp = n
                            } else {
                                return Ok(Node::Bool(false))
                            }
                        } else {
                            return Err(IncorrectTypeOfArgumentError(a.clone()))
                        }
                    }
                    Ok(Node::Bool(true))
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_if(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 3 {
                if try!(self.eval(&args[0])).as_bool() {
                    self.eval(&args[1])
                } else {
                    self.eval(&args[2])
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_syntax_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                self.eval_quoted(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_unquote(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_unquote_splicing(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                self.eval(&args[0])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_eval(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                self.eval(&args[0]).and_then(|e| self.eval(&e))
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_gensym(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                if let Node::String(ref s) = args[0] {
                    Ok(e_symbol![format!("{}{}", s, self.next_id())])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_builtin_in_ns(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref args, .. } = *node {
            if args.len() == 1 {
                if let Node::Symbol { ref name, .. } = try!(self.eval(&args[0])) {
                    let old_current = self.get_current().clone();
                    self.set_current(name.clone());
                    Ok(e_symbol![old_current])
                } else {
                    Err(IncorrectTypeOfArgumentError(args[0].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn eval_call_custom(&mut self, node: &Node) -> EvalResult {
        if let Node::Call { ref ns, ref name, ref args, .. } = *node {
            let func = try!(self.get(ns.as_ref(), name)
                                .map(|e| Ok(e.clone()))
                                .unwrap_or_else(|| Err(ResolveError(name.clone()))));
            match func {
                Node::Fn { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }

                    let mut e_args = vec![];
                    for a in args {
                        e_args.push(try!(self.eval(a)))
                    }

                    let ref mut fn_state = State::new_chained(self);
                    for (p, a) in params.iter().zip(e_args.iter()) {
                        if let &Node::Symbol { ref name, .. } = p {
                            fn_state.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(fn_state.eval(e));
                    }

                    Ok(result)

                },
                Node::Macro { ref params, ref body } => {
                    if args.len() != params.len() {
                        return Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }

                    let ref mut macro_state = State::new_chained(self);
                    for (p, a) in params.iter().zip(args.iter()) {
                        if let Node::Symbol { ref name, .. } = *p {
                            macro_state.insert(name.clone(), a.clone());
                        } else {
                            return Err(IncorrectTypeOfArgumentError(p.clone()))
                        }
                    }

                    let mut result = e_list![];
                    for e in body {
                        result = try!(macro_state.eval(&e));
                    }

                    Ok(try!(macro_state.expand(&result)))
                },
                _ => {
                    Err(IncorrectTypeOfArgumentError(node.clone()))
                }
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() > 0 {
                if let Node::Symbol { ref name, .. } = l[0] {
                    match &name[..] {
                        "def" => {
                            return self.expand_def(node)
                        },
                        "fn" => {
                            return self.expand_fn(node)
                        },
                        "macro" => {
                            return self.expand_macro(node)
                        },
                        "quote" => {
                            return self.expand_quote(node)
                        },
                        "syntax-quote" => {
                            return self.expand_syntax_quote(node)
                        },
                        "unquote" => {
                            return self.expand_unquote(node)
                        },
                        "unquote-splicing" => {
                            return self.expand_unquote_splicing(node)
                        },
                        "let" => {
                            return self.expand_let(node)
                        },
                        _ => {
                            return self.expand_call(node)
                        }
                    }
                } else {
                    return Err(DispatchError(l[0].clone()))
                }
            }
        }
        Ok(node.clone())
    }

    fn expand_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(node)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_quoted(i)));
                    }
                    Ok(Node::List(v))
                }
            },
            _ => {
                self.expand(node)
            }
        }
    }

    fn expand_syntax_quoted(&mut self, node: &Node) -> EvalResult {
        match *node {
            Node::Symbol { ns: None, ref name } => {
                Ok(e_symbol![self.get_current(), name])
            },
            Node::List(ref l) if l.len() > 0 => {
                if l[0].is_symbol("unquote") || l[0].is_symbol("unquote-splicing") {
                    self.expand(node)
                } else {
                    let mut v = vec![];
                    for i in l {
                        v.push(try!(self.expand_syntax_quoted(i)));
                    }
                    Ok(Node::List(v))
                }
            },
            _ => {
                self.expand(node)
            }
        }
    }

    fn expand_def(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 3 {
                if let Node::Symbol { ref name, .. } = l[1] {
                    Ok(Node::Def {
                        sym: name.clone(),
                        expr: Box::new(try!(self.expand(&l[2]))),
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_fn(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref params) = l[1] {
                    let mut fn_params = vec![];
                    for p in params {
                        fn_params.push(try!(self.expand(p)))
                    }
                    let mut fn_body = vec![];
                    for be in &l[2..] {
                        fn_body.push(try!(self.expand(be)))
                    }
                    Ok(Node::Fn {
                        params: fn_params,
                        body: fn_body,
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_macro(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref params) = l[1] {
                    let mut macro_params = vec![];
                    for p in params {
                        macro_params.push(try!(self.expand(p)))
                    }
                    let mut macro_body = vec![];
                    for be in &l[2..] {
                        macro_body.push(try!(self.expand(be)))
                    }
                    Ok(Node::Macro {
                        params: macro_params,
                        body: macro_body,
                    })
                } else {
                    Err(IncorrectTypeOfArgumentError(l[1].clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(e_call!["quote", try!(self.expand_quoted(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_syntax_quote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(e_call!["syntax-quote", try!(self.expand_syntax_quoted(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_unquote(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(e_call!["unquote", try!(self.expand(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_unquote_splicing(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() == 2 {
                Ok(e_call!["unquote-splicing", try!(self.expand(&l[1]))])
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_let(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if l.len() >= 3 {
                if let Node::Vec(ref v) = l[1] {
                    if v.len() % 2 == 0 {
                        let mut let_bindings = vec![];
                        for c in v.chunks(2) {
                            if let Some(s @ &Node::Symbol {.. }) = c.first() {
                                let_bindings.push(s.clone())
                            } else {
                                return Err(IncorrectTypeOfArgumentError(node.clone()))
                            }
                            if let Some(ref e) = c.last() {
                                let_bindings.push(try!(self.expand(e)))
                            } else {
                                return Err(IncorrectTypeOfArgumentError(node.clone()))
                            }
                        }
                        let mut let_body = vec![];
                        for be in &l[2..] {
                            let_body.push(try!(self.expand(be)))
                        }
                        Ok(Node::Let {
                            bindings: let_bindings,
                            body: let_body,
                        })
                    } else {
                        Err(IncorrectNumberOfArgumentsError(node.clone()))
                    }
                } else {
                    Err(IncorrectTypeOfArgumentError(node.clone()))
                }
            } else {
                Err(IncorrectNumberOfArgumentsError(node.clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }

    fn expand_call(&mut self, node: &Node) -> EvalResult {
        if let Node::List(ref l) = *node {
            if let Node::Symbol { ref ns, ref name, .. } = l[0] {
                let mut args = vec![];
                for a in &l[1..] {
                    args.push(try!(self.expand(a)))
                }
                let call = Node::Call {
                    ns: ns.clone(),
                    name: name.clone(),
                    args: args
                };
                if self.get(ns.as_ref(), name).map_or(false, |e| e.is_macro()) {
                    Ok(try!(self.eval(&call)))
                } else {
                    Ok(call)
                }
            } else {
                Err(IncorrectTypeOfArgumentError(l[0].clone()))
            }
        } else {
            Err(DispatchError(node.clone()))
        }
    }
}
