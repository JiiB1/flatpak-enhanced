use std::collections::{HashMap, HashSet};

use crate::model::CmdResult;

pub struct Aliases {
    pub target: String,
    pub aliases: HashSet<String>,
}

impl Aliases {
    pub fn new(target: String, aliases: HashSet<String>) -> Self {
        Aliases { target, aliases }
    }
}

pub type AliasesCollection = HashMap<String, HashSet<String>>;

pub trait AliasesCollectionExt {
    fn targets_from_alias(&self, alias: &str) -> CmdResult<Vec<String>>;

    fn search_target(&self, alias: &str) -> CmdResult<Option<String>>;
}

impl AliasesCollectionExt for AliasesCollection {
    fn targets_from_alias(&self, alias: &str) -> CmdResult<Vec<String>> {
        let mut res = Vec::new();
        for (target, aliases) in self.iter() {
            if aliases.contains(alias) {
                res.push(target.clone());
            }
        }
        Ok(res)
    }

    fn search_target(&self, alias: &str) -> CmdResult<Option<String>> {
        for (target, aliases) in self.iter() {
            if aliases.contains(alias) {
                return Ok(Some(target.clone()));
            }
        }
        Ok(None)
    }
}
