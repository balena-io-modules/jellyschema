use serde_json::Value;

use crate::path::{Path, PathItem};

/// Value lookup.
pub trait Lookup {
    /// Lookup value with the given path.
    fn lookup(&self, path: &Path) -> Option<&Value>;
}

impl Lookup for Value {
    fn lookup(&self, path: &Path) -> Option<&Value> {
        let mut result = self;

        for index in path.iter() {
            let value = match index {
                PathItem::Number(idx) => result.as_array().and_then(|x| x.get(*idx)),
                PathItem::Name(name) => result.as_object().and_then(|x| x.get(name)),
            };

            if let Some(value) = value {
                result = value;
            } else {
                return None;
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn lookup_array() {
        let json = json!(["first", "second", "third"]);
        assert_eq!(json.lookup(&"$[0]".parse().unwrap()), Some(&json!("first")));
        assert_eq!(json.lookup(&"$[1]".parse().unwrap()), Some(&json!("second")));
        assert_eq!(json.lookup(&"$[2]".parse().unwrap()), Some(&json!("third")));
        assert_eq!(json.lookup(&"$[3]".parse().unwrap()), None);
    }

    #[test]
    fn lookup_object() {
        let json = json!({
            "first": "Robert",
            "last": "Vojta"
        });
        assert_eq!(json.lookup(&"$['first']".parse().unwrap()), Some(&json!("Robert")));
        assert_eq!(json.lookup(&"$['last']".parse().unwrap()), Some(&json!("Vojta")));
        assert_eq!(json.lookup(&"$['foo']".parse().unwrap()), None);
    }

    #[test]
    fn lookup_nested() {
        let json = json!([
            {
                "type": "foo",
                "values": []
            },
            {
                "type": "bar",
                "values": [0, 1, 2, 3, 4]
            }
        ]);
        assert_eq!(json.lookup(&"$[1]['type']".parse().unwrap()), Some(&json!("bar")));
        assert_eq!(json.lookup(&"$[1]['values'][3]".parse().unwrap()), Some(&json!(3)));
    }
}
