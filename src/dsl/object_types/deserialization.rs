use crate::dsl::enums::EnumerationValues;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use crate::dsl::object_types::TypeDefinition;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;
use std::fmt;
use std::fmt::Formatter;

// TODO deserialize types with their bounds here, pass the whole object to the smaller methods
impl<'de> Deserialize<'de> for TypeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mapping = Mapping::deserialize(deserializer)?;

        let spec = deserialize_object_type(&mapping)?;

        // enum bound
        let enum_key = Value::from("enum");
        let enumeration_values = deserialize_enumeration_values(&mapping)?;

        let constant_key = Value::from("const");
        let constant = &mapping.get(&constant_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone())
                .map_err(|e| Error::custom(format!("cannot deserialize constant specifier: {:?} - {}", value, e)))
        })?;

        Ok(TypeDefinition {
            r#type: spec,
            enumeration_values,
            constant_value: constant.clone(),
        })
    }
}

fn deserialize_object_type<E>(mapping: &Mapping) -> Result<Option<ObjectType>, E>
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
        None => Ok(None),
    })?)
}

fn deserialize_enumeration_values<E>(mapping: &Mapping) -> Result<Option<EnumerationValues>, E>
where E: Error
{
    let enum_key = Value::from("enum");
    Ok(mapping.get(&enum_key).map_or(Ok(None), |value| {
        serde_yaml::from_value(value.clone()).map_err(|e| {
            Error::custom(format!(
                "cannot deserialize list of enumeration values: {:#?} - {}",
                value, e
            ))
        })
    })?)
}

impl RawObjectType {
    fn from<E>(value: &str, mapping: &Mapping) -> Result<Self, E>
    where
        E: Error,
    {

        let enumeration_values = deserialize_enumeration_values(&mapping)?;
        let object_type = match value {
            "object" => RawObjectType::Object,
            "string" => RawObjectType::String(enumeration_values),
            "hostname" => RawObjectType::Hostname,
            "integer" => RawObjectType::Integer(vec![]),
            _ => return Err(Error::custom(format!("unknown object type `{}`", value))),
        };
        Ok(object_type)
    }
}
