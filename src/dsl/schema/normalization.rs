use crate::dsl::compiler::normalizer::Normalize;
use crate::dsl::compiler::normalizer::Normalized;
use crate::dsl::schema::SourceSchema;

impl Normalize<SourceSchema> for SourceSchema {
    fn normalize(self) -> Normalized<SourceSchema> {
        Normalized::with(self)
    }
}
