//! Defines error handling types used by the create
//! uses the `snafu` crate for generation

use std::{convert::From, fmt::Display};

use neon::result::Throw;
use serde::{de, ser};
use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    /// NodeJS has a hardcoded limit on string length
    ///
    /// Trying to serialize a string that is too long will result in an error
    #[snafu(display("String too long for NodeJS, len: {len}"))]
    StringTooLong {
        len: usize,
        backtrace: Backtrace,
    },

    /// Unable to coerce type
    ///
    /// When deserializing to a boolean, valid inputs are:
    /// * `false` / `true`
    /// * `undefined`
    /// * `null`
    /// * `number`
    ///
    /// Any other type will result in an error
    #[snafu(display("Unable to coerce value to type: {to_type}"))]
    UnableToCoerce {
        to_type: &'static str,
        backtrace: Backtrace,
    },

    /// Occurs when deserializing a char from an empty string
    #[snafu(display("Attempted to deserialize from an empty string"))]
    EmptyString {
        backtrace: Backtrace,
    },

    /// Occurs when deserializing a char from a sting with
    /// more than one character
    #[snafu(display("String too long to be a char expected len: 1, got {len}"))]
    StringTooLongForChar {
        len: usize,
        backtrace: Backtrace,
    },

    /// Occurs when deserializer expects a `null` or `undefined`
    /// but instead another type was found
    #[snafu(display("Found unexpected non-null type when deserializing"))]
    ExpectingNull {
        backtrace: Backtrace,
    },

    /// Occurs when deserializing to an enum where the source object has
    /// a none-1 number of properties
    #[snafu(display("Error when deserializing enum, found key: '{key}'"))]
    InvalidKeyType {
        key: String,
        backtrace: Backtrace,
    },

    /// An internal deserialization error from an invalid array
    #[snafu(display("ArrayIndexOutOfBounds: attempted access to ({index}) when size: ({length})"))]
    ArrayIndexOutOfBounds {
        index: u32,
        length: u32,
        backtrace: Backtrace,
    },

    /// A JS exception was throws
    #[snafu(display("JS exception: {throw}"))]
    Js {
        throw: Throw,
        backtrace: Backtrace,
    },

    /// Failed to convert something to f64
    #[snafu(display("Unable to convert something to f64"))]
    CastError {
        backtrace: Backtrace,
    },

    /// An error from serde
    #[snafu(display("Error occurred while (de)serializing: {msg}"))]
    #[snafu(context(suffix(false)))]
    Serde {
        msg: String,
        backtrace: Backtrace,
    },

    /// This type of object is not supported
    #[doc(hidden)]
    #[snafu(display("Deserialization not implemented for {name}"))]
    #[snafu(context(suffix(false)))]
    NotImplemented {
        name: &'static str,
        backtrace: Backtrace,
    },
}

pub type Result<T> = ::core::result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Serde { msg: msg.to_string() }.build()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Serde { msg: msg.to_string() }.build()
    }
}

impl From<Throw> for Error {
    fn from(err: Throw) -> Self {
        JsSnafu { throw: err }.build()
    }
}
