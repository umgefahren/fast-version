#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;


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
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let major_ordering = self.major.partial_cmp(&other.major);
        if let Some(d) = major_ordering {
            if d.is_ne() {
                return Some(d);
            }
        }
        let minor_ordering = self.minor.partial_cmp(&other.minor);
        if let Some(d) = minor_ordering {
            if d.is_ne() {
                return Some(d);
            }
        }
        self.patch.partial_cmp(&other.patch)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let major_ordering = self.major.cmp(&other.major);
        if major_ordering.is_ne() {
            return major_ordering;
        }
        let minor_ordering = self.minor.cmp(&other.minor);
        if minor_ordering.is_ne() {
            return minor_ordering;
        }
        self.patch.cmp(&other.patch)
    }
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

    pub fn new_from_str(input: &str) -> Result<Self, VersionParseError> {
        let splits: Vec<&str> = input.split('.').collect();
        if splits.len() != 3 {
            return Err(VersionParseError::FormatWrong);
        }
        let major_str = splits.get(0).unwrap();
        let major = u64::from_str(major_str).map_err(|_| VersionParseError::MajorParseError)?;
        let minor_str = splits.get(1).unwrap();
        let minor = u64::from_str(minor_str).map_err(|_| VersionParseError::MinorParseError)?;
        let patch_str = splits.get(2).unwrap();
        let patch = u64::from_str(patch_str).map_err(|_| VersionParseError::PatchParseError)?;
        Ok(Self::new(major, minor, patch))
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
        Self::new_from_str(s)
    }
}

#[cfg(feature = "alloc")]
impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
