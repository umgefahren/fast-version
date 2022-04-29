use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref VERSION_RE: Regex = Regex::new("([0-9]+).([0-9]+).([0-9]+)").unwrap();
}

/// Version in a SemVer **like** way.
///
/// ## Example:
/// ```
/// # use fast_version_core::version::Version;
/// use std::str::FromStr;
///
/// const version_str: &'static str = "1.2.3";
///
/// let version = Version::from_str(version_str).unwrap();
/// assert_eq!(version.major, 1);
/// assert_eq!(version.minor, 2);
/// assert_eq!(version.patch, 3);
///
/// let version_out_str = version.to_string();
/// assert_eq!(&version_out_str, version_str);
/// ```
///
/// This crate is heavily optimized to allow compile-time evaluation:
/// ```
/// # use fast_version_core::version::Version;
/// const VERSION: Version = Version::new(1, 2, 3);
///
/// assert_eq!(VERSION.major, 1);
/// assert_eq!(VERSION.minor, 2);
/// assert_eq!(VERSION.patch, 3);
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}


impl Version {
    /// Create a new version from major, minor and patch.
    /// ```
    /// # use fast_version_core::version::Version;
    ///
    /// let version = Version::new(1, 2, 3);
    /// 
    /// assert_eq!(version.major, 1);
    /// assert_eq!(version.minor, 2);
    /// assert_eq!(version.patch, 3);
    /// ```
    #[inline]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Error, Debug)]
pub enum VersionParseError {
    #[error("Format of version string is wrong")]
    FormatWrong,
    #[error("Parsing error in major")]
    MajorParseError,
    #[error("Major element was not found")]
    MajorNotFound,
    #[error("Minor Parse Error")]
    MinorParseError,
    #[error("Minor element was not found")]
    MinorNotFound,
    #[error("Patch Parse Error")]
    PatchParseError,
    #[error("Patch element was not found")]
    PatchNotFound,
}

impl FromStr for Version {
    type Err = VersionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let match_result = VERSION_RE
            .captures(s)
            .ok_or(VersionParseError::FormatWrong)?;
        let major_str = match_result
            .get(1)
            .ok_or(VersionParseError::MajorNotFound)?
            .as_str();
        let minor_str = match_result
            .get(2)
            .ok_or(VersionParseError::MinorNotFound)?
            .as_str();
        let patch_str = match_result
            .get(3)
            .ok_or(VersionParseError::PatchNotFound)?
            .as_str();
        let major_num = u64::from_str(major_str).map_err(|_| VersionParseError::MajorParseError)?;
        let minor_num = u64::from_str(minor_str).map_err(|_| VersionParseError::MinorParseError)?;
        let patch_num = u64::from_str(patch_str).map_err(|_| VersionParseError::PatchParseError)?;
        let ret = Version::new(major_num, minor_num, patch_num);
        Ok(ret)
    }
}

#[cfg(feature = "alloc")]
impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
