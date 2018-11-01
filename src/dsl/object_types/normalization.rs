use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use crate::dsl::object_types::TypeDefinition;

impl Normalize for TypeDefinition {
    fn normalize(&mut self) {
        for r#type in self.r#type.iter_mut() {
            match r#type {
                ObjectType::Required(object_type) => object_type.normalize(),
                ObjectType::Optional(object_type) => object_type.normalize(),
            }
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
            _ => {}
        }
    }
}
