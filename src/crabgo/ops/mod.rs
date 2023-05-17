use crate::sources::CRATES_IO_DOMAIN;

pub use self::crabgo_clean::{clean, CleanOptions};
pub use self::crabgo_compile::{
    compile, compile_with_exec, compile_ws, create_bcx, print, resolve_all_features, CompileOptions,
};
pub use self::crabgo_compile::{CompileFilter, FilterRule, LibRule, Packages};
pub use self::crabgo_doc::{doc, DocOptions};
pub use self::crabgo_fetch::{fetch, FetchOptions};
pub use self::crabgo_generate_lockfile::generate_lockfile;
pub use self::crabgo_generate_lockfile::update_lockfile;
pub use self::crabgo_generate_lockfile::UpdateOptions;
pub use self::crabgo_install::{install, install_list};
pub use self::crabgo_new::{init, new, NewOptions, NewProjectKind, VersionControl};
pub use self::crabgo_output_metadata::{output_metadata, ExportInfo, OutputMetadataOptions};
pub use self::crabgo_package::{check_yanked, package, package_one, PackageOpts};
pub use self::crabgo_pkgid::pkgid;
pub use self::crabgo_read_manifest::{read_package, read_packages};
pub use self::crabgo_run::run;
pub use self::crabgo_test::{run_benches, run_tests, TestOptions};
pub use self::crabgo_uninstall::uninstall;
pub use self::fix::{fix, fix_exec_rustc, fix_get_proxy_lock_addr, FixOptions};
pub use self::lockfile::{load_pkg_lockfile, resolve_to_string, write_pkg_lockfile};
pub use self::registry::HttpTimeout;
pub use self::registry::{configure_http_handle, http_handle, http_handle_and_timeout};
pub use self::registry::{modify_owners, yank, OwnersOptions, PublishOpts};
pub use self::registry::{needs_custom_http_transport, registry_login, registry_logout, search};
pub use self::registry::{publish, RegistryCredentialConfig};
pub use self::resolve::{
    add_overrides, get_resolved_packages, resolve_with_previous, resolve_ws, resolve_ws_with_opts,
    WorkspaceResolve,
};
pub use self::vendor::{vendor, VendorOptions};

pub mod crabgo_add;
mod crabgo_clean;
pub(crate) mod crabgo_compile;
pub mod crabgo_config;
mod crabgo_doc;
mod crabgo_fetch;
mod crabgo_generate_lockfile;
mod crabgo_install;
mod crabgo_new;
mod crabgo_output_metadata;
mod crabgo_package;
mod crabgo_pkgid;
mod crabgo_read_manifest;
pub mod crabgo_remove;
mod crabgo_run;
mod crabgo_test;
mod crabgo_uninstall;
mod common_for_install_and_uninstall;
mod fix;
pub(crate) mod lockfile;
pub(crate) mod registry;
pub(crate) mod resolve;
pub mod tree;
mod vendor;

/// Returns true if the dependency is either git or path, false otherwise
/// Error if a git/path dep is transitive, but has no version (registry source).
/// This check is performed on dependencies before publishing or packaging
fn check_dep_has_version(dep: &crate::core::Dependency, publish: bool) -> crate::CrabgoResult<bool> {
    let which = if dep.source_id().is_path() {
        "path"
    } else if dep.source_id().is_git() {
        "git"
    } else {
        return Ok(false);
    };

    if !dep.specified_req() && dep.is_transitive() {
        let dep_version_source = dep.registry_id().map_or_else(
            || CRATES_IO_DOMAIN.to_string(),
            |registry_id| registry_id.display_registry_name(),
        );
        anyhow::bail!(
            "all dependencies must have a version specified when {}.\n\
             dependency `{}` does not specify a version\n\
             Note: The {} dependency will use the version from {},\n\
             the `{}` specification will be removed from the dependency declaration.",
            if publish { "publishing" } else { "packaging" },
            dep.package_name(),
            if publish { "published" } else { "packaged" },
            dep_version_source,
            which,
        )
    }
    Ok(true)
}
