use std::collections::{HashMap, HashSet};

use crate::model::Result;

/// Represent all aliases linked to a target's id
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

pub type AliasIndex = HashMap<String, String>;

/// An extension for crate::alias::model::AliasesCollection to ease the map manipulations
pub trait AliasesCollectionExt {
    /// Try to obtain all targets' id with the given alias
    fn targets_from_alias(&self, alias: &str) -> Result<Vec<String>>;

    /// Try to search if the given argument is an existing alias
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(String))` containing the linked target's id if there is one or None
    ///
    /// # Notes
    ///
    /// Returns the first found target. It does not check if there are multiples target with this same alias.
    fn search_target(&self, alias: &str) -> Result<Option<String>>;

    /// Build a map of with the alias as a key and the target's id as a value
    fn build_alias_index(&self) -> AliasIndex;
}

impl AliasesCollectionExt for AliasesCollection {
    fn targets_from_alias(&self, alias: &str) -> Result<Vec<String>> {
        let mut res = Vec::new();
        for (target, aliases) in self.iter() {
            if aliases.contains(alias) {
                res.push(target.clone());
            }
        }
        Ok(res)
    }

    fn search_target(&self, alias: &str) -> Result<Option<String>> {
        for (target, aliases) in self.iter() {
            if aliases.contains(alias) {
                return Ok(Some(target.clone()));
            }
        }
        Ok(None)
    }

    fn build_alias_index(&self) -> AliasIndex {
        let mut index = HashMap::new();
        for (target, aliases) in self.iter() {
            for alias in aliases.iter() {
                index.insert(alias.clone(), target.clone());
            }
        }
        index
    }
}
