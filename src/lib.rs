mod cli;
mod data_parser;
mod log;
// pub mod model;
// mod puppets;

use ::log::{error, LevelFilter};
use godot::{
    prelude::*,
};
use godot::classes::Os;
use godot::global::Error;

pub use log::Log;

/// Helper struct for information about the libvpuppr library.
#[derive(Debug, Default, GodotClass)]
#[class(base = RefCounted)]
struct LibVpuppr;

#[godot_api]
impl IRefCounted for LibVpuppr {
    fn init(_base: Base<RefCounted>) -> Self {
        Self
    }
}

#[godot_api]
impl LibVpuppr {
    /// Parse user args and return a [Dictionary] containing all args found.
    #[func]
    fn parse_user_args() -> Dictionary {
        let godot_user_args = Os::singleton()
            .get_cmdline_user_args()
            .as_slice()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        // Argh requires a &[&str] and there isn't an easy way of doing that, so here we are
        match cli::Args::parse(
            godot_user_args
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        ) {
            Ok(v) => v.to_dict(),
            Err(e) => {
                error!("{e}");
                Dictionary::new()
            }
        }
    }

    /// Initialize logging of Rust libraries.
    ///
    /// # Note
    /// A new [String] must be allocated when printing, otherwise Godot is not
    /// able to print anything.
    #[func]
    fn init_rust_log(quiet: bool, verbose: bool) -> Error {
        match youlog::Youlog::new_from_default_env()
            .global_level(if quiet {
                LevelFilter::Error
            } else if verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            })
            .log_fn(LevelFilter::Info, |r| {
                Log::global(LevelFilter::Info, r.target(), r.args().to_string().as_str());
            })
            .log_fn(LevelFilter::Warn, |r| {
                Log::global(LevelFilter::Warn, r.target(), r.args().to_string().as_str());
            })
            .log_fn(LevelFilter::Error, |r| {
                Log::global(
                    LevelFilter::Error,
                    r.target(),
                    r.args().to_string().as_str(),
                );
            })
            .log_fn(LevelFilter::Debug, |r| {
                Log::global(
                    LevelFilter::Debug,
                    r.target(),
                    r.args().to_string().as_str(),
                );
            })
            .init()
        {
            Ok(_) => Error::OK,
            Err(_) => Error::ERR_UNCONFIGURED,
        }
    }

    /// A mapping of various vpuppr metadata.
    #[func]
    fn metadata() -> Dictionary {
        let mut mapping = Dictionary::new();

        let is_debug = if cfg!(debug_assertions) { true } else { false };
        let _ = mapping.insert("DEBUG", is_debug);
        let _ = mapping.insert("RELEASE", !is_debug);

        let _ = mapping.insert("VERSION", env!("CARGO_PKG_VERSION"));
        let _ = mapping.insert("VERSION_MAJOR", env!("CARGO_PKG_VERSION_MAJOR"));
        let _ = mapping.insert("VERSION_MINOR", env!("CARGO_PKG_VERSION_MINOR"));
        let _ = mapping.insert("VERSION_PATCH", env!("CARGO_PKG_VERSION_PATCH"));

        let _ = mapping.insert("LIBVPUPPR_AUTHORS", env!("CARGO_PKG_AUTHORS"));

        mapping
    }
}

struct GodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}
