use serde::{Deserialize, Serialize};
use url::Url;

pub mod v0;

/// To be changed if the schema changes.
/// You should then also implement a migration path from the previous version.
/// Note: version 0 indicates an unstable schema that might change without incrementing.
// Todo: implement a migrate/upgrade command to handle rewriting the data between versions.
pub const SCHEMA_VERSION: i8 = 0;

pub trait SchemaValidation {
    fn get_schema_version(&self) -> i8;

    fn check_schema(&self) -> SchemaVersion {
        match (SCHEMA_VERSION - self.get_schema_version()) as i16 {
            i16::MIN..=-1 => SchemaVersion::Newer,
            0 => SchemaVersion::Same,
            1.. => SchemaVersion::Older,
        }
    }
}

// This is a minimal version of the shema so we can check the version before fully deserialising it.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MinimalSchema {
    schema_version: i8,
}

impl SchemaValidation for MinimalSchema {
    fn get_schema_version(&self) -> i8 {
        self.schema_version
    }
}

#[derive(Debug, PartialEq)]
pub enum SchemaVersion {
    Same,
    Newer,
    Older,
}

pub trait BookmarkUrl {
    fn get_url(&self) -> Url;
}

pub trait BookmarkTable {}
