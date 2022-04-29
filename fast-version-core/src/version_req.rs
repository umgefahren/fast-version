use crate::version::Version;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// The variants in which a version requirenment can be constructed.
#[non_exhaustive]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VersionReqVariant {
    /// Equivalent of "1.2.3" where `1.2.3` is the only version this requirenment will match to.
    Strict(Version),
    /// Composition of an lower and an upper bound.
    Compound(VersionReqVariantLowerBound, VersionReqVariantUpperBound),
    /// Equivalent of ">1"
    MajorGreater { major: u64 },
    /// Equivalent of ">1.2"
    MinorGreater { major: u64, minor: u64 },
    /// Equivalent of ">1.2.3"
    PatchGreater { major: u64, minor: u64, patch: u64 },
    /// Equivalent of ">=1"
    MajorGreaterEqual { major: u64 },
    /// Equivalent of ">=1.2"
    MinorGreaterEqual { major: u64, minor: u64 },
    /// Equivalent of ">=1.2.3"
    PatchGreaterEqual { major: u64, minor: u64, patch: u64 },
    /// Equivalent of "<1"
    MajorLess { major: u64 },
    /// Equivalent of "<1.2"
    MinorLess { major: u64, minor: u64 },
    /// Equivalent of "<1.2.3"
    PatchLess { major: u64, minor: u64, patch: u64 },
    /// Equivalent of "<=1"
    MajorLessEqual { major: u64 },
    /// Equivalent of "<=1.2"
    MinorLessEqual { major: u64, minor: u64 },
    /// Equivalent of "<=1.2.3"
    PatchLessEqual { major: u64, minor: u64, patch: u64 },
}

/// Lower bound part of [VersionReqVariant::Compound]
#[non_exhaustive]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VersionReqVariantLowerBound {
    /// Equivalent of ">1"
    MajorGreater { major: u64 },
    /// Equivalent of ">1.2"
    MinorGreater { major: u64, minor: u64 },
    /// Equivalent of ">1.2.3"
    PatchGreater { major: u64, minor: u64, patch: u64 },
    /// Equivalent of ">=1"
    MajorGreaterEqual { major: u64 },
    /// Equivalent of ">=1.2"
    MinorGreaterEqual { major: u64, minor: u64 },
    /// Equivalent of ">=1.2.3"
    PatchGreaterEqual { major: u64, minor: u64, patch: u64 },
}

/// Lower bound part of [VersionReqVariant::Compound]
#[non_exhaustive]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VersionReqVariantUpperBound {
    /// Equivalent of "<1"
    MajorLess { major: u64 },
    /// Equivalent of "<1.2"
    MinorLess { major: u64, minor: u64 },
    /// Equivalent of "<1.2.3"
    PatchLess { major: u64, minor: u64, patch: u64 },
    /// Equivalent of "<=1"
    MajorLessEqual { major: u64 },
    /// Equivalent of "<=1.2"
    MinorLessEqual { major: u64, minor: u64 },
    /// Equivalent of "<=1.2.3"
    PatchLessEqual { major: u64, minor: u64, patch: u64 },
}

/// Representing an actual version requirenment, normally constructed through [VersionReq::new].
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VersionReq {
    pub(crate) major_lower: u64,
    pub(crate) minor_lower: u64,
    pub(crate) patch_lower: u64,
    pub(crate) major_higher: u64,
    pub(crate) minor_higher: u64,
    pub(crate) patch_higher: u64,
}

impl VersionReq {
    /// Equivalent of "*"
    pub const STAR: Self = Self::star();

    const fn star() -> Self {
        const MAX: u64 = u64::MAX;
        const MIN: u64 = u64::MIN;
        Self {
            major_lower: MIN,
            minor_lower: MIN,
            patch_lower: MIN,
            major_higher: MAX,
            minor_higher: MAX,
            patch_higher: MAX,
        }
    }

    /// checks wether the Version Requirenment matches with the version. Returnes true if the
    /// Requirenments are met.
    pub const fn matches(&self, version: &Version) -> bool {
        let lower_match = self.major_lower <= version.major
            && self.minor_lower <= version.minor
            && self.patch_lower <= version.patch;
        let higher_match = self.major_higher >= version.major
            && self.minor_higher >= version.minor
            && self.patch_higher >= version.patch;
        lower_match && higher_match
    }

    /// Normal constructer of the Version Requirenment.
    pub const fn new(version_req: &VersionReqVariant) -> Self {
        match version_req {
            VersionReqVariant::Strict(d) => Self::new_strict(d),
            VersionReqVariant::Compound(lower, upper) => Self::new_compound(lower, upper),
            VersionReqVariant::MajorGreater { major } => {
                let major_geq = major.saturating_add(1);
                Self::new_lower_bounded_equal(major_geq, 0, 0)
            },
            VersionReqVariant::MinorGreater { major, minor } => {
                let major_geq = major.saturating_add(1);
                let minor_geq = minor.saturating_add(1);
                Self::new_lower_bounded_equal(major_geq, minor_geq, 0)
            },
            VersionReqVariant::PatchGreater { major, minor, patch } => {
                let major_geq = major.saturating_add(1);
                let minor_geq = minor.saturating_add(1);
                let patch_geq = patch.saturating_add(1);
                Self::new_lower_bounded_equal(major_geq, minor_geq, patch_geq)
            },
            VersionReqVariant::MajorGreaterEqual { major } => Self::new_lower_bounded_equal(*major, 0, 0),
            VersionReqVariant::MinorGreaterEqual { major, minor } => Self::new_lower_bounded_equal(*major, *minor, 0),
            VersionReqVariant::PatchGreaterEqual { major, minor, patch } => Self::new_lower_bounded_equal(*major, *minor, *patch),
            VersionReqVariant::MajorLess { major } => {
                let major_leq = major.saturating_sub(1);
                Self::new_upper_bounded_equal(major_leq, u64::MAX, u64::MAX)
            },
            VersionReqVariant::MinorLess { major, minor } => {
                let major_leq = major.saturating_sub(1);
                let minor_leq = minor.saturating_sub(1);
                Self::new_upper_bounded_equal(major_leq, minor_leq, u64::MAX)
            },
            VersionReqVariant::PatchLess { major, minor, patch } => {
                let major_leq = major.saturating_sub(1);
                let minor_leq = minor.saturating_sub(1);
                let patch_leq = patch.saturating_sub(1);
                Self::new_upper_bounded_equal(major_leq, minor_leq, patch_leq)
            },
            VersionReqVariant::MajorLessEqual { major } => Self::new_upper_bounded_equal(*major, u64::MAX, u64::MAX),
            VersionReqVariant::MinorLessEqual { major, minor } => Self::new_upper_bounded_equal(*major, *minor, u64::MAX),
            VersionReqVariant::PatchLessEqual { major, minor, patch } => Self::new_upper_bounded_equal(*major, *minor, *patch),
        }
    }

    #[inline]
    const fn new_upper_bounded_equal(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major_lower: 0,
            minor_lower: 0,
            patch_lower: 0,
            major_higher: major,
            minor_higher: minor,
            patch_higher: patch
        }
    }

    #[inline]
    const fn new_lower_bounded_equal(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major_lower: major,
            minor_lower: minor,
            patch_lower: patch,
            major_higher: u64::MAX,
            minor_higher: u64::MAX,
            patch_higher: u64::MAX
        }
    }

    #[inline]
    const fn new_strict(version: &Version) -> Self {
        let major = version.major;
        let minor = version.minor;
        let patch = version.patch;
        VersionReq { major_lower: major, minor_lower: minor, patch_lower: patch, major_higher: major, minor_higher: minor, patch_higher: patch }
    }
    
    #[inline]
    const fn new_compound(lower_bound: &VersionReqVariantLowerBound, upper_bound: &VersionReqVariantUpperBound) -> Self {
        let (major_lower, minor_lower, patch_lower) = Self::new_lower_bound(lower_bound);
        let (major_higher, minor_higher, patch_higher) = Self::new_upper_bound(upper_bound);
        Self {
            major_lower,
            minor_lower,
            patch_lower,
            major_higher,
            minor_higher,
            patch_higher
        }
    }

    #[inline]
    const fn new_lower_bound(lower_bound: &VersionReqVariantLowerBound) -> (u64, u64, u64) {
        match lower_bound {
            VersionReqVariantLowerBound::MajorGreater { major } => {
                let major_geq = major.saturating_add(1);
                (major_geq, 0, 0)
            },
            VersionReqVariantLowerBound::MinorGreater { major, minor } => {
                let major_geq = major.saturating_add(1);
                let minor_geq = minor.saturating_add(1);
                (major_geq, minor_geq, 0)
            },
            VersionReqVariantLowerBound::PatchGreater { major, minor, patch } => {
                let major_geq = major.saturating_add(1);
                let minor_geq = minor.saturating_add(1);
                let patch_geq = patch.saturating_add(1);
                (major_geq, minor_geq, patch_geq)
            },
            VersionReqVariantLowerBound::MajorGreaterEqual { major } => (*major, 0, 0),
            VersionReqVariantLowerBound::MinorGreaterEqual { major, minor } => (*major, *minor, 0),
            VersionReqVariantLowerBound::PatchGreaterEqual { major, minor, patch } => (*major, *minor, *patch),
        }
    }

    #[inline]
    const fn new_upper_bound(upper_bound: &VersionReqVariantUpperBound) -> (u64, u64, u64) {
        match upper_bound {
            VersionReqVariantUpperBound::MajorLess { major } => {
                let major_leq = major.saturating_sub(1);
                (major_leq, u64::MAX, u64::MAX)
            },
            VersionReqVariantUpperBound::MinorLess { major, minor } => {
                let major_leq = major.saturating_sub(1);
                let minor_leq = minor.saturating_sub(1);
                (major_leq, minor_leq, u64::MAX)
            },
            VersionReqVariantUpperBound::PatchLess { major, minor, patch } => {
                let major_leq = major.saturating_sub(1);
                let minor_leq = minor.saturating_sub(1);
                let patch_leq = patch.saturating_sub(1);
                (major_leq, minor_leq, patch_leq)
            },
            VersionReqVariantUpperBound::MajorLessEqual { major } => (*major, u64::MAX, u64::MAX),
            VersionReqVariantUpperBound::MinorLessEqual { major, minor } => (*major, *minor, u64::MAX),
            VersionReqVariantUpperBound::PatchLessEqual { major, minor, patch } => (*major, *minor, *patch),
        }
    }
}
