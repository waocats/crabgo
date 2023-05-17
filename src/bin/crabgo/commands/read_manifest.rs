use crate::command_prelude::*;

pub fn cli() -> Command {
    subcommand("read-manifest")
        .about(
            "\
Print a JSON representation of a Crabgo.toml manifest.

Deprecated, use `crabgo metadata --no-deps` instead.\
",
        )
        .arg_quiet()
        .arg_manifest_path()
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let ws = args.workspace(config)?;
    config.shell().print_json(&ws.current()?.serialized())?;
    Ok(())
}
