use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

pub(crate) type BookmarkTable = BTreeMap<String, Bookmark>;

/// To be changed if the schema changes.
/// You should then also implement a migration path from the previous version.
/// Note: version 0 indicates an unstable schema that might change without incrementing.
// Todo: implement a migrate/upgrade command to handle rewriting the data between versions.
pub const SCHEMA_VERSION: i8 = 0;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookmarksDatabase {
    schema_version: i8,
    bookmarks: BookmarkTable,
}

impl Default for BookmarksDatabase {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            bookmarks: BookmarkTable::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SchemaVersion {
    Same,
    Newer,
    Older,
}

impl BookmarksDatabase {
    pub fn check_schema(&self) -> SchemaVersion {
        match (SCHEMA_VERSION - self.schema_version) as i16 {
            i16::MIN..=-1 => SchemaVersion::Newer,
            0 => SchemaVersion::Same,
            1.. => SchemaVersion::Older,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Bookmark {
    timestamp: DateTime<Utc>,
    url: Url,
}

#[cfg(test)]
mod test {
    use super::{BookmarksDatabase, SchemaVersion, SCHEMA_VERSION};

    #[test]
    fn newer_db_schema() {
        // Loading a DB where the schema is newer than we can handle
        let db = BookmarksDatabase {
            schema_version: SCHEMA_VERSION + 2,
            ..Default::default()
        };

        assert_eq!(db.check_schema(), SchemaVersion::Newer)
    }

    #[test]
    fn equal_db_schema() {
        // Loading a DB where the schema is newer than we can handle
        let db = BookmarksDatabase {
            schema_version: SCHEMA_VERSION,
            ..Default::default()
        };

        assert_eq!(db.check_schema(), SchemaVersion::Same)
    }

    #[test]
    fn older_db_schema() {
        // Loading a DB where the schema is newer than we can handle
        let db = BookmarksDatabase {
            schema_version: SCHEMA_VERSION - 2,
            ..Default::default()
        };

        assert_eq!(db.check_schema(), SchemaVersion::Older)
    }
}
