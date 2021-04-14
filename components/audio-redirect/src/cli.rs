//! CLI (command line interface).

use std::{fmt, path::PathBuf, str::FromStr as _};

use anyhow::anyhow;
use ephyr_log::slog;
use structopt::StructOpt;

/// CLI (command line interface) of the audio-redirect.
#[derive(Clone, Debug, StructOpt)]
#[structopt(about = "Audio redirect")]
pub struct Opts {
    /// Debug mode of the audio-redirect.
    #[structopt(short, long, help = "Enables debug mode")]
    pub debug: bool,

    /// Path to a file to capture audio from.
    #[structopt(
        long,
        env = "AUDIO_REDIRECT_INPUT_FILE",
        help = "Path to a file to capture audio from",
        long_help = "Path to a file to capture audio from. The supported formats are: [MP3, AAC, MP4]"
    )]
    pub input_file: Option<PathBuf>,

    /// Device input to capture audio from.
    #[structopt(
        long,
        env = "AUDIO_REDIRECT_INPUT_DEVICE",
        help = "Device input to capture audio from",
        long_help = "Device input to capture audio from. It could be microphone or virtual audio cable"
    )]
    pub input_device: Option<String>,

    /// Device output to play audio to.
    #[structopt(
        long,
        env = "AUDIO_REDIRECT_OUTPUT_DEVICE",
        help = "Device output to play audio to",
        long_help = "Device output to play audio to. It could be speaker or virtual audio cable"
    )]
    pub output_device: Option<String>,

    /// Display all input and output devices.
    #[structopt(
        long,
        help = "Display all input and output devices",
        long_help = "Display all input and output devices"
    )]
    pub show_devices: Option<bool>,

    /// Display all input devices.
    #[structopt(
        long,
        help = "Display all input devices",
        long_help = "Display all input devices"
    )]
    pub show_input_devices: Option<bool>,
    /// Display all output devices.
    #[structopt(
        long,
        help = "Display all output devices",
        long_help = "Display all output devices"
    )]
    pub show_output_devices: Option<bool>,
    /// Verbosity level of the audio-resirct logs.
    #[structopt(
    short,
    long,
    parse(try_from_str = Self::parse_log_level),
    help = "Logs verbosity level: \
                OFF | CRIT | ERRO | WARN | INFO | DEBG | TRCE"
    )]
    pub verbose: Option<slog::Level>,
    // /// Path to [FFmpeg] binary.
    // ///
    // /// [FFmpeg]: https://ffmpeg.org
    // #[structopt(
    //     short,
    //     long,
    //     env = "FFMPEG_PATH",
    //     default_value = "/usr/local/bin/ffmpeg",
    //     help = "Path to FFmpeg binary",
    //     long_help = "Path to FFmpeg binary"
    // )]
    // pub ffmpeg_path: PathBuf,
}

impl Opts {
    /// Parses CLI [`Opts`] from command line arguments.
    ///
    /// Prints the error message and quits the program in case of failure.
    #[inline]
    #[must_use]
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }

    /// Parses [`slog::Level`] from the given string.
    ///
    /// This function is required, because [`slog::Level`]'s [`FromStr`]
    /// implementation returns `()`, which is not [`Display`] as [`StructOpt`]
    /// requires.
    ///
    /// # Errors
    ///
    /// If [`slog::Level`] failed to parse from the string.
    ///
    /// [`Display`]: std::fmt::Display
    /// [`FromStr`]: std::str::FromStr
    pub fn parse_log_level(lvl: &str) -> Result<slog::Level, anyhow::Error> {
        #[allow(clippy::map_err_ignore)]
        slog::Level::from_str(lvl).map_err(|_| {
            anyhow!(
                "'{}' is invalid verbosity level, allowed levels are: \
                 OFF | CRIT | ERRO | WARN | INFO | DEBG | TRCE",
                lvl,
            )
        })
    }
}

/// Error type indicating non-zero process exit code.
pub struct Failure;

impl fmt::Debug for Failure {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl From<()> for Failure {
    #[inline]
    fn from(_: ()) -> Self {
        Self
    }
}
