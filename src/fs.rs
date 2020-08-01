use anyhow::Context as _;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

pub(crate) fn read_to_string(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).with_context(|| format!("could not read `{}`", path.display()))
}

pub(crate) fn read_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> anyhow::Result<T> {
    let path = path.as_ref();
    serde_json::from_str(&read_to_string(path)?)
        .with_context(|| format!("could not parse the JSON file at `{}`", path.display()))
}

pub(crate) fn read_toml<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> anyhow::Result<T> {
    let path = path.as_ref();
    toml::from_str(&read_to_string(path)?).with_context(|| {
        format!(
            "could not parse the {} at `{}`",
            if path.file_name() == Some("Cargo.toml".as_ref()) {
                "manifest"
            } else {
                "TOML file"
            },
            path.display()
        )
    })
}

pub(crate) fn read_toml_preserving<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> anyhow::Result<(T, toml_edit::Document)> {
    let path = path.as_ref();
    let content = read_to_string(path)?;
    let value = toml::from_str(&content).with_context(|| {
        format!(
            "could not parse the {} at `{}`",
            if path.file_name() == Some("Cargo.toml".as_ref()) {
                "manifest"
            } else {
                "TOML file"
            },
            path.display()
        )
    })?;
    let edit = content.parse()?;
    Ok((value, edit))
}

pub(crate) fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> anyhow::Result<()> {
    let path = path.as_ref();
    std::fs::write(path, content).with_context(|| format!("could not write `{}`", path.display()))
}

pub(crate) fn write_json(path: impl AsRef<Path>, content: impl Serialize) -> anyhow::Result<()> {
    write(path, serde_json::to_string(&content)?)
}

pub(crate) fn remove_file(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    std::fs::remove_file(path).with_context(|| format!("could not remove `{}`", path.display()))
}

pub(crate) fn create_dir_all(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).with_context(|| format!("could not create `{}`", path.display()))
}
