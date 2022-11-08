use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::data::BookmarkTable;

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

pub struct Bm {
    db_path: PathBuf,
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

        Ok(Self { db_path })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn fails_if_xdg_and_home_env_unset() {
        std::env::remove_var("XDG_DATA_HOME");
        std::env::remove_var("HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_err())
    }

    #[test]
    fn succeeds_if_xdg_env_set() {
        std::env::set_var("XDG_DATA_HOME", "/sample");
        std::env::remove_var("HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_ok())
    }

    #[test]
    fn succeeds_if_home_env_set() {
        std::env::set_var("HOME", "/sample");
        std::env::remove_var("XDG_DATA_HOME");

        let bm = crate::bm::Bm::new();
        assert!(bm.is_ok())
    }
}
