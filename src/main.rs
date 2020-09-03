use clap_verbosity_flag;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

// check https://github.com/garbas/ynab-sync/blob/master/src/logging.rs for an example of using
// clap_verbosity_flag to set up loggin
#[derive(StructOpt)]
struct Cli {
    /// the pattern to look for
    pattern: String,
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let f = std::fs::File::open(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let l = line.unwrap();
        saws::find_match(&l, &args.pattern, &mut std::io::stdout())
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
