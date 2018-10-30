use crate::dsl::compiler::validator::Validated;
use crate::dsl::schema::SourceSchema;

pub struct Normalized<T> {
    normalized: T,
}

impl<T> Normalized<T> {
    pub fn from(value: T) -> Self {
        Normalized { normalized: value }
    }

    pub fn normalized(self) -> T {
        self.normalized
    }
}

pub trait Normalize<T> {
    fn normalize(&mut self);
}

pub fn normalize(validated_schema: Validated<SourceSchema>) -> Normalized<SourceSchema> {
    let mut schema = validated_schema.validated();
    schema.normalize();
    Normalized::from(schema)
}
