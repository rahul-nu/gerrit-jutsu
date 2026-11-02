use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefUpdate<'a> {
    #[serde(borrow)]
    pub old_rev: &'a str,

    /// All-zero SHA indicates the ref was deleted.
    #[serde(borrow)]
    pub new_rev: &'a str,

    #[serde(borrow)]
    pub ref_name: &'a str,

    #[serde(borrow)]
    pub project: &'a str,
}
