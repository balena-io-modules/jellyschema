use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::IntegerObjectBounds;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;

impl Normalize for ObjectType {
    fn normalize(&mut self) {
        match self {
            ObjectType::Required(object_type) => object_type.normalize(),
            ObjectType::Optional(object_type) => object_type.normalize(),
        }
    }
}

impl Normalize for RawObjectType {
    fn normalize(&mut self) {
        match self {
            RawObjectType::String(object_bounds) => {
                for bounds in object_bounds {
                    bounds.normalize()
                }
            }
            RawObjectType::Integer(object_bounds) => {
                for bounds in object_bounds {
                    bounds.normalize()
                }
            }
            _ => {}
        }
    }
}

impl Normalize for IntegerObjectBounds {
    fn normalize(&mut self) {
        // TODO: this being empty may indicate that we don't need normalization step, or it needs to be split
    }
}
