use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::compiler::normalizer::Normalized;
use crate::dsl::schema::SourceSchema;

impl Normalize<SourceSchema> for SourceSchema {
    fn normalize(&mut self) {
        for list in &mut self.property_list {
            for property in &mut list.entries {
                for enumeration_values in &mut property.property.type_information.enumeration_values {
                    for enumeration_value in &mut enumeration_values.possible_values {
                        enumeration_value.normalize()
                    }
                }
            }
        }
    }

}
