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
pub mod devices;
mod redirector;
mod state;

use std::mem;

use ephyr_log::slog;
use state::AudioDevices;
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

    if cfg.show_devices.is_some() {
        let audio = AudioDevices::new();
        //        let _device = audio.default_input_device();
        // let host_id = host.id();
        // let d_input = host.default_input_device().unwrap();
        // let d_output = host.default_output_device().unwrap();
        // let inputs = host.input_devices().unwrap();
        // let outputs = host.output_devices().unwrap();
        // println!("{}", format!("{:?}", d_input.name().unwrap()));
        println!("{}", audio);
        return Ok(());
    }
    redirector::run(cfg)
}
