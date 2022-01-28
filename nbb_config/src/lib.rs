//! `nbb_config`
#![feature(once_cell)]
#![deny(missing_docs)]

#[macro_use]
extern crate serde;

mod cfg_storage;
mod global_cfg;
mod load_cfg;
mod markdown_cfg;
mod post_cfg;

pub use cfg_storage::get_config;
pub use global_cfg::*;
pub use load_cfg::*;
pub use markdown_cfg::*;
pub use post_cfg::*;
