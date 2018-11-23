use crate::{
    environment as e,
    system::SystemInput,
    unit::{self, SystemUnit},
};
use failure::{format_err, Error};
use serde_derive::Deserialize;
use std::collections::{BTreeSet, HashSet};

/// Builds one unit for every directory and file that needs to be copied.
system_struct! {
    InstallPackages {
        #[doc="Hierarchy key to lookup for packages to install."]
        #[serde(default = "default_key")]
        pub key: String,
        #[doc="Package provider to use."]
        pub provider: Option<String>,
    }
}

/// Default key to look up for installing packages.
fn default_key() -> String {
    String::from("packages")
}

impl InstallPackages {
    /// Copy one directory to another.
    pub fn apply<E>(&self, input: SystemInput<E>) -> Result<Vec<SystemUnit>, Error>
    where
        E: Copy + e::Environment,
    {
        let SystemInput {
            packages,
            data,
            allocator,
            state,
            ..
        } = input;

        let mut units = Vec::new();

        let provider = self.provider.as_ref();

        let id = self
            .id
            .as_ref()
            .map(|id| id.to_string())
            .or_else(|| provider.map(|id| id.to_string()))
            .or_else(|| packages.default().map(|p| p.name().to_string()))
            .ok_or_else(|| format_err!("no usable package `id`"))?;

        let mut all_packages = BTreeSet::new();

        let key = match provider {
            Some(provider) => format!("{}::{}", provider, self.key),
            None => self.key.to_string(),
        };

        all_packages.extend(data.load_or_default::<Vec<String>>(&key)?);

        // test if stored hash is stale.
        if state.is_hash_fresh(&id, &all_packages)? {
            log::trace!("Skipping `{}` since hash is fresh", id);
            return Ok(units);
        }

        let package_manager = match provider {
            Some(provider) => packages.get(provider)?,
            None => packages.default(),
        };

        let package_manager = match package_manager {
            Some(package_manager) => package_manager,
            None => {
                if !all_packages.is_empty() {
                    return Ok(units);
                }

                // warn, because we have packages that we want to install but can't since there is
                // no package manager.
                match provider {
                    Some(provider) => {
                        log::warn!("No package manager for provider `{}` found", provider)
                    }
                    None => log::warn!("No primary package manager found"),
                }

                return Ok(units);
            }
        };

        let mut to_install = all_packages.iter().cloned().collect::<HashSet<_>>();

        for package in package_manager.list_packages()? {
            to_install.remove(&package.name);
        }

        let to_install = to_install.into_iter().collect();

        // thread-local if package manager requires user interaction.
        let thread_local = package_manager.needs_interaction();

        let mut unit = allocator.unit(unit::InstallPackages {
            package_manager,
            all_packages,
            to_install,
            id,
        });

        // NB: sometimes requires user input.
        unit.thread_local = thread_local;
        units.push(unit);
        return Ok(units);
    }
}
