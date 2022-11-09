use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

pub(crate) type BookmarkTable = BTreeMap<String, Bookmark>;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookmarksDatabase {
    schema_version: i8,
    bookmarks: BookmarkTable,
}

impl Default for BookmarksDatabase {
    fn default() -> Self {
        Self {
            schema_version: 0,
            bookmarks: BookmarkTable::default(),
        }
    }
}

impl super::SchemaValidation for BookmarksDatabase {
    fn get_schema_version(&self) -> i8 {
        self.schema_version
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Bookmark {
    timestamp: DateTime<Utc>,
    url: Url,
}

#[cfg(test)]
mod test {
    use crate::data::{SchemaValidation, SchemaVersion, SCHEMA_VERSION};

    use super::BookmarksDatabase;

    #[test]
    fn newer_db_schema() {
        // Loading a DB where the schema is newer than we can handle
        let db = BookmarksDatabase {
            schema_version: SCHEMA_VERSION + 2,
            ..Default::default()
        };

        assert_eq!(db.check_schema(), SchemaVersion::Newer { by: 2 })
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

        assert_eq!(db.check_schema(), SchemaVersion::Older { by: 2 })
    }
}
