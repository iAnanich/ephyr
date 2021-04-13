//! Audio Redirect
#![deny(
    broken_intra_doc_links,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code
)]
#![warn(
    deprecated_in_future,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]

pub mod cli;
use std::mem;

use ephyr_log::slog;

/// Runs application.
///
/// # Errors
///
/// If running has failed and could not be performed. The appropriate error
/// is logged.
pub fn run() -> Result<(), cli::Failure> {
    let mut cfg = cli::Opts::from_args();
    cfg.verbose = cfg.verbose.or_else(|| {
        if cfg.debug {
            Some(slog::Level::Debug)
        } else {
            None
        }
    });

    // This guard should be held till the end of the program for the logger
    // to present in global context.
    mem::forget(ephyr_log::init(cfg.verbose));

    Ok(())
}
