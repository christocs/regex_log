# regex_log
Watch logs with regular expressions for pattern matching!

# Planned Features
- Have "actions" that can be applied to a whole line and each regex group, including:
    - Foreground colorising
    - Background colorising
    - Hide
    - Underline
    - Strikethrough
    - Bold
    - Italics
- User profiles that contain a list of regular expressions and actions
    - Ideally this should be custom to make profiles easy and quick to write
- Watch multiple logs at once
    - Watch logs over SSH sessions
    - Able to watch local and remote logs at the same time
- Cross platform support on Windows, Linux and MacOs
- Exposed modules as a crate, to be easily re-usable for other packages

# Re-requisites
1. Cargo - [Install here](https://rustup.rs/)

# Build Steps
1. `cargo build`
