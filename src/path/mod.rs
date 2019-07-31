//! Path manipulation and value lookup.
//!
//! # Example
//!
//! ```rust
//! use serde_json::json;
//! use jellyschema::path::{Lookup, Path};
//!
//! // Parse JSON path
//! let path = "$['foo'][0]".parse::<Path>().unwrap();
//!
//! // Lookup data
//! let data = json!({"foo": ["bar", "baz"]});
//! assert_eq!(data.lookup(&path), Some(json!("bar")).as_ref());
//! ```
use std::fmt;
use std::slice::Iter;

mod lookup;
mod parser;

pub use lookup::Lookup;

/// Path item - property name, array index.
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum PathItem {
    /// Property name.
    Name(String),
    /// Array index.
    Number(usize),
}

impl From<&str> for PathItem {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<String> for PathItem {
    fn from(s: String) -> Self {
        PathItem::Name(s)
    }
}

impl From<usize> for PathItem {
    fn from(idx: usize) -> Self {
        PathItem::Number(idx)
    }
}

impl fmt::Display for PathItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            PathItem::Name(ref name) => write!(f, "['{}']", name),
            PathItem::Number(idx) => write!(f, "[{}]", idx),
        }
    }
}

/// Path.
#[derive(Debug, Hash, Clone)]
pub struct Path {
    items: Vec<PathItem>,
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Path {
    /// Create empty path.
    pub fn new() -> Path {
        Path { items: vec![] }
    }

    /// Append new path item at the end.
    ///
    /// # Arguments
    ///
    /// * `item` - Path item to append.
    pub fn push<T>(&mut self, item: T)
    where
        T: Into<PathItem>,
    {
        self.items.push(item.into());
    }

    /// Remove the last path item.
    pub fn pop(&mut self) -> Option<PathItem> {
        self.items.pop()
    }

    /// Iterate over path items.
    pub fn iter(&self) -> PathIterator {
        PathIterator::new(self)
    }
}

/// Path items iterator.
pub struct PathIterator<'a> {
    iter: Iter<'a, PathItem>,
}

impl<'a> PathIterator<'a> {
    fn new(path: &Path) -> PathIterator {
        PathIterator {
            iter: path.items.iter(),
        }
    }
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = &'a PathItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl std::str::FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::parse(s)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "$")?;
        for index in &self.items[..] {
            index.fmt(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path_points_to_root() {
        assert_eq!(&format!("{}", Path::new()), "$");
    }

    #[test]
    fn pop_unable_to_pop_root() {
        let mut p = Path::new();
        assert_eq!(p.pop(), None);
        assert_eq!(&format!("{}", p), "$");
    }

    #[test]
    fn push_string() {
        let mut p = Path::new();
        p.push("foo");
        p.push("bar".to_string());
        assert_eq!(&format!("{}", p), "$['foo']['bar']");
    }

    #[test]
    fn push_number() {
        let mut p = Path::new();
        p.push(1);
        p.push(2);
        assert_eq!(&format!("{}", p), "$[1][2]");
    }

    #[test]
    fn push_combined() {
        let mut p = Path::new();
        p.push("foo");
        p.push(2);
        p.push("bar");
        p.push(4);
        assert_eq!(&format!("{}", p), "$['foo'][2]['bar'][4]");
    }

    #[test]
    fn pop() {
        let mut p = Path::new();
        p.push("foo");
        p.push(2);
        p.push("bar");
        p.push(4);
        assert_eq!(&format!("{}", p), "$['foo'][2]['bar'][4]");
        assert_eq!(p.pop(), Some(PathItem::Number(4)));
        assert_eq!(p.pop(), Some(PathItem::Name("bar".to_string())));
        assert_eq!(p.pop(), Some(PathItem::Number(2)));
        assert_eq!(p.pop(), Some(PathItem::Name("foo".to_string())));
        assert_eq!(p.pop(), None);
    }

    #[test]
    fn dotted_path() {
        assert_eq!(
            &format!("{}", "$.foo.0.bar.1".parse::<Path>().unwrap()),
            "$['foo'][0]['bar'][1]"
        );
    }

    #[test]
    fn square_bracket_path() {
        assert_eq!(
            &format!("{}", "$['foo'][0]['bar'][1]".parse::<Path>().unwrap()),
            "$['foo'][0]['bar'][1]"
        );
    }

    #[test]
    fn mixed_path() {
        assert_eq!(
            &format!("{}", "$['foo'].0['bar'].1.2".parse::<Path>().unwrap()),
            "$['foo'][0]['bar'][1][2]"
        );
    }
}
