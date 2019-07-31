use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::path::{Lookup, Path};
use crate::WalkContext;

pub struct UniqueItems {
    pub unique: bool,
    pub paths: Option<Vec<Path>>,
}

impl Validator for UniqueItems {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let array = validator_non_strict_as!(data.as_array());

        let validate_without_paths = |_: &Value, unique: bool, ctx: &WalkContext| -> ValidationState {
            if !unique {
                return ValidationState::new();
            }

            for (idx_to_validate, item_to_validate) in array.iter().enumerate() {
                for (idx, item) in array.iter().enumerate() {
                    if idx == idx_to_validate {
                        continue;
                    }

                    if item_to_validate == item {
                        return ctx
                            .push(idx_to_validate)
                            .validation_error("uniqueItems", "item is not unique")
                            .into();
                    }
                }
            }

            ValidationState::new()
        };

        let validate_with_paths = |_: &Value, paths: &[Path], ctx: &WalkContext| -> ValidationState {
            for path in paths {
                for (idx_to_validate, item_to_validate) in array.iter().enumerate() {
                    for (idx, item) in array.iter().enumerate() {
                        if idx == idx_to_validate {
                            continue;
                        }

                        let value_to_validate = item_to_validate.lookup(path);
                        let value = item.lookup(path);

                        if value_to_validate == value {
                            return ctx
                                .push(idx_to_validate)
                                .validation_error("uniqueItems", "item is not unique")
                                .into();
                        }
                    }
                }
            }

            ValidationState::new()
        };

        if let Some(paths) = self.paths.as_ref() {
            validate_with_paths(data, paths, ctx)
        } else {
            validate_without_paths(data, self.unique, ctx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn unique_strings() {
        let ui = UniqueItems {
            unique: true,
            paths: None,
        };

        let valid = json!(["foo", "bar", "baz"]);
        let invalid = json!(["foo", "bar", "foo"]);

        assert!(ui.validate(&valid, &WalkContext::new()).is_valid());
        assert!(ui.validate(&invalid, &WalkContext::new()).is_invalid());
    }

    #[test]
    fn not_unique_strings() {
        let ui = UniqueItems {
            unique: false,
            paths: None,
        };

        let valid = json!(["foo", "bar", "foo"]);

        assert!(ui.validate(&valid, &WalkContext::new()).is_valid());
    }

    #[test]
    fn unique_objects() {
        let ui = UniqueItems {
            unique: true,
            paths: None,
        };

        let valid = json!([
            {
              "type": "foo"
            },
            {
              "type": "bar"
            },
            {
              "type": "baz"
            }
        ]);
        let invalid = json!([
            {
              "type": "foo"
            },
            {
              "type": "bar"
            },
            {
              "type": "bar"
            }
        ]);

        assert!(ui.validate(&valid, &WalkContext::new()).is_valid());
        assert!(ui.validate(&invalid, &WalkContext::new()).is_invalid());
    }

    #[test]
    fn not_unique_objects() {
        let ui = UniqueItems {
            unique: false,
            paths: None,
        };

        let valid = json!([
            {
              "type": "foo"
            },
            {
              "type": "baz"
            },
            {
              "type": "baz"
            }
        ]);

        assert!(ui.validate(&valid, &WalkContext::new()).is_valid());
    }

    #[test]
    fn unique_object_paths() {
        let ui = UniqueItems {
            unique: true,
            paths: Some(vec!["$.person.first".parse().unwrap()]),
        };

        let valid = json!([
            {
              "person": {
                "first": "foo",
                "last": "last"
              }
            },
            {
              "person": {
                "first": "bar",
                "last": "last"
              }
            }
        ]);
        let invalid = json!([
            {
              "person": {
                "first": "first",
                "last": "last"
              }
            },
            {
              "person": {
                "first": "first",
                "last": "last"
              }
            }
        ]);

        assert!(ui.validate(&valid, &WalkContext::new()).is_valid());
        assert!(ui.validate(&invalid, &WalkContext::new()).is_invalid());
    }
}
