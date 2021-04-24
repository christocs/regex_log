use std::io::Write;

pub struct Config {}

#[derive(Debug)]
pub enum Err {}

#[derive(Debug)]
pub enum CreateFileErr {
    AlreadyExists,
    IsDirectory,
    IoErr(std::io::ErrorKind),
}

static TEMPLATE: &str = r##"
# Patterns and actions are defined as
# /regex/ -> [<line_actions...>][<regex_group_actions...>...]
# e.g.
# /^IMPORTANT!/ -> [underline]
# /^(REPORT!)([A-Z]) -> [bold][fg = green]
#
# prefix a line with '#' to make it a comment

[[global]]
# /^regex$/ -> [action, action][action][action]

[[default]]
# /^default_regex/ -> [action]

[[profiles]]
# [pink]
# /^PINK!$/ -> [fg = pink, bg = brown]

"##;

impl Config {
    pub fn create_template(path: &std::path::PathBuf) -> Result<(), CreateFileErr> {
        if path.exists() {
            if path.is_dir() {
                return Err(CreateFileErr::IsDirectory);
            } else {
                return Err(CreateFileErr::AlreadyExists);
            }
        }

        // if we can't canonicalize path, return error (except if the file is missing)
        if let Err(err) = path.canonicalize() {
            if std::io::ErrorKind::NotFound != err.kind() {
                return Err(CreateFileErr::IoErr(err.kind()));
            }
        }

        // create template
        let mut file = match std::fs::File::create(&path) {
            Ok(f) => f,
            Err(err) => {
                return Err(CreateFileErr::IoErr(err.kind()));
            }
        };

        // write our template
        if let Err(err) = file.write_all(TEMPLATE.as_bytes()) {
            return Err(CreateFileErr::IoErr(err.kind()));
        }

        // if we can't canonicalize path, return error
        if let Err(err) = path.canonicalize() {
            return Err(CreateFileErr::IoErr(err.kind()));
        }

        // if we get to here, we should be good
        Ok(())
    }

    pub fn read(path: std::path::PathBuf) -> Result<Self, Err> {
        Ok(Config {})
    }
}
