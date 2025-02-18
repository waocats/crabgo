use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::path::{Path, PathBuf};
use std::task::Poll;

use crate::core::source::MaybePackage;
use crate::core::{Dependency, Package, PackageId, QueryKind, Source, SourceId, Summary};
use crate::sources::PathSource;
use crate::util::errors::CrabgoResult;
use crate::util::Config;

use anyhow::Context as _;
use crabgo_util::{paths, Sha256};
use serde::Deserialize;

pub struct DirectorySource<'cfg> {
    source_id: SourceId,
    root: PathBuf,
    packages: HashMap<PackageId, (Package, Checksum)>,
    config: &'cfg Config,
    updated: bool,
}

#[derive(Deserialize)]
struct Checksum {
    package: Option<String>,
    files: HashMap<String, String>,
}

impl<'cfg> DirectorySource<'cfg> {
    pub fn new(path: &Path, id: SourceId, config: &'cfg Config) -> DirectorySource<'cfg> {
        DirectorySource {
            source_id: id,
            root: path.to_path_buf(),
            config,
            packages: HashMap::new(),
            updated: false,
        }
    }
}

impl<'cfg> Debug for DirectorySource<'cfg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DirectorySource {{ root: {:?} }}", self.root)
    }
}

impl<'cfg> Source for DirectorySource<'cfg> {
    fn query(
        &mut self,
        dep: &Dependency,
        kind: QueryKind,
        f: &mut dyn FnMut(Summary),
    ) -> Poll<CrabgoResult<()>> {
        if !self.updated {
            return Poll::Pending;
        }
        let packages = self.packages.values().map(|p| &p.0);
        let matches = packages.filter(|pkg| match kind {
            QueryKind::Exact => dep.matches(pkg.summary()),
            QueryKind::Fuzzy => true,
        });
        for summary in matches.map(|pkg| pkg.summary().clone()) {
            f(summary);
        }
        Poll::Ready(Ok(()))
    }

    fn supports_checksums(&self) -> bool {
        true
    }

    fn requires_precise(&self) -> bool {
        true
    }

    fn source_id(&self) -> SourceId {
        self.source_id
    }

    fn block_until_ready(&mut self) -> CrabgoResult<()> {
        if self.updated {
            return Ok(());
        }
        self.packages.clear();
        let entries = self.root.read_dir().with_context(|| {
            format!(
                "failed to read root of directory source: {}",
                self.root.display()
            )
        })?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Ignore hidden/dot directories as they typically don't contain
            // crates and otherwise may conflict with a VCS
            // (rust-lang/crabgo#3414).
            if let Some(s) = path.file_name().and_then(|s| s.to_str()) {
                if s.starts_with('.') {
                    continue;
                }
            }

            // Vendor directories are often checked into a VCS, but throughout
            // the lifetime of a vendor dir crates are often added and deleted.
            // Some VCS implementations don't always fully delete the directory
            // when a dir is removed from a different checkout. Sometimes a
            // mostly-empty dir is left behind.
            //
            // Additionally vendor directories are sometimes accompanied with
            // readme files and other auxiliary information not too interesting
            // to Crabgo.
            //
            // To help handle all this we only try processing folders with a
            // `Crabgo.toml` in them. This has the upside of being pretty
            // flexible with the contents of vendor directories but has the
            // downside of accidentally misconfigured vendor directories
            // silently returning less crates.
            if !path.join("Crabgo.toml").exists() {
                continue;
            }

            let mut src = PathSource::new(&path, self.source_id, self.config);
            src.update()?;
            let mut pkg = src.root_package()?;

            let cksum_file = path.join(".crabgo-checksum.json");
            let cksum = paths::read(&path.join(cksum_file)).with_context(|| {
                format!(
                    "failed to load checksum `.crabgo-checksum.json` \
                     of {} v{}",
                    pkg.package_id().name(),
                    pkg.package_id().version()
                )
            })?;
            let cksum: Checksum = serde_json::from_str(&cksum).with_context(|| {
                format!(
                    "failed to decode `.crabgo-checksum.json` of \
                     {} v{}",
                    pkg.package_id().name(),
                    pkg.package_id().version()
                )
            })?;

            if let Some(package) = &cksum.package {
                pkg.manifest_mut()
                    .summary_mut()
                    .set_checksum(package.clone());
            }
            self.packages.insert(pkg.package_id(), (pkg, cksum));
        }

        self.updated = true;
        Ok(())
    }

    fn download(&mut self, id: PackageId) -> CrabgoResult<MaybePackage> {
        self.packages
            .get(&id)
            .map(|p| &p.0)
            .cloned()
            .map(MaybePackage::Ready)
            .ok_or_else(|| anyhow::format_err!("failed to find package with id: {}", id))
    }

    fn finish_download(&mut self, _id: PackageId, _data: Vec<u8>) -> CrabgoResult<Package> {
        panic!("no downloads to do")
    }

    fn fingerprint(&self, pkg: &Package) -> CrabgoResult<String> {
        Ok(pkg.package_id().version().to_string())
    }

    fn verify(&self, id: PackageId) -> CrabgoResult<()> {
        let (pkg, cksum) = match self.packages.get(&id) {
            Some(&(ref pkg, ref cksum)) => (pkg, cksum),
            None => anyhow::bail!("failed to find entry for `{}` in directory source", id),
        };

        for (file, cksum) in cksum.files.iter() {
            let file = pkg.root().join(file);
            let actual = Sha256::new()
                .update_path(&file)
                .with_context(|| format!("failed to calculate checksum of: {}", file.display()))?
                .finish_hex();
            if &*actual != cksum {
                anyhow::bail!(
                    "the listed checksum of `{}` has changed:\n\
                     expected: {}\n\
                     actual:   {}\n\
                     \n\
                     directory sources are not intended to be edited, if \
                     modifications are required then it is recommended \
                     that `[patch]` is used with a forked copy of the \
                     source\
                     ",
                    file.display(),
                    cksum,
                    actual
                );
            }
        }

        Ok(())
    }

    fn describe(&self) -> String {
        format!("directory source `{}`", self.root.display())
    }

    fn add_to_yanked_whitelist(&mut self, _pkgs: &[PackageId]) {}

    fn is_yanked(&mut self, _pkg: PackageId) -> Poll<CrabgoResult<bool>> {
        Poll::Ready(Ok(false))
    }

    fn invalidate_cache(&mut self) {
        // Directory source has no local cache.
    }

    fn set_quiet(&mut self, _quiet: bool) {
        // Directory source does not display status
    }
}
