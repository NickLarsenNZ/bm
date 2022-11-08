# `bm` - CLI Bookmark Utility

A simple way to save and open bookmarks from the CLI instead of slowly through the browser UI.

## Install

_Whoa there, horsey!_

Todo:
- cargo install
- asdf

## How to use

> **Note**: The [RON] DB will be stored in `$XDG_DATA_HOME`, and fall back to `$HOME/.local/share` as per [XDG Base Directory Specification].

Simply store a Key (Description) and Value (URL), and it can be opened in your default handler. You can even do local file/folders.

```sh
# Save a bookmark
bm save "XDG Base Directory Specification" https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

# Open a bookmark (exact match)
bm open "XDG Base Directory Specification"

# Open a bookmark (unambiguous match)
bm open "xdg"

# Open from a list (fzf)
bm open

# Or, shorthand for `bm open`
bm
```

[RON]: https://github.com/ron-rs/ron
[XDG Base Directory Specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

## Git sync

1. Move the [RON] file to your local repository clone. Eg:
   ```sh
   REPO_PATH="/path/to/repo"
   mv "${XDG_DATA_HOME:-$HOME/.local/share/bm.ron}" "${REPO_PATH}/"
   ```
2. Create a symbolic link pointing to the [RON] file in your repo. Eg:
   ```sh
   ln -s "${REPO_PATH}/bm.ron" "${XDG_DATA_HOME:-$HOME/.local/share/bm.ron}"
   ```
3. Commit and push changes as per normal.
