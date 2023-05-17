mod xtask;

fn main() {
    env_logger::init_from_env("CRABGO_LOG");
    let cli = xtask::cli();
    let matches = cli.get_matches();

    let mut config = crabgo::util::config::Config::default().unwrap_or_else(|e| {
        let mut eval = crabgo::core::shell::Shell::new();
        crabgo::exit_with_error(e.into(), &mut eval)
    });
    if let Err(e) = xtask::exec(&matches, &mut config) {
        crabgo::exit_with_error(e, &mut config.shell())
    }
}
