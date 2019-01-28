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
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_boolean_object_bounds;
use crate::dsl::schema::object_types::bounds::deserialization::deserialize_bool;

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

pub fn deserialize_individual_type_definition<E>(definition: &str, mapping: &Mapping) -> Result<ObjectType, E>
where
    E: Error,
{
    let default_value = deserialize_default_value(mapping)?;
    let additional_properties = deserialize_bool("additionalProperties", mapping)?.unwrap_or(false);
    let is_optional = is_optional_type(definition);
    let type_name = type_name(definition);
    let raw_type = RawObjectType::from(&type_name, &mapping)?;
    let type_data = ObjectTypeData::new(raw_type, default_value, additional_properties);

    if is_optional {
        Ok(ObjectType::Optional(type_data))
    } else {
        Ok(ObjectType::Required(type_data))
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

fn type_name(definition: &str) -> String {
    let mut type_name = definition.trim().to_lowercase();
    if is_optional_type(definition) {
        type_name.remove(type_name.len() - 1);
    };
    type_name
}

fn is_optional_type(definition: &str) -> bool {
    definition.ends_with('?')
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
            "boolean" => RawObjectType::Boolean(deserialize_boolean_object_bounds(mapping)?),
            "array" => RawObjectType::Array(Box::new(deserialize_array_object_bounds(mapping)?)),
            "stringlist" => RawObjectType::Stringlist(Box::new(deserialize_array_object_bounds(mapping)?)),
            "hostname" => RawObjectType::Hostname(deserialize_string_object_bounds(mapping)?),
            "datetime" => RawObjectType::Datetime(deserialize_string_object_bounds(mapping)?),
            "date" => RawObjectType::Date(deserialize_string_object_bounds(mapping)?),
            "time" => RawObjectType::Time(deserialize_string_object_bounds(mapping)?),
            "email" => RawObjectType::Email(deserialize_string_object_bounds(mapping)?),
            "ipv4" => RawObjectType::IPV4(deserialize_string_object_bounds(mapping)?),
            "ipv6" => RawObjectType::IPV6(deserialize_string_object_bounds(mapping)?),
            "uri" => RawObjectType::URI(deserialize_string_object_bounds(mapping)?),
            "file" => RawObjectType::File(deserialize_string_object_bounds(mapping)?),
            "dnsmasq-address" => RawObjectType::DnsmasqAddress(deserialize_string_object_bounds(mapping)?),
            "chrony-address" => RawObjectType::ChronyAddress(deserialize_string_object_bounds(mapping)?),
            "iptables-address" => RawObjectType::IpTablesAddress(deserialize_string_object_bounds(mapping)?),
            _ => RawObjectType::Text(deserialize_string_object_bounds(mapping)?),
        };
        Ok(object_type)
    }
}
