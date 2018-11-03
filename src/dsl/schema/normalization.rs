use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
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
        match &mut self.property.type_information {
            Some(spec) => spec.normalize(),
            None => {}
        }

        if self.property.type_information.is_none() {
            self.property.type_information = Some(ObjectType::Required(RawObjectType::Object));
        }
    }
}
