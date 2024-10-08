//! `mp4` is a Rust library to read and write ISO-MP4 files.
//!
//! This package contains MPEG-4 specifications defined in parts:
//!    * ISO/IEC 14496-12 - ISO Base Media File Format (QuickTime, MPEG-4, etc)
//!    * ISO/IEC 14496-14 - MP4 file format
//!    * ISO/IEC 14496-17 - Streaming text format
//!

use std::io::Cursor;

mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

mod types;
pub use types::*;

mod mp4box;
pub use mp4box::*;

mod reader;
pub use reader::Mp4;

pub use types::TrackKind;

pub fn read(bytes: &[u8]) -> Result<Mp4> {
    let mp4 = reader::Mp4::read(Cursor::new(bytes), bytes.len() as u64)?;
    Ok(mp4)
}
