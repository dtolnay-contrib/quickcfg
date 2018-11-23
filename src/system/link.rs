use crate::{
    environment as e, system::SystemInput, template::Template, unit::SystemUnit, FileUtils,
};
use failure::Error;
use serde_derive::Deserialize;

/// Builds one unit for every directory and file that needs to be copied.
system_struct! {
    Link {
        pub path: Template,
        pub link: Template,
    }
}

impl Link {
    /// Copy one directory to another.
    pub fn apply<E>(&self, input: SystemInput<E>) -> Result<Vec<SystemUnit>, Error>
    where
        E: Copy + e::Environment,
    {
        let SystemInput {
            root,
            base_dirs,
            facts,
            environment,
            file_utils,
            ..
        } = input;

        let mut units = Vec::new();

        let path = match self
            .path
            .render_as_path(root, base_dirs, facts, environment)?
        {
            Some(path) => path,
            None => return Ok(units),
        };

        let link = match self
            .link
            .render_as_path(root, base_dirs, facts, environment)?
        {
            Some(link) => link,
            None => return Ok(units),
        };

        let m = FileUtils::try_open_meta(&path)?;

        // try to relativize link.
        let link = if link.is_absolute() {
            match path
                .parent()
                .and_then(|p| FileUtils::path_relative_from(&link, p))
            {
                Some(link) => link,
                None => link,
            }
        } else {
            link
        };

        units.extend(file_utils.symlink(&path, link, m.as_ref())?);
        Ok(units)
    }
}