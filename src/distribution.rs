use std::str::FromStr;

use crate::settings::UrlOrPath;

#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Distribution {
    Nix,
}

impl Distribution {
    pub fn tarball_location_or(&self, user_preference: &Option<UrlOrPath>) -> TarballLocation {
        if let Some(pref) = user_preference {
            return TarballLocation::UrlOrPath(pref.clone());
        }

        self.tarball_location()
    }

    pub fn tarball_location(&self) -> TarballLocation {
        TarballLocation::UrlOrPath(
            UrlOrPath::from_str(NIX_TARBALL_URL)
                .expect("Fault: the built-in Nix tarball URL does not parse."),
        )
    }
}

pub enum TarballLocation {
    UrlOrPath(UrlOrPath),
}

pub const NIX_TARBALL_URL: &str = env!("NIX_TARBALL_URL");

