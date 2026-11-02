use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Account;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PatchSetKind {
    Rework,
    TrivialRebase,
    TrivialRebaseWithMessageUpdate,
    MergeFirstParentUpdate,
    NoCodeChange,
    NoChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatchSetRef {
    pub change_num: u32,
    pub patch_num: u16,
}

impl<'de> Deserialize<'de> for PatchSetRef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let s = s
            .strip_prefix("refs/changes/")
            .ok_or_else(|| serde::de::Error::custom("missing refs/changes/ prefix"))?;

        let mut parts = s.split('/');

        let shard = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing shard"))?;
        if shard.len() != 2 {
            return Err(serde::de::Error::custom("shard must be 2 digits"));
        }

        let change_num = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing change_num"))?
            .parse::<u32>()
            .map_err(serde::de::Error::custom)?;

        let patch_num = parts
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing patch_num"))?
            .parse::<u16>()
            .map_err(serde::de::Error::custom)?;

        Ok(PatchSetRef {
            change_num,
            patch_num,
        })
    }
}

impl Serialize for PatchSetRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = format!(
            "refs/changes/{:02}/{}/{}",
            self.change_num % 100,
            self.change_num,
            self.patch_num
        );
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchSet<'a> {
    pub number: u16,

    #[serde(borrow)]
    pub revision: &'a str,

    #[serde(borrow)]
    #[serde(default)]
    pub parents: Vec<&'a str>,

    #[serde(rename = "ref")]
    pub patch_set_ref: PatchSetRef,

    #[serde(borrow)]
    pub uploader: Account<'a>,

    #[serde(borrow)]
    pub author: Option<Account<'a>>,

    pub created_on: u64,

    pub kind: Option<PatchSetKind>,
    // #[serde(default)]
    // #[serde(borrow)]
    // pub approvals: Vec<Approval<'a>>,
    // pub size_insertions: Option<i64>,
    // pub size_deletions: Option<i64>,
}
