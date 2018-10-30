use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::schema::PropertyEntry;
use crate::dsl::schema::SourceSchema;

impl Normalize<SourceSchema> for SourceSchema {
    fn normalize(&mut self) {
        for list in &mut self.property_list {
            for property in &mut list.entries {
                property.normalize()
            }
        }
    }
}

impl Normalize<PropertyEntry> for PropertyEntry {
    fn normalize(&mut self) {
        self.property.type_information.normalize();
    }
}
