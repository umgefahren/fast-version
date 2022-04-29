//! Utility crate for fast SemVer **like** Versions and Version Requirenments. If you look for full
//! SemVer experience and/or easy usage, please consider the excellent
//! [semver](https://docs.rs/semver/latest/semver/index.html) crate.
//!
//! This implementation however doesn't require allocations, thus can be used in compile time
//! evaluation. (support for embedded will follow)
//!
//! ## Example
//! ```
//! use fast_version::{Version, VersionReq, VersionReqVariant, const_version};
//!
//! const VERSION: Version = const_version!("1.2.3");
//!
//! assert_eq!(VERSION.major, 1);
//! assert_eq!(VERSION.minor, 2);
//! assert_eq!(VERSION.patch, 3);
//!
//! const VERSION_REQ_MATCH: VersionReq = {
//!     let version_req_variant = VersionReqVariant::Strict(VERSION); 
//!     VersionReq::new(&version_req_variant)
//! };
//!
//! assert!(VERSION_REQ_MATCH.matches(&VERSION));
//!
//! const VERSION_REQ_UNMATCH: VersionReq = {
//!     let version_req_variant = VersionReqVariant::MajorLessEqual { major: 0 };
//!     VersionReq::new(&version_req_variant)
//! };
//!
//! assert!(!VERSION_REQ_UNMATCH.matches(&VERSION));
//! ```


pub use fast_version_core::version::Version;
pub use fast_version_core::version_req::*;
pub use fast_version_derive::const_version;
