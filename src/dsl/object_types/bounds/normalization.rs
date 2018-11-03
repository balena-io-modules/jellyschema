use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::StringObjectBounds;

impl Normalize for StringObjectBounds {
    fn normalize(&mut self) {
        for value in self.possible_values.iter_mut() {
            value.normalize()
        }
    }
}

impl Normalize for EnumerationValue {
    fn normalize(&mut self) {
        if self.display_information.title.is_none() {
            self.display_information.title = Some(self.value.clone());
        }
    }
}
