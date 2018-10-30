use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::TypeDefinition;

impl Normalize<TypeDefinition> for TypeDefinition {
    fn normalize(&mut self) {
        for enumeration_values in &mut self.enumeration_values {
            for enumeration_value in &mut enumeration_values.possible_values {
                enumeration_value.normalize()
            }
        }
    }
}
