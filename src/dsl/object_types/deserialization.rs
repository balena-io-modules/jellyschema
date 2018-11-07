use crate::dsl::object_types::bounds::deserialization::deserialize_boolean_object_bounds;
use crate::dsl::object_types::bounds::deserialization::deserialize_integer_bounds;
use crate::dsl::object_types::bounds::deserialization::deserialize_string_object_bounds;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

pub fn deserialize_object_type<E>(mapping: &Mapping) -> Result<Option<ObjectType>, E>
where
    E: Error,
{
    let type_key = Value::from("type");
    Ok(mapping.get(&type_key).map_or(Ok(None), |value| match value {
        Value::String(string) => Ok(Some(deserialize_individual_type_definition(string, &mapping)?)),
        _ => Err(Error::custom(format!(
            "cannot recognize type definition format `{:#?}`",
            value
        ))),
    })?)
}

fn deserialize_individual_type_definition<E>(definition: &str, mapping: &Mapping) -> Result<ObjectType, E>
where
    E: Error,
{
    let mut type_name = definition.trim().to_lowercase();
    Ok(if type_name.ends_with('?') {
        type_name.remove(type_name.len() - 1);
        ObjectType::Optional(RawObjectType::from(&type_name, &mapping)?)
    } else {
        ObjectType::Required(RawObjectType::from(&type_name, &mapping)?)
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
            "hostname" => RawObjectType::Hostname,
            "integer" => RawObjectType::Integer(deserialize_integer_bounds(mapping)?),
            "password" => RawObjectType::Password(deserialize_string_object_bounds(mapping)?),
            "boolean" => RawObjectType::Boolean(deserialize_boolean_object_bounds(mapping)?),
            _ => return Err(Error::custom(format!("unknown object type `{}`", value))),
        };
        Ok(object_type)
    }
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
