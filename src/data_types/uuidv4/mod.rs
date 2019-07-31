use serde_json::Value;
use uuid;

use crate::generator::GenerateValueError;
use crate::validators::{ValidationState, WalkContextExt};
use crate::{Schema, WalkContext};

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    match data.parse::<uuid::Uuid>() {
        Ok(uuid) if uuid.get_version_num() == 4 => ValidationState::new(),
        Ok(_) => ctx.validation_error("type", "expected valid UUIDv4").into(),
        Err(_) => ctx.validation_error("type", "expected valid UUIDv4").into(),
    }
}

fn generate(_schema: &Schema) -> Result<Value, GenerateValueError> {
    Ok(Value::String(uuid::Uuid::new_v4().to_hyphenated().to_string()))
}

data_type!("uuidv4", validator: validate, generator: generate);
