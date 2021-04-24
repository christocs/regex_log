use atty;
use std::io::Write;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor};

mod cli;
mod config;
mod watcher;

fn main() {
    let args = match cli::Args::from_args_safe() {
        Ok(good_args) => good_args,
        Err(err) => {
            if let structopt::clap::ErrorKind::HelpDisplayed = err.kind {
                println!("{}", err.message);
            } else {
                eprintln!("{}", err.message);
            }
            return;
        }
    };

    if args.files.is_empty() {
        eprintln!("no files passed");
        return;
    }

    // pass in config path from args, or use a "default" config path
    // e.g. /home/$USER/.config/tail_regex.txt
    let config_path = match args.config_path {
        Some(arg_config_path) => arg_config_path,
        None => match dirs::config_dir() {
            Some(mut default_config_dir) => {
                &default_config_dir.to_owned();
                default_config_dir.push("tail_regex");
                default_config_dir.set_extension("txt");
                default_config_dir
            }
            None => panic!("Could not find config directory"),
        },
    };

    let config_path_str = match config_path.to_str() {
        Some(str) => str,
        None => {
            eprintln!("Config file: Unable to find path");
            return;
        }
    };

    // fail if we cannot resolve the path, except if the file doesn't exist yet
    if let Err(err) = config_path.canonicalize() {
        if std::io::ErrorKind::NotFound != err.kind() {
            eprintln!("Config file: Unable to resolve config path - {}", err);
            return;
        }
    }

    if config_path.exists() {
        if !config_path.is_file() {
            if config_path.is_dir() {
                eprintln!("Config file: {} is a directory", config_path_str);
                return;
            } else {
                eprintln!("Config file: {} is not a regular file", config_path_str);
                return;
            }
        }
    } else {
        print!(
            "Config file: {} doesn't exist, create it? [y/n]",
            config_path_str
        );

        match config::Config::create_template(&config_path) {
            Ok(_) => println!("Created config file at: {}", config_path_str),
            Err(err) => {
                eprintln!("Failed to create config file at: {} - {:?}", config_path_str, err);
                return;
            }
        }
    }

    let config = match config::Config::read(config_path) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };

    // copy the flag so we can use it in threads
    let force_colors = args.force_colors;

    let handles: Vec<std::thread::JoinHandle<_>> = args
        .files
        .into_iter()
        .map(move |item| {
            std::thread::spawn(move || {
                let mut watcher = match watcher::Watcher::register(item.file.clone()) {
                    Ok(watch) => watch,
                    Err(err) => {
                        eprintln!("{}", err);
                        return;
                    }
                };

                let watch_action = move |line: &str| {
                    let buff_writer = termcolor::BufferWriter::stdout(if force_colors {
                        ColorChoice::Always
                    } else if atty::is(atty::Stream::Stdout) {
                        ColorChoice::Auto
                    } else {
                        ColorChoice::Never
                    });

                    let mut buffer = buff_writer.buffer();
                    buffer
                        .set_color(
                            ColorSpec::new()
                                .set_fg(Some(Color::Green))
                                .set_bg(Some(Color::Cyan))
                                .set_bold(true),
                        )
                        .unwrap();

                    write!(&mut buffer, "{}", line).unwrap();
                    buff_writer.print(&buffer).unwrap();
                };

                watcher
                    .watch(watch_action)
                    .unwrap_or_else(move |err| eprintln!("{}", err));
            })
        })
        .collect();

    // loop all threads
    for handle in handles {
        let _ = handle.join();
    }
}
