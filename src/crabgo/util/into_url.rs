use std::path::{Path, PathBuf};

use url::Url;

use crate::util::CrabgoResult;

/// A type that can be converted to a Url
pub trait IntoUrl {
    /// Performs the conversion
    fn into_url(self) -> CrabgoResult<Url>;
}

impl<'a> IntoUrl for &'a str {
    fn into_url(self) -> CrabgoResult<Url> {
        Url::parse(self).map_err(|s| anyhow::format_err!("invalid url `{}`: {}", self, s))
    }
}

impl<'a> IntoUrl for &'a Path {
    fn into_url(self) -> CrabgoResult<Url> {
        Url::from_file_path(self)
            .map_err(|()| anyhow::format_err!("invalid path url `{}`", self.display()))
    }
}

impl<'a> IntoUrl for &'a PathBuf {
    fn into_url(self) -> CrabgoResult<Url> {
        self.as_path().into_url()
    }
}
