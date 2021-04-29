use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
}

#[derive(Debug)]
pub enum Action {
    // set foreground color
    SetForeground(Color),
    // set background color
    SetBackground(Color),
    Bold(bool),
    // set bold on/off
    Italic(bool),
    // set underline on/off
    Underline(bool),
    // set coloring to brighter colors
    IntenseColor(bool),
    // reset to "default"
    Reset,
}

pub type Actions = Vec<Action>;

#[derive(Debug)]
pub struct Rule {
    // regex rule to test
    regex: Regex,
    // actions to apply to each regex group in order
    group_actions: Vec<Actions>,
    // actions to apply to the whole string after group actions
    line_actions: Actions,
}

pub type Rules = Vec<Rule>;
pub type Profiles = HashMap<String, Rules>;

#[derive(Debug)]
pub struct Config {
    global: Rules,
    default: Rules,
    profiles: Profiles,
}

#[derive(Debug)]
pub enum ConfigErr {}

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

    pub fn read(path: std::path::PathBuf) -> Result<Self, ConfigErr> {
        Ok(Config {
            global: Rules::new(),
            default: Rules::new(),
            profiles: Profiles::new(),
        })
    }
}
