use crate::point::Point;
use std::collections::HashMap;

pub(crate) struct Scope {
    variables: HashMap<String, Point>,
}

impl Scope {
    pub(crate) fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub(crate) fn resolve(&self, name: &str) -> Point {
        if let Some(point) = self.variables.get(name) {
            *point
        } else {
            panic!("unresolved variable: {}", name);
        }
    }

    pub(crate) fn assign(&mut self, name: &str, value: Point) {
        self.variables.insert(name.to_owned(), value);
    }
}
