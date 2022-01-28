use std::path::Path;

/// Load the config specified by `path`, and set the global config variable.
///
/// # Panics
/// This function panics if any of the following happen:
/// * reading the config at `path` failed
/// * the config at `path` is invalid TOML
pub fn load_config(path: impl AsRef<Path>) {
    let data = std::fs::read(path).expect("failed to load config: does it exist?");
    let cfg =
        serde_yaml::from_slice(&data[..]).expect("failed to load config as TOML: is it valid?");

    crate::cfg_storage::set_config(cfg).expect("don't call `load_config` more than once!");
}
