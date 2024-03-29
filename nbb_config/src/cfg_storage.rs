use crate::GlobalConfig;
use once_cell::sync::OnceCell;

static APP_CONFIG: OnceCell<GlobalConfig> = OnceCell::new();

#[inline]
pub fn set_config(config: GlobalConfig<'static>) -> Result<(), ()> {
    APP_CONFIG.set(config).map_err(|_| ())
}

/// Get the global config object.
///
/// # Panics
/// This function panics if this is called before the config is loaded.
#[inline]
#[must_use]
pub fn get_config<'a>() -> &'a GlobalConfig<'a> {
    APP_CONFIG
        .get()
        .expect("called `get_config` before config loaded")
}
