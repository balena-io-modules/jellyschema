use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::object_types::bounds::deserialization::deserialize_array_object_bounds;
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_default_value;
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_integer_bounds;
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_string_object_bounds;
use crate::dsl::schema::object_types::bounds::IntegerValueConditionObjectBounds;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_integer_bounds_with_defaults;
use crate::dsl::schema::object_types::bounds::IntegerBound;
use crate::dsl::schema::object_types::ObjectTypeData;

pub fn deserialize_object_type<E>(mapping: &Mapping) -> Result<Option<Vec<ObjectType>>, E>
where
    E: Error,
{
    let type_key = Value::from("type");
    Ok(mapping.get(&type_key).map_or(Ok(None), |value| match value {
        Value::String(string) => Ok(Some(vec![deserialize_individual_type_definition(string, &mapping)?])),
        Value::Sequence(sequence) => Ok(Some(deserialize_array_of_types(sequence, mapping)?)),
        _ => Err(Error::custom(format!(
            "cannot recognize type definition format `{:#?}`",
            value
        ))),
    })?)
}

fn deserialize_array_of_types<E>(sequence: &[Value], mapping: &Mapping) -> Result<Vec<ObjectType>, E>
where
    E: Error,
{
    sequence
        .iter()
        .map(|type_definition| match type_definition {
            Value::String(string) => Ok(deserialize_individual_type_definition(string, mapping)?),
            _ => Err(Error::custom("type definition needs to be a string")),
        })
        .collect()
}

fn deserialize_individual_type_definition<E>(definition: &str, mapping: &Mapping) -> Result<ObjectType, E>
where
    E: Error,
{
    let mut type_name = definition.trim().to_lowercase();
    let default_value = deserialize_default_value(mapping)?;
    Ok(if type_name.ends_with('?') {
        type_name.remove(type_name.len() - 1);
        let raw_type = RawObjectType::from(&type_name, &mapping)?;
        let type_data = ObjectTypeData::with_raw_type_and_default_value(raw_type, default_value);
        ObjectType::Optional(type_data)
    } else {
        let raw_type = RawObjectType::from(&type_name, &mapping)?;
        let type_data = ObjectTypeData::with_raw_type_and_default_value(raw_type, default_value);
        ObjectType::Required(type_data)
    })
}

pub fn deserialize_integer<E>(name: &str, mapping: &Mapping) -> Result<Option<i64>, E>
where
    E: Error,
{
    let value = mapping.get(&Value::from(name));
    value.map_or(Ok(None), |value| {
        Ok(Some(value.as_i64().ok_or_else(|| {
            Error::custom(format!("cannot deserialize {:#?} as integer", value))
        })?))
    })
}

impl RawObjectType {
    fn from<E>(value: &str, mapping: &Mapping) -> Result<Self, E>
    where
        E: Error,
    {
        let object_type = match value {
            "object" => RawObjectType::Object,
            "string" => RawObjectType::String(deserialize_string_object_bounds(mapping)?),
            "text" => RawObjectType::Text(deserialize_string_object_bounds(mapping)?),
            "integer" => RawObjectType::Integer(deserialize_integer_bounds(mapping)?),
            "number" => RawObjectType::Number(deserialize_integer_bounds(mapping)?),
            "port" => {
                let defaults = IntegerValueConditionObjectBounds {
                    minimum: Some(IntegerBound::Inclusive(0)),
                    maximum: Some(IntegerBound::Inclusive(65535)),
                    multiple_of: None,
                };
                let defaults = IntegerObjectBounds::Conditions(defaults);
                RawObjectType::Port(Some(deserialize_integer_bounds_with_defaults(defaults, mapping)?))
            }
            "password" => RawObjectType::Password(deserialize_string_object_bounds(mapping)?),
            "boolean" => RawObjectType::Boolean(),
            "array" => RawObjectType::Array(Box::new(deserialize_array_object_bounds(mapping)?)),
            "hostname" => RawObjectType::Hostname,
            "datetime" => RawObjectType::Datetime,
            "date" => RawObjectType::Date,
            "time" => RawObjectType::Time,
            "email" => RawObjectType::Email,
            "ipv4" => RawObjectType::IPV4,
            "ipv6" => RawObjectType::IPV6,
            "uri" => RawObjectType::URI,
            _ => RawObjectType::Text(deserialize_string_object_bounds(mapping)?),
        };
        Ok(object_type)
    }
}
