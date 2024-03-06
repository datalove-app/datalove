use crate::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, hash,
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Subject(async_nats::Subject);

impl Subject {
    pub const fn from_static(input: &'static str) -> Self {
        Subject(async_nats::Subject::from_static(input))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // pub fn is_empty(&self) -> bool {
    //     self.0.is_empty()
    // }

    // pub fn is_exact(&self) -> bool {
    //     self.0.as_str().chars().all(|c| !matches!(c, '*' | '>'))
    // }

    /// Does the subject match the pattern?
    ///
    /// # Examples
    /// ```
    /// use iroh_nats::Subject;
    /// assert!(Subject::from_static("sub").matches("sub"));
    /// assert!(Subject::from_static("sub").matches("*"));
    /// assert!(Subject::from_static("sub").matches(">"));
    ///
    /// assert!(!Subject::from_static("pub").matches("sub"));
    /// assert!(!Subject::from_static("sub").matches("sub.pub"));
    /// assert!(!Subject::from_static("sub").matches("*.pub"));
    ///
    /// assert!(Subject::from_static("sub.pub").matches("sub.pub"));
    /// assert!(Subject::from_static("sub.pub").matches("sub.*"));
    /// assert!(Subject::from_static("sub.pub").matches("*.pub"));
    /// assert!(Subject::from_static("sub.pub").matches("*.*"));
    /// assert!(Subject::from_static("sub.pub").matches(">"));
    ///
    /// assert!(!Subject::from_static("sub.pub").matches("sub"));
    /// assert!(!Subject::from_static("sub.pub").matches("pub"));
    /// assert!(!Subject::from_static("sub.pub").matches("sub.*.pub"));
    /// ```
    pub fn matches(&self, pattern: impl AsRef<str>) -> bool {
        let mut pattern_parts = pattern.as_ref().split('.');
        for subject_part in self.as_str().split('.') {
            if let Some(pattern_part) = pattern_parts.next() {
                if pattern_part == ">" {
                    return true;
                } else if pattern_part == subject_part || pattern_part == "*" {
                    continue;
                }
            }
            return false;
        }
        pattern_parts.next().is_none()
    }

    /// Find any subscriptions which match the subject
    ///
    /// # Examples
    /// ```
    /// use iroh_nats::Subject;
    /// use std::collections::{BTreeMap, BTreeSet};
    ///
    /// // let mut subs = BTreeMap::new();
    ///
    /// ```
    pub fn filter_subscriptions<'a, T>(
        &'a self,
        subs: &'a BTreeMap<Subject, BTreeSet<T>>,
    ) -> impl Iterator<Item = &'a T> {
        subs.iter()
            .filter(|(subj, _)| self.matches(subj))
            .flat_map(|(_, ids)| ids.iter())
    }
}

impl AsRef<[u8]> for Subject {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl AsRef<str> for Subject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for Subject {
    fn from(s: &str) -> Self {
        Self(async_nats::Subject::from(s))
    }
}

impl From<String> for Subject {
    fn from(s: String) -> Self {
        Self(async_nats::Subject::from(s))
    }
}

impl FromStr for Subject {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl hash::Hash for Subject {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
