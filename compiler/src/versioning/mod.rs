use anyhow::Result;
use semver::Version;

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: Version,
    pub tier: Tier,
    pub build: u32,
    pub commit: String,
    pub date: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Alpha,
    Beta,
    Rc,
    Stable,
}

impl VersionInfo {
    pub fn from_cargo() -> Self {
        let version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(Version::new(0, 0, 0));
        Self {
            version,
            tier: Tier::Alpha,
            build: 0,
            commit: env!("VERGEN_GIT_SHA").to_string(),
            date: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        }
    }

    pub fn display(&self) -> String {
        let tier_str = match self.tier {
            Tier::Alpha => "alpha",
            Tier::Beta => "beta",
            Tier::Rc => "rc",
            Tier::Stable => "",
        };
        if tier_str.is_empty() {
            format!("{}", self.version)
        } else {
            format!("{}-{}.{}", self.version, tier_str, self.build)
        }
    }
}