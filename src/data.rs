use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

pub(crate) type BookmarkTable = BTreeMap<String, Bookmark>;

/// To be changed if the schema changes.
/// You should then also implement a migration path from the previous version.
/// Note: version 0 indicates an unstable schema that might change without incrementing.
// Todo: implement a migrate/upgrade command to handle rewriting the data between versions.
pub const SCHEMA_VERSION: u8 = 0;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookmarksDatabase {
    schema_version: u8,
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

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Bookmark {
    timestamp: DateTime<Utc>,
    url: Url,
}
