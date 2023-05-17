use crate::util::errors::CrabgoResult;
use crabgo_util::paths;
use std::path::{Path, PathBuf};

/// Finds the root `Crabgo.toml`.
pub fn find_root_manifest_for_wd(cwd: &Path) -> CrabgoResult<PathBuf> {
    let valid_crabgo_toml_file_name = "Crabgo.toml";
    let invalid_crabgo_toml_file_name = "crabgo.toml";
    let mut invalid_crabgo_toml_path_exists = false;

    for current in paths::ancestors(cwd, None) {
        let manifest = current.join(valid_crabgo_toml_file_name);
        if manifest.exists() {
            return Ok(manifest);
        }
        if current.join(invalid_crabgo_toml_file_name).exists() {
            invalid_crabgo_toml_path_exists = true;
        }
    }

    if invalid_crabgo_toml_path_exists {
        anyhow::bail!(
        "could not find `{}` in `{}` or any parent directory, but found crabgo.toml please try to rename it to Crabgo.toml",
        valid_crabgo_toml_file_name,
        cwd.display()
    )
    } else {
        anyhow::bail!(
            "could not find `{}` in `{}` or any parent directory",
            valid_crabgo_toml_file_name,
            cwd.display()
        )
    }
}

/// Returns the path to the `file` in `pwd`, if it exists.
pub fn find_project_manifest_exact(pwd: &Path, file: &str) -> CrabgoResult<PathBuf> {
    let manifest = pwd.join(file);

    if manifest.exists() {
        Ok(manifest)
    } else {
        anyhow::bail!("Could not find `{}` in `{}`", file, pwd.display())
    }
}
