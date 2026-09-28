
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
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
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            tier: Tier::Alpha,
            build: 0,
            commit: std::env::var("VERGEN_GIT_SHA").unwrap_or_else(|_| "unknown".to_string()),
            date: std::env::var("VERGEN_BUILD_TIMESTAMP").unwrap_or_else(|_| chrono::Utc::now().to_rfc3339()),
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