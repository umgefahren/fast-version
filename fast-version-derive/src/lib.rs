//! Derive macro defintion for the fast-version crate
//!
//! Refer to the [fast-version](https://crates.io/crates/fast-version) for usage and documentation. 

use litrs::Literal;
use quote::quote;
use std::str::FromStr;

/// Allows compile time generation of Versions from string literals.
/// ```
/// # use fast_version_core::version::Version;
/// # use fast_version_derive::const_version;
/// const VERSION: Version = const_version!("1.2.3");
///
/// assert_eq!(VERSION.major, 1);
/// assert_eq!(VERSION.minor, 2);
/// assert_eq!(VERSION.patch, 3);
/// ```
#[proc_macro]
pub fn const_version(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let first_token = input.into_iter().next().expect("expected input into macro");

    match Literal::try_from(first_token) {
        Err(e) => return e.to_compile_error(),
        Ok(Literal::String(string)) => {
            let value = string.value();
            let version = fast_version_core::version::Version::from_str(value)
                .expect("Error while parsing string literal into version");
            let major = version.major;
            let minor = version.minor;
            let patch = version.patch;
            quote! {
                {
                    const MAJOR: u64 = #major;
                    const MINOR: u64 = #minor;
                    const PATCH: u64 = #patch;
                    const VERSION: Version = Version::new(MAJOR, MINOR, PATCH);
                    VERSION
                }
            }.into()
        }
        Ok(other) => {
            panic!("Got non string literal: {}", other);
        }
    }
}
