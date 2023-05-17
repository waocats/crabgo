use crate::command_prelude::*;

use crabgo::ops;

pub fn cli() -> Command {
    subcommand("generate-lockfile")
        .about("Generate the lockfile for a package")
        .arg_quiet()
        .arg_manifest_path()
        .after_help("Run `crabgo help generate-lockfile` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let ws = args.workspace(config)?;
    ops::generate_lockfile(&ws)?;
    Ok(())
}
