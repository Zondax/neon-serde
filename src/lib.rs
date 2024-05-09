#![allow(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(clippy)]
#![deny(clippy::pedantic)]
#![allow(stutter)]
#![recursion_limit = "128"]

//!
//! Neon-serde
//! ==========
//!
//! This crate is a utility to easily convert values between
//!
//! A `Handle<JsValue>` from the `neon` crate
//! and any value implementing `serde::{Serialize, Deserialize}`
//!
//! ## Usage
//!
//! #### `neon_serde::from_value`
//! Convert a `Handle<js::JsValue>` to
//! a type implementing `serde::Deserialize`
//!
//! #### `neon_serde::to_value`
//! Convert a value implementing `serde::Serialize` to
//! a `Handle<JsValue>`
//!
//!
//! ## Example
//!
//! ```rust,no_run
//! # #![allow(dead_code)]
//! # extern crate neon_serde;
//! # extern crate neon;
//! # extern crate serde;
//! # use serde::{Serialize, Deserialize};
//! use neon::prelude::*;
//!
//! type Result<'a, T> = neon_serde::errors::Result<Handle<'a, T>>;
//!
//! #[derive(Serialize, Debug, Deserialize)]
//! struct AnObject {
//!     a: u32,
//!     b: Vec<f64>,
//!     c: String,
//! }
//!#[cfg(feature = "legacy-runtime")]
//! fn deserialize_something<'j>(mut cx: FunctionContext<'j>) -> Result<'j, JsValue> {
//!     let arg0 = cx.argument::<JsValue>(0)?;
//!
//!     let arg0_value :AnObject = neon_serde::from_value(&mut cx, arg0)?;
//!     println!("{:?}", arg0_value);
//!
//!     Ok(JsUndefined::new().upcast())
//! }
//!
//! #[cfg(feature = "napi-6")]
//!  fn deserialize_something<'j>(mut cx: FunctionContext<'j>) -> Result<'j, JsValue> {
//!     let arg0 = cx.argument::<JsValue>(0)?;
//!
//!     let arg0_value :AnObject = neon_serde::from_value(&mut cx, arg0)?;
//!     println!("{:?}", arg0_value);
//!
//!     Ok(JsUndefined::new(&mut cx).upcast())
//! }
//!
//! fn serialize_something<'j>(mut cx: FunctionContext<'j>) -> Result<'j, JsValue> {
//!     let value = AnObject {
//!         a: 1,
//!         b: vec![2f64, 3f64, 4f64],
//!         c: "a string".into()
//!     };
//!
//!     let js_value = neon_serde::to_value(&mut cx, &value)?;
//!     Ok(js_value)
//! }
//! ```
//!

pub mod de;
pub mod errors;
pub mod ser;

mod macros;

pub use de::from_value;
pub use de::from_value_opt;
use neon::{context::Context, result::NeonResult};
pub use ser::to_value;

pub trait ResultExt<T>: Sized {
    fn throw<'cx, C: Context<'cx>>(
        self,
        cx: &mut C,
    ) -> NeonResult<T>;
}

impl<T> ResultExt<T> for errors::Result<T> {
    fn throw<'cx, C: Context<'cx>>(
        self,
        cx: &mut C,
    ) -> NeonResult<T> {
        use snafu::ErrorCompat;
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => match e.backtrace() {
                Some(backtrace) => cx.throw_error(format!("{e}! Backtrace:\n{backtrace}")),
                None => cx.throw_error(e.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use errors::Result as LibResult;
    use neon::prelude::*;

    use super::*;

    type Result<'a, T> = LibResult<Handle<'a, T>>;

    #[test]
    fn test_it_compiles() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> Result<'j, JsValue> {
            let result: () = {
                let arg: Handle<'j, JsValue> = cx.argument::<JsValue>(0)?;
                let () = from_value(&mut cx, arg)?;
                ()
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)?;
            Ok(result)
        }

        let _ = check;
    }

    #[test]
    fn test_it_compiles_2() {
        fn check<'j>(mut cx: FunctionContext<'j>) -> Result<'j, JsValue> {
            let result: () = {
                let arg: Option<Handle<'j, JsValue>> = cx.argument_opt(0);
                let () = from_value_opt(&mut cx, arg)?;
                ()
            };
            let result: Handle<'j, JsValue> = to_value(&mut cx, &result)?;
            Ok(result)
        }

        let _ = check;
    }
}
