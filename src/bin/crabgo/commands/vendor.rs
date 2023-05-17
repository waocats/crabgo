use crate::command_prelude::*;
use crabgo::ops;
use std::path::PathBuf;

pub fn cli() -> Command {
    subcommand("vendor")
        .about("Vendor all dependencies for a project locally")
        .arg_quiet()
        .arg_manifest_path()
        .arg(
            Arg::new("path")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Where to vendor crates (`vendor` by default)"),
        )
        .arg(flag(
            "no-delete",
            "Don't delete older crates in the vendor directory",
        ))
        .arg(
            Arg::new("tomls")
                .short('s')
                .long("sync")
                .help("Additional `Crabgo.toml` to sync and vendor")
                .value_name("TOML")
                .value_parser(clap::value_parser!(PathBuf))
                .action(clap::ArgAction::Append),
        )
        .arg(flag(
            "respect-source-config",
            "Respect `[source]` config in `.crabgo/config`",
        ))
        .arg(flag(
            "versioned-dirs",
            "Always include version in subdir name",
        ))
        .arg(flag("no-merge-sources", "Not supported").hide(true))
        .arg(flag("relative-path", "Not supported").hide(true))
        .arg(flag("only-git-deps", "Not supported").hide(true))
        .arg(flag("disallow-duplicates", "Not supported").hide(true))
        .after_help("Run `crabgo help vendor` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    // We're doing the vendoring operation ourselves, so we don't actually want
    // to respect any of the `source` configuration in Crabgo itself. That's
    // intended for other consumers of Crabgo, but we want to go straight to the
    // source, e.g. crates.io, to fetch crates.
    if !args.flag("respect-source-config") {
        config.values_mut()?.remove("source");
    }

    // When we moved `crabgo vendor` into Crabgo itself we didn't stabilize a few
    // flags, so try to provide a helpful error message in that case to ensure
    // that users currently using the flag aren't tripped up.
    let crates_io_crabgo_vendor_flag = if args.flag("no-merge-sources") {
        Some("--no-merge-sources")
    } else if args.flag("relative-path") {
        Some("--relative-path")
    } else if args.flag("only-git-deps") {
        Some("--only-git-deps")
    } else if args.flag("disallow-duplicates") {
        Some("--disallow-duplicates")
    } else {
        None
    };
    if let Some(flag) = crates_io_crabgo_vendor_flag {
        return Err(anyhow::format_err!(
            "\
the crates.io `crabgo vendor` command has now been merged into Crabgo itself
and does not support the flag `{}` currently; to continue using the flag you
can execute `crabgo-vendor vendor ...`, and if you would like to see this flag
supported in Crabgo itself please feel free to file an issue at
https://github.com/rust-lang/crabgo/issues/new
",
            flag
        )
        .into());
    }

    let ws = args.workspace(config)?;
    let path = args
        .get_one::<PathBuf>("path")
        .cloned()
        .unwrap_or_else(|| PathBuf::from("vendor"));
    ops::vendor(
        &ws,
        &ops::VendorOptions {
            no_delete: args.flag("no-delete"),
            destination: &path,
            versioned_dirs: args.flag("versioned-dirs"),
            extra: args
                .get_many::<PathBuf>("tomls")
                .unwrap_or_default()
                .cloned()
                .collect(),
        },
    )?;
    Ok(())
}
