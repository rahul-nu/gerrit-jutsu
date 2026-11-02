use std::borrow::Cow;

use serde::Deserialize;

use crate::{Account, Approval, Change, ChangeKey, PatchSet, RefUpdate};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(tag = "type")]
#[serde(bound(deserialize = "'de: 'a"))]
pub enum GerritEvent<'a> {
    #[serde(rename = "change-abandoned", rename_all = "camelCase")]
    ChangeAbandoned {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        abandoner: Account<'a>,

        #[serde(borrow)]
        reason: Cow<'a, str>,

        event_created_on: i64,
    },

    #[serde(rename = "change-deleted", rename_all = "camelCase")]
    ChangeDeleted {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        deleter: Account<'a>,
    },

    #[serde(rename = "change-merged", rename_all = "camelCase")]
    ChangeMerged {
        #[serde(borrow)]
        submitter: Account<'a>,

        #[serde(borrow)]
        new_rev: &'a str,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        change_key: ChangeKey<'a>,

        event_created_on: i64,
    },

    #[serde(rename = "change-restored", rename_all = "camelCase")]
    ChangeRestored {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        restorer: Account<'a>,

        #[serde(borrow)]
        reason: Cow<'a, str>,

        event_created_on: i64,
    },

    #[serde(rename = "dropped-output", rename_all = "camelCase")]
    DroppedOutput,

    #[serde(rename = "comment-added", rename_all = "camelCase")]
    CommentAdded {
        #[serde(borrow)]
        author: Account<'a>,

        #[serde(default, borrow)]
        approvals: Vec<Approval<'a>>,

        #[serde(default, borrow)]
        comment: Option<Cow<'a, str>>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        change: Change<'a>,

        event_created_on: i64,

        #[serde(borrow)]
        instance_id: &'a str,
    },

    #[serde(rename = "patchset-created", rename_all = "camelCase")]
    PatchsetCreated {
        #[serde(borrow)]
        uploader: Account<'a>,

        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        event_created_on: i64,
    },
    #[serde(rename = "hashtags-changed", rename_all = "camelCase")]
    HashtagsChanged {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        editor: Account<'a>,

        #[serde(default, borrow)]
        added: Vec<&'a str>,

        #[serde(default, borrow)]
        removed: Vec<&'a str>,

        #[serde(default, borrow)]
        hashtags: Vec<&'a str>,

        event_created_on: i64,
    },

    #[serde(rename = "project-created", rename_all = "camelCase")]
    ProjectCreated {
        #[serde(borrow)]
        project_name: &'a str,

        #[serde(borrow)]
        project_head: &'a str,

        event_created_on: i64,
    },

    #[serde(rename = "ref-updated", rename_all = "camelCase")]
    RefUpdated {
        #[serde(default, borrow)]
        submitter: Option<Account<'a>>,

        #[serde(borrow)]
        ref_update: RefUpdate<'a>,

        event_created_on: i64,
    },

    #[serde(rename = "reviewer-added", rename_all = "camelCase")]
    ReviewerAdded {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        reviewer: Account<'a>,

        #[serde(borrow)]
        adder: Account<'a>,

        event_created_on: i64,
    },

    #[serde(rename = "reviewer-deleted", rename_all = "camelCase")]
    ReviewerDeleted {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        reviewer: Account<'a>,

        #[serde(borrow)]
        remover: Account<'a>,

        #[serde(default, borrow)]
        approvals: Vec<Approval<'a>>,

        #[serde(borrow)]
        comment: Option<Cow<'a, str>>,

        event_created_on: i64,
    },
    #[serde(rename = "topic-changed", rename_all = "camelCase")]
    TopicChanged {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        changer: Account<'a>,

        #[serde(borrow)]
        old_topic: Option<&'a str>,

        event_created_on: i64,
    },

    #[serde(rename = "batch-ref-updated", rename_all = "camelCase")]
    BatchRefUpdated {
        #[serde(borrow)]
        submitter: Option<Account<'a>>,

        #[serde(borrow)]
        ref_updates: Vec<RefUpdate<'a>>,

        event_created_on: i64,
    },

    #[serde(rename = "wip-state-changed", rename_all = "camelCase")]
    WipStateChanged {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        changer: Account<'a>,

        event_created_on: i64,
    },

    #[serde(rename = "private-state-changed", rename_all = "camelCase")]
    PrivateStateChanged {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        changer: Account<'a>,

        event_created_on: i64,

        #[serde(borrow)]
        change_key: ChangeKey<'a>,
    },

    #[serde(rename = "vote-deleted", rename_all = "camelCase")]
    VoteDeleted {
        #[serde(borrow)]
        change: Change<'a>,

        #[serde(borrow)]
        patch_set: PatchSet<'a>,

        #[serde(borrow)]
        reviewer: Account<'a>,

        #[serde(borrow)]
        remover: Account<'a>,

        #[serde(default, borrow)]
        approvals: Vec<Approval<'a>>,

        #[serde(borrow)]
        comment: Option<Cow<'a, str>>,
    },

    #[serde(rename = "project-head-updated", rename_all = "camelCase")]
    ProjectHeadUpdated {
        #[serde(borrow)]
        old_head: &'a str,

        #[serde(borrow)]
        new_head: &'a str,

        event_created_on: i64,
    },
}

impl<'a> GerritEvent<'a> {
    pub fn project(&'a self) -> Option<&'a str> {
        match self {
            GerritEvent::RefUpdated { ref_update, .. } => Some(ref_update.project),
            GerritEvent::ChangeAbandoned { change, .. }
            | GerritEvent::ChangeMerged { change, .. }
            | GerritEvent::ChangeDeleted { change, .. }
            | GerritEvent::ChangeRestored { change, .. }
            | GerritEvent::CommentAdded { change, .. }
            | GerritEvent::ReviewerAdded { change, .. }
            | GerritEvent::ReviewerDeleted { change, .. }
            | GerritEvent::TopicChanged { change, .. }
            | GerritEvent::WipStateChanged { change, .. }
            | GerritEvent::PrivateStateChanged { change, .. }
            | GerritEvent::VoteDeleted { change, .. }
            | GerritEvent::PatchsetCreated { change, .. }
            | GerritEvent::HashtagsChanged { change, .. } => Some(change.project),
            GerritEvent::ProjectCreated { project_name, .. } => Some(project_name),
            GerritEvent::DroppedOutput { .. }
            | GerritEvent::ProjectHeadUpdated { .. }
            | GerritEvent::BatchRefUpdated { .. } => None,
        }
    }
}
