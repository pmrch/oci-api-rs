use std::collections::{HashMap, HashSet};
use std::env::var;
use std::fmt::Display;

use compact_str::CompactString;

use crate::prelude::SecretsBuilder;

#[derive(Debug, Default)]
pub struct Secrets {
    image_id:  CompactString,
    subnet_id: CompactString,
    ssh_key:   CompactString,
    extra:     Option<HashMap<CompactString, CompactString>>,
}

impl Display for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Image id: {}\nSubnet id: {}\nSSH Key: {}",
            self.image_id, self.subnet_id, self.ssh_key
        )
    }
}

impl<'a> From<SecretsBuilder<'a>> for Secrets {
    fn from(builder: SecretsBuilder<'a>) -> Self {
        let mut secrets: Self = Self::default();

        if let Some(image_id) = builder.image_id {
            secrets.image_id = CompactString::from(image_id);
        }

        if let Some(subnet_id) = builder.subnet_id {
            secrets.subnet_id = CompactString::from(subnet_id);
        }

        if let Some(ssh_key) = builder.ssh_key {
            secrets.ssh_key = CompactString::from(ssh_key);
        }

        secrets
    }
}

impl<'a> From<&SecretsBuilder<'a>> for Secrets {
    fn from(builder: &SecretsBuilder<'a>) -> Self {
        let mut secrets: Self = Self::default();

        if let Some(image_id) = builder.image_id {
            secrets.image_id = CompactString::from(image_id);
        }

        if let Some(subnet_id) = builder.subnet_id {
            secrets.subnet_id = CompactString::from(subnet_id);
        }

        if let Some(ssh_key) = builder.ssh_key {
            secrets.ssh_key = CompactString::from(ssh_key);
        }

        secrets
    }
}

impl Secrets {
    /// Loads secrets from environment variables, reading a `.env` file if
    /// present.
    ///
    /// Not necessary to call `dotenv::dotenv()`, because it's done by this
    /// function too
    ///
    /// # Errors
    /// - `.env` file exists but cannot be parsed
    /// - Any of `IMAGE_OCID`, `SUBNET_OCID`, or `SSH_PUBLIC_KEY` are missing
    pub fn from_env(extra_keys: Option<HashSet<CompactString>>) -> anyhow::Result<Self> {
        dotenv::dotenv()?;
        let image_id: String = var("IMAGE_OCID")?;
        let subnet_id: String = var("SUBNET_OCID")?;
        let ssh_key: String = var("SSH_PUBLIC_KEY")?;

        let mut map: HashMap<CompactString, CompactString> = HashMap::new();
        if let Some(extra_keys) = extra_keys {
            for key in extra_keys {
                if let Ok(value) = var(&key) {
                    map.insert(key, CompactString::from(value));
                }
            }
        }

        Ok(Self {
            image_id:  CompactString::from(image_id),
            subnet_id: CompactString::from(subnet_id),
            ssh_key:   CompactString::from(ssh_key),
            extra:     Some(map),
        })
    }

    #[must_use]
    pub fn builder<'a>() -> SecretsBuilder<'a> { SecretsBuilder::default() }

    #[must_use]
    pub fn image_id(&self) -> &str { self.image_id.as_str() }

    #[must_use]
    pub fn subnet_id(&self) -> &str { self.subnet_id.as_str() }

    #[must_use]
    pub fn ssh_key(&self) -> &str { self.ssh_key.as_str() }

    #[must_use]
    pub const fn get_extra(&self) -> Option<&HashMap<CompactString, CompactString>> { self.extra.as_ref() }
}
