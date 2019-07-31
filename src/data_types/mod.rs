//! Custom data types built on top of the basic Jelly Schema.
use std::collections::HashMap;

use crate::generator::BoxedGenerator;
use crate::validators::BoxedValidator;

#[macro_use]
mod macros;

mod binary;
mod chrony_address;
mod date;
mod date_time;
mod dnsmasq_address;
mod email;
mod file;
mod hostname;
mod ip_address;
mod ipv4_address;
mod ipv6_address;
mod password;
mod port;
mod time;
mod uuidv4;

// Following macro can generate modules as well, but some IDEs are puzzled and report
// that the data type module is not part of the project, stop highlighting, ...
// In general, it makes it harder to develop a data type.

data_types!(
    binary,
    chrony_address,
    date,
    date_time,
    dnsmasq_address,
    email,
    file,
    hostname,
    ip_address,
    ipv4_address,
    ipv6_address,
    password,
    port,
    time,
    uuidv4
);

/// Data type trait object.
pub type BoxedDataType = Box<dyn DataType>;

/// Map of data types (name -> trait object).
pub type DataTypeMap = HashMap<String, BoxedDataType>;

/// Data type interface.
pub trait DataType {
    /// Data type schema.
    fn schema(&self) -> &str;

    /// Data type validator.
    fn validator(&self) -> Option<BoxedValidator> {
        None
    }

    /// Data type generator.
    fn generator(&self) -> Option<BoxedGenerator> {
        None
    }
}
