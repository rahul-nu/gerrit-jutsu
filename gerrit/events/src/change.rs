use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Account, PatchSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeKey<'a> {
    #[serde(borrow)]
    pub id: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change<'a> {
    #[serde(borrow)]
    pub project: &'a str,

    #[serde(borrow)]
    pub branch: &'a str,

    #[serde(borrow, default)]
    pub topic: Option<&'a str>,

    #[serde(borrow)]
    pub id: &'a str,

    pub number: u32,

    #[serde(borrow)]
    pub subject: Cow<'a, str>,

    #[serde(borrow)]
    pub owner: Account<'a>,

    #[serde(borrow)]
    pub url: &'a str,

    /// it seems strings with newlines don't behave well with serde(borrow)
    pub commit_message: Option<String>,

    #[serde(borrow, default)]
    pub hashtags: Vec<&'a str>,

    pub created_on: u64,

    #[serde(default)]
    pub last_updated: Option<u64>,

    pub status: Status,

    #[serde(default)]
    pub private: Option<bool>,

    #[serde(default)]
    pub wip: Option<bool>,

    #[serde(default)]
    pub current_patch_set: Option<PatchSet<'a>>,
    // pub patch_sets: Vec<PatchSet>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    New,
    Merged,
    Abandoned,
    NoChange,
}
