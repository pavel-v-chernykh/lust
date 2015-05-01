#[cfg(test)]
mod tests;

use std::collections::HashMap;
use ast::Expr;

type Alias = (String, String);

#[derive(Debug)]
struct Namespace {
    mappings: HashMap<String, Expr>,
    aliases: HashMap<String, Alias>,
}

impl Namespace {
    fn new() -> Namespace {
        Namespace {
            mappings: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    fn get(&self, name: &String) -> Option<&Expr> {
        self.mappings.get(name)
    }

    fn insert(&mut self, name: String, expr: Expr) -> Option<Expr> {
        self.mappings.insert(name, expr)
    }

    fn get_alias(&self, alias: &String) -> Option<&Alias> {
        self.aliases.get(alias)
    }

    fn insert_alias(&mut self, alias: String, ns: String, name: String) -> Option<Alias> {
        self.aliases.insert(alias, (ns, name))
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

    pub fn new_chained(parent: &'s State<'s>) -> State<'s> {
        let mut state = State::new(format!("{}_chained", parent.current));
        state.parent = Some(parent);
        state
    }

    pub fn insert(&mut self, name: String, expr: Expr) -> Option<Expr> {
        self.namespaces
            .get_mut(&self.current)
            .and_then(|scope| scope.insert(name, expr))
    }

    pub fn get(&self, ns: Option<&String>, name: &String) -> Option<&Expr> {
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

    pub fn get_current(&self) -> &String {
        &self.current
    }

    pub fn set_current(&mut self, current: String) -> String {
        if !self.namespaces.contains_key(&current) {
            self.namespaces.insert(current.clone(), Namespace::new());
        }
        let old_current = self.current.clone();
        self.current = current;
        old_current
    }

    pub fn next_id(&mut self) -> usize {
        let next = self.id;
        self.id += 1;
        next
    }
}