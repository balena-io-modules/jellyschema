use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::schema::PropertyEntry;
use crate::dsl::schema::SourceSchema;

impl Normalize for SourceSchema {
    fn normalize(&mut self) {
        for list in &mut self.property_list {
            for property in &mut list.entries {
                property.normalize()
            }
        }
    }
}

impl Normalize for PropertyEntry {
    fn normalize(&mut self) {
        match &mut self.property.types {
            Some(spec) => {
                for def in spec {
                    def.normalize();
                }
            }
            None => {}
        }
    }
}
