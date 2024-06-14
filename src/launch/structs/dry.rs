use serde::{Deserialize, Serialize};

// This file is for repeating Virables to reduce repeating yourself based on the the DRY(Do not Repeat Yourself) rule.

#[derive(Serialize, Deserialize, Clone)]
pub enum Args {
    Simple(String),
    ComplexArgs,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComplexArgs {
    pub(crate) rules: Vec<Rules>,
    pub(crate) value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rules {
    pub(crate) action: String,
    pub(crate) features: Option<RulesFutures>,
    pub(crate) os: Option<RulesOS>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RulesFutures {}

#[derive(Serialize, Deserialize, Clone)]
pub struct RulesOS {
    pub(crate) name: String,
    pub(crate) arch: String,
}
