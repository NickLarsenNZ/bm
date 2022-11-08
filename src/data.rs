use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use url::Url;

pub(crate) type BookmarkTable = BTreeMap<String, Bookmark>;

struct Bookmarks {
    schema_version: String,
    bookmarks: BookmarkTable,
}

pub(crate) struct Bookmark {
    timestamp: DateTime<Utc>,
    url: Url,
}
