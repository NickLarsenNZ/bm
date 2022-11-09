use std::{
    env,
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use ron::ser::PrettyConfig;

use crate::data::{self, BookmarkTable};

trait BmOps {
    fn save(&self, title: String, url: String) -> anyhow::Result<()>;
    fn delete(&self) -> anyhow::Result<()>;
    fn open(partial_title: String) -> anyhow::Result<crate::data::Bookmark>;
    fn list() -> anyhow::Result<BookmarkTable>;
}

trait BmStats {
    type Filter;

    fn count(&self, filter_by: Self::Filter) -> u32;
}

enum SimpleFilter {
    /// Do not filter
    All,

    /// The description or URL contains the text (case-insensitive)
    Contains(String),

    /// Filter by domain name.
    // eg: example.com
    Domain(String),

    /// Filter by URL scheme.
    /// eg: https, file, mastodon
    Scheme(String),
}

#[derive(Default)]
pub struct Bm {
    db_path: PathBuf,
    bookmarks: crate::data::BookmarksDatabase,
}

const DB_FILENAME: &str = "bm.ron";

impl Bm {
    pub fn new() -> anyhow::Result<Self> {
        let db_path = env::var("XDG_DATA_HOME")
            .map(|dir| Path::new(&dir).join(DB_FILENAME))
            .or_else(|_| {
                env::var("HOME")
                    .map(|dir| {
                        Path::new(&dir)
                            .join(".local")
                            .join("share")
                            .join(DB_FILENAME)
                    })
                    .map_err(|e| {
                        anyhow!("Unable to build a path for the {DB_FILENAME} database using $HOME: {e}. Consider setting $XDG_DATA_HOME")
                    })
            })?;

        Ok(Self {
            db_path,
            ..Default::default()
        })
    }

    /// Read in the database if it exists, otherwise create an empty database
    pub(crate) fn load_db(&mut self) -> anyhow::Result<()> {
        match std::fs::metadata(&self.db_path) {
            Ok(metadata) => {
                // The file exists, lets try and open it
                self.bookmarks = ron::de::from_reader(
                    File::open(&self.db_path)
                        .map_err(|e| anyhow!("Unable to open DB file: {e}"))?,
                )
                .map_err(|e| anyhow!("Unable to deserialize DB from file: {e}"))?;

                match self.bookmarks.check_schema() {
                    data::SchemaVersion::Newer => todo!("hint to upgrade the binary"),
                    data::SchemaVersion::Older => todo!("hint to upgrade the schema (migration)"),
                    _ => Ok(()),
                }
            }
            _ => {
                // The file doesn't appear to exist
                let new_db_file = File::create(&self.db_path)
                    .map_err(|e| anyhow!("Unable to create DB file: {e}"))?;
                ron::ser::to_writer_pretty(
                    new_db_file,
                    &self.bookmarks,
                    PrettyConfig::new(), // todo: tidy up file formatting
                )
                .map_err(|e| anyhow!("Unable to write new DB: {e}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    // Setting and unsetting env vars is thread-safe, however we don't want to be changing our test invariants.
    static TEST_MUTEX: Mutex<i32> = Mutex::new(1);

    #[test]
    fn fails_if_xdg_and_home_env_unset() {
        let _guard = TEST_MUTEX.lock().unwrap();

        std::env::remove_var("XDG_DATA_HOME");
        std::env::remove_var("HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_err())
    }

    #[test]
    fn succeeds_if_xdg_env_set() {
        let _guard = TEST_MUTEX.lock().unwrap();

        std::env::set_var("XDG_DATA_HOME", "/sample");
        std::env::remove_var("HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_ok())
    }

    #[test]
    fn succeeds_if_home_env_set() {
        let _guard = TEST_MUTEX.lock().unwrap();

        std::env::set_var("HOME", "/sample");
        std::env::remove_var("XDG_DATA_HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_ok())
    }

    // todo: add tests for corrupt DB, empty DB
}
