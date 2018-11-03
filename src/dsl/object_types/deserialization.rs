use crate::dsl::object_types::bounds::deserialization::deserialize_enumeration_values;
use crate::dsl::object_types::bounds::deserialization::deserialize_integer_bounds;
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
    Ok(mapping.get(&type_key).map_or(Ok(None), |value| match value.as_str() {
        Some(value) => Ok(Some({
            let mut type_name = value.trim().to_lowercase();
            if type_name.ends_with('?') {
                type_name.remove(type_name.len() - 1);
                ObjectType::Optional(RawObjectType::from(&type_name, &mapping)?)
            } else {
                ObjectType::Required(RawObjectType::from(&type_name, &mapping)?)
            }
        })),
        None => Err(Error::custom("cannot find type definition")),
    })?)
}

impl RawObjectType {
    fn from<E>(value: &str, mapping: &Mapping) -> Result<Self, E>
    where
        E: Error,
    {
        let object_type = match value {
            "object" => RawObjectType::Object, // fixme - deserialize recursively here
            "string" => {
                let enumeration_values = deserialize_enumeration_values(mapping)?;
                RawObjectType::String(enumeration_values)
            }
            "hostname" => RawObjectType::Hostname,
            "integer" => {
                let bounds = deserialize_integer_bounds(mapping)?;
                RawObjectType::Integer(bounds)
            }
            _ => return Err(Error::custom(format!("unknown object type `{}`", value))),
        };
        Ok(object_type)
    }
}

pub fn deserialize_integer<E>(name: &str, mapping: &Mapping) -> Result<Option<i64>, E>
where
    E: Error,
{
    let maximum_key = Value::from(name);
    let value = mapping.get(&maximum_key);
    println!("value from mapping {:#?} for {}", &value, &name);

    match value {
        None => Ok(None),
        Some(value) => match value.as_i64() {
            None => Err(Error::custom(format!("cannot deserialize {:#?} as integer", value))),
            Some(value) => Ok(Some(value)),
        },
    }
}
