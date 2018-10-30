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

pub trait Normalize {
    fn normalize(&mut self);
}

pub fn normalize(schema: SourceSchema) -> Normalized<SourceSchema> {
    let mut schema = schema;
    schema.normalize();
    Normalized::from(schema)
}
