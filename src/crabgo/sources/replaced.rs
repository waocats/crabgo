use crate::core::source::MaybePackage;
use crate::core::{Dependency, Package, PackageId, QueryKind, Source, SourceId, Summary};
use crate::util::errors::CrabgoResult;
use std::task::Poll;

use anyhow::Context as _;

pub struct ReplacedSource<'cfg> {
    to_replace: SourceId,
    replace_with: SourceId,
    inner: Box<dyn Source + 'cfg>,
}

impl<'cfg> ReplacedSource<'cfg> {
    pub fn new(
        to_replace: SourceId,
        replace_with: SourceId,
        src: Box<dyn Source + 'cfg>,
    ) -> ReplacedSource<'cfg> {
        ReplacedSource {
            to_replace,
            replace_with,
            inner: src,
        }
    }
}

impl<'cfg> Source for ReplacedSource<'cfg> {
    fn source_id(&self) -> SourceId {
        self.to_replace
    }

    fn replaced_source_id(&self) -> SourceId {
        self.replace_with
    }

    fn supports_checksums(&self) -> bool {
        self.inner.supports_checksums()
    }

    fn requires_precise(&self) -> bool {
        self.inner.requires_precise()
    }

    fn query(
        &mut self,
        dep: &Dependency,
        kind: QueryKind,
        f: &mut dyn FnMut(Summary),
    ) -> Poll<CrabgoResult<()>> {
        let (replace_with, to_replace) = (self.replace_with, self.to_replace);
        let dep = dep.clone().map_source(to_replace, replace_with);

        self.inner
            .query(&dep, kind, &mut |summary| {
                f(summary.map_source(replace_with, to_replace))
            })
            .map_err(|e| {
                e.context(format!(
                    "failed to query replaced source {}",
                    self.to_replace
                ))
            })
    }

    fn invalidate_cache(&mut self) {
        self.inner.invalidate_cache()
    }

    fn set_quiet(&mut self, quiet: bool) {
        self.inner.set_quiet(quiet);
    }

    fn download(&mut self, id: PackageId) -> CrabgoResult<MaybePackage> {
        let id = id.with_source_id(self.replace_with);
        let pkg = self
            .inner
            .download(id)
            .with_context(|| format!("failed to download replaced source {}", self.to_replace))?;
        Ok(match pkg {
            MaybePackage::Ready(pkg) => {
                MaybePackage::Ready(pkg.map_source(self.replace_with, self.to_replace))
            }
            other @ MaybePackage::Download { .. } => other,
        })
    }

    fn finish_download(&mut self, id: PackageId, data: Vec<u8>) -> CrabgoResult<Package> {
        let id = id.with_source_id(self.replace_with);
        let pkg = self
            .inner
            .finish_download(id, data)
            .with_context(|| format!("failed to download replaced source {}", self.to_replace))?;
        Ok(pkg.map_source(self.replace_with, self.to_replace))
    }

    fn fingerprint(&self, id: &Package) -> CrabgoResult<String> {
        self.inner.fingerprint(id)
    }

    fn verify(&self, id: PackageId) -> CrabgoResult<()> {
        let id = id.with_source_id(self.replace_with);
        self.inner.verify(id)
    }

    fn describe(&self) -> String {
        if self.replace_with.is_crates_io() && self.to_replace.is_crates_io() {
            // Built-in source replacement of crates.io for sparse registry or tests
            // doesn't need duplicate description (crates.io replacing crates.io).
            self.inner.describe()
        } else {
            format!(
                "{} (which is replacing {})",
                self.inner.describe(),
                self.to_replace
            )
        }
    }

    fn is_replaced(&self) -> bool {
        true
    }

    fn add_to_yanked_whitelist(&mut self, pkgs: &[PackageId]) {
        let pkgs = pkgs
            .iter()
            .map(|id| id.with_source_id(self.replace_with))
            .collect::<Vec<_>>();
        self.inner.add_to_yanked_whitelist(&pkgs);
    }

    fn is_yanked(&mut self, pkg: PackageId) -> Poll<CrabgoResult<bool>> {
        self.inner.is_yanked(pkg)
    }

    fn block_until_ready(&mut self) -> CrabgoResult<()> {
        self.inner
            .block_until_ready()
            .with_context(|| format!("failed to update replaced source {}", self.to_replace))
    }
}
