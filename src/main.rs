use eyre::ContextCompat;
use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::sync::Mutex;

mod cli;

fn main() -> Result<(), eyre::Report> {
    let matches = cli::build()?.get_matches();

    // Generate CLI completions if prompted, then exit.
    if let Some(sub_matches) = matches.subcommand_matches("completions") {
        let shell = sub_matches
            .get_one::<clap_complete_command::Shell>("shell")
            .wrap_err("No way, I failed to get the shell. #freak accident")?;

        let mut cli = cli::build()?;

        shell.generate(&mut cli, &mut std::io::stdout());

        std::process::exit(0);
    }

    let percent = matches.get_one::<u32>("percent").unwrap();

    let mut walk = walkdir::WalkDir::new(matches.get_one::<String>("root").unwrap());

    if matches.get_flag("norec") {
        walk = walk.max_depth(1);
    } else if let Some(&depth) = matches.get_one::<usize>("depth") {
        walk = walk.max_depth(depth);
    }

    let files = Mutex::new(Vec::new());

    let include_dirs = matches.get_flag("dirs");

    walk.into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .into_par_iter()
        .filter(|de| include_dirs || de.metadata().unwrap().is_file())
        .for_each(|de| files.lock().unwrap().push(de));

    let mut rng = rand::rng();

    let (choose_n, total) = {
        let mut files = files.lock().unwrap();

        files.shuffle(&mut rng);

        // I can't be bothered
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let choose_n = matches.get_one::<usize>("number").map_or_else(
            || (files.len() as f32 / (100_f32 / *percent as f32).ceil()) as usize,
            |n| *n.min(&files.len()),
        );

        for f in files.iter().take(choose_n) {
            println!("{}", f.path().display());
        }

        (choose_n, files.len())
    };

    let percent = choose_n * 100 / total;
    eprintln!("Selected {choose_n} out of {total} files ({percent}%)");

    Ok(())
}
