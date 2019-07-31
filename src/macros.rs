macro_rules! compiler_non_strict_get {
    ($i:ident, $e:expr) => {
        match $i.get($e) {
            Some(x) => x,
            None => return Ok(None),
        }
    };
}

macro_rules! validator_non_strict_as {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return crate::validators::ValidationState::new(),
        }
    };
}
