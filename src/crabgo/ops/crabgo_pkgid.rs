use crate::core::{PackageIdSpec, Workspace};
use crate::ops;
use crate::util::CrabgoResult;

pub fn pkgid(ws: &Workspace<'_>, spec: Option<&str>) -> CrabgoResult<PackageIdSpec> {
    let resolve = match ops::load_pkg_lockfile(ws)? {
        Some(resolve) => resolve,
        None => anyhow::bail!("a Crabgo.lock must exist for this command"),
    };

    let pkgid = match spec {
        Some(spec) => PackageIdSpec::query_str(spec, resolve.iter())?,
        None => ws.current()?.package_id(),
    };
    Ok(PackageIdSpec::from_package_id(pkgid))
}
