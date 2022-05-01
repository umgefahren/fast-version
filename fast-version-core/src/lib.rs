//! Core definitions for the fast-version crate
//!
//! Refer to the [fast-version](https://crates.io/crates/fast-version) for usage and documentation.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(portable_simd))]

pub mod version;
pub mod version_req;
