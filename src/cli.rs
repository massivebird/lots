use clap::{Arg, ArgAction, value_parser};
use std::sync::OnceLock;

// whatever dude
static HOME_DIR: OnceLock<String> = OnceLock::new();

pub fn build() -> Result<clap::Command, eyre::Report> {
    HOME_DIR.set(std::env::var("HOME")?).unwrap();

    Ok(clap::command!()
        .args_conflicts_with_subcommands(true)
        .about("Returns the absolute paths of a randomly selected portion of local files.")
        .subcommand(
            clap::Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .value_name("shell")
                        .value_parser(
                            clap::builder::EnumValueParser::<clap_complete_command::Shell>::new(),
                        ),
                ),
        )
        .next_help_heading("Positional arguments")
        .args([Arg::new("root")
            .value_name("PATH")
            .help("Where to begin (recursively) collecting files.")
            .required(false)
            .value_hint(clap::ValueHint::DirPath)
            .default_value(clap::builder::OsStr::from(HOME_DIR.get().unwrap().as_str()))])
        .next_help_heading("Selection options")
        .args([
            Arg::new("percent")
                .short('p')
                .long("percent")
                .value_name("0..=100")
                .default_value("50")
                .value_parser(value_parser!(u32))
                .help("Integer percentage of files to select."),
            Arg::new("number")
                .short('n')
                .conflicts_with("percent")
                .long("number")
                .value_name("value")
                .value_parser(value_parser!(usize))
                .help("Exact number of files to select."),
        ])
        .next_help_heading("Filtering options")
        .args([
            Arg::new("dirs")
                .short('d')
                .long("include-dirs")
                .help("Collect directories in addition to normal files.")
                .action(ArgAction::SetTrue),
            Arg::new("depth")
                .short('D')
                .long("max-depth")
                .value_name("value")
                .value_parser(value_parser!(usize))
                .help("Set a maximum recursion depth."),
            Arg::new("norec")
                .short('N')
                .long("no-recursion")
                .conflicts_with("depth")
                .action(ArgAction::SetTrue),
        ]))
}
