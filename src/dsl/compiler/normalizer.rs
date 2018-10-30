use crate::dsl::compiler::validator::Validated;
use crate::dsl::schema::SourceSchema;

pub struct Normalized<T> {
    normalized: T,
}

impl<T> Normalized<T> {
    pub fn with(value: T) -> Self {
        Normalized { normalized: value }
    }

    pub fn normalized(self) -> T {
        self.normalized
    }
}

pub trait Normalize<T> {
    fn normalize(self) -> Normalized<T>;
}

pub fn normalize(validated_schema: Validated<SourceSchema>) -> Normalized<SourceSchema> {
    validated_schema.validated().normalize()
}
