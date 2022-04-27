use std::time::Duration;

use anyhow::{anyhow, Result};
use sifis::{Discovery, Thing};

use crate::ConnectedObject;

/// All information needed to interact with the Connected Things
#[derive(Default)]
pub struct Context {
    /// All methods and protocols used to discover the Connected Things
    discovery: Discovery,
}

impl Context {
    /// Retrieves all the objects of a certain type.
    pub fn find_all<T: ConnectedObject + TryFrom<Thing>>(&self) -> Result<Vec<T>> {
        let all = self
            .discovery
            .discover_timeout(Duration::from_secs(2))?
            .into_iter()
            .filter_map(|t| t.try_into().ok())
            .collect();
        Ok(all)
    }
    /// Retrieves an object of a certain type using a specific identifier.
    pub fn find<T: ConnectedObject + TryFrom<Thing>>(&self, id: &str) -> Result<T> {
        self.discovery
            .discover_timeout(Duration::from_secs(2))?
            .into_iter()
            .filter(|co| co.has_attype(T::AT_TYPE))
            .find(|co| co.id == id)
            .and_then(|t| t.try_into().ok())
            .ok_or_else(|| anyhow!("Not found!"))
    }
}
