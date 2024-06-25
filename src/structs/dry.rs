use serde::{Deserialize, Serialize};

// This file is for repeating Virables to reduce repeating yourself based on the the DRY(Do not Repeat Yourself) rule.

#[derive(Deserialize, Clone, Debug)]
pub enum Args {
    Simple(String),
    ComplexArgs,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexArgs {
    pub(crate) rules: Vec<Rules>,
    pub(crate) value: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rules {
    pub(crate) action: String,
    pub(crate) features: Option<RulesFutures>,
    pub(crate) os: Option<RulesOS>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RulesFutures {}

#[derive(Deserialize, Clone, Debug)]
pub struct RulesOS {
    pub(crate) name: String,
    pub(crate) arch: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub(crate) id: Option<String>,
    pub(crate) sha1: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) size: Option<i32>,
    pub(crate) totalsize: Option<i32>,
    pub(crate) url: Option<String>,
}
