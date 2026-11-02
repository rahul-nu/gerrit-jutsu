use std::ops::Deref;

use serde::{Deserialize, Deserializer};

use crate::account::Account;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApprovalValue(pub i8);

impl<'de> Deserialize<'de> for ApprovalValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.trim_start_matches('+')
            .parse::<i8>()
            .map(ApprovalValue)
            .map_err(serde::de::Error::custom)
    }
}

impl Deref for ApprovalValue {
    type Target = i8;

    fn deref(&self) -> &i8 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Approval<'a> {
    #[serde(rename = "type", borrow)]
    pub kind: &'a str,

    #[serde(borrow)]
    pub description: Option<&'a str>,

    pub value: ApprovalValue,

    #[serde(default)]
    pub old_value: Option<ApprovalValue>,

    pub granted_on: Option<i64>,

    #[serde(borrow)]
    pub by: Option<Account<'a>>,
}
