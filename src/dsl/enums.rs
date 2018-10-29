use crate::dsl::types::ObjectType;
use crate::dsl::types::ObjectValue;

#[derive(Clone, Debug)]
pub struct EnumerationValues {
    value_type: ObjectType,
    possible_value: Vec<ObjectValue>,
}
