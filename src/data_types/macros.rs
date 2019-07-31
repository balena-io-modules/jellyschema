// TODO Replace with procedural macros

macro_rules! data_type {
    // Data type with just name & schema.yaml
    ($name:expr) => {
        _impl_custom_data_type!($name, validator: None, generator: None);
    };

    ($name:expr, validator: $validator:ident) => {
        _impl_custom_data_type_validator!($validator);
        _impl_custom_data_type!(
            $name,
            validator: Some(Box::new(CustomDataTypeValidator)),
            generator: None
        );
    };

    ($name:expr, generator: $generator:ident) => {
        _impl_custom_data_type_generator!($generator);
        _impl_custom_data_type!(
            $name,
            validator: None,
            generator: Some(Box::new(CustomDataTypeGenerate))
        );
    };

    ($name:expr, validator: $validator:ident, generator: $generator:ident) => {
        _impl_custom_data_type_validator!($validator);
        _impl_custom_data_type_generator!($generator);
        _impl_custom_data_type!(
            $name,
            validator: Some(Box::new(CustomDataTypeValidator)),
            generator: Some(Box::new(CustomDataTypeGenerator))
        );
    };
}

macro_rules! _impl_custom_data_type_validator {
    ($validator:ident) => {
        struct CustomDataTypeValidator;

        impl crate::validators::Validator for CustomDataTypeValidator {
            fn validate(
                &self,
                item: &serde_json::Value,
                ctx: &crate::WalkContext,
            ) -> crate::validators::ValidationState {
                $validator(item, ctx)
            }
        }
    };
}

macro_rules! _impl_custom_data_type_generator {
    ($generator:ident) => {
        struct CustomDataTypeGenerator;

        impl crate::generator::Generator for CustomDataTypeGenerator {
            fn generate(&self, schema: &crate::Schema) -> crate::generator::Result<serde_json::Value> {
                $generator(schema)
            }
        }
    };
}

macro_rules! _impl_custom_data_type {
    ($name:expr, validator: $validator:expr, generator: $generator:expr) => {
        pub const NAME: &str = $name;

        pub struct CustomDataType;

        impl crate::data_types::DataType for CustomDataType {
            fn schema(&self) -> &str {
                include_str!("schema.yaml")
            }

            fn validator(&self) -> Option<crate::validators::BoxedValidator> {
                $validator
            }

            fn generator(&self) -> Option<crate::generator::BoxedGenerator> {
                $generator
            }
        }
    };
}

macro_rules! data_types {
    ( $( $module:ident ),* ) => {

//        $(
//            mod $module;
//        )*

        pub(crate) fn default() -> crate::data_types::DataTypeMap {
            let mut m = crate::data_types::DataTypeMap::new();
            $(
                m.insert($module::NAME.to_string(), Box::new($module::CustomDataType) as Box<dyn crate::data_types::DataType>);
            )*
            m
        }
    };
}
