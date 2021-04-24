pub use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str), short = "c", long = "config-path")]
    pub config_path: Option<std::path::PathBuf>,
    #[structopt(short = "fc", long = "force-colors")]
    pub force_colors: bool,
    #[structopt(parse(try_from_str))]
    pub files: Vec<WatchItem>,
}

#[derive(Debug)]
pub struct WatchItem {
    pub file: std::path::PathBuf,
    pub profile: Option<String>,
}

#[derive(Debug)]
pub enum WatchItemFileError {
    NoFile,
    TooManySeperators,
}

impl std::fmt::Display for WatchItemFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WatchItemFileError::NoFile => "No File",
                WatchItemFileError::TooManySeperators => "Too Many Seperators",
            }
        )
    }
}

impl std::str::FromStr for WatchItem {
    type Err = WatchItemFileError;
    fn from_str(s: &str) -> Result<WatchItem, Self::Err> {
        let vec = s.split(':').collect::<Vec<&str>>(); // first part is file, second is pattern if it exists

        if vec.len() > 2 {
            return Err(WatchItemFileError::TooManySeperators);
        }

        if vec.is_empty() || vec[0].is_empty() {
            return Err(WatchItemFileError::NoFile);
        }

        Ok(WatchItem {
            file: std::path::PathBuf::from(vec[0]),
            profile: if vec.len() == 2 && !vec[1].is_empty() {
                Option::Some(vec[1].to_string())
            } else {
                Option::None
            },
        })
    }
}
