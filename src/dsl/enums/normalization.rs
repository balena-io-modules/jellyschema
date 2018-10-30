use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::enums::EnumerationValue;
use crate::dsl::compiler::normalizer::Normalized;

impl Normalize<EnumerationValue> for EnumerationValue {
    fn normalize(&mut self) {
        if self.display_information.title.is_none() {
            self.display_information.title = self.value.clone();
        }
        if self.value.is_none() {
            self.value = self.display_information.title.clone();
        }

    }
}