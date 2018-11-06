use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::StringObjectBounds;

impl Normalize for StringObjectBounds {
    fn normalize(&mut self) {
        match self {
            StringObjectBounds::PossibleValues(values) => {
                for value in values.iter_mut() {
                    value.normalize()
                }
            }
            StringObjectBounds::Pattern(_) => {}
            StringObjectBounds::Length(_) => {}
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
