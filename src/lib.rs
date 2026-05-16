#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod artwork;
pub mod constants;
pub mod error;
pub mod ffi;
pub mod now_playing;
pub mod remote_commands;

pub use artwork::Artwork;
pub use error::MediaPlayerError;
pub use now_playing::{NowPlayingInfo, NowPlayingInfoCenter, NowPlayingMediaType, PlaybackState};
pub use remote_commands::{
    Command, CommandEvent, CommandToken, HandlerStatus, RemoteCommandCenter, SeekType,
};

pub mod prelude {
    pub use crate::artwork::Artwork;
    pub use crate::error::MediaPlayerError;
    pub use crate::now_playing::{
        NowPlayingInfo, NowPlayingInfoCenter, NowPlayingMediaType, PlaybackState,
    };
    pub use crate::remote_commands::{
        Command, CommandEvent, CommandToken, HandlerStatus, RemoteCommandCenter, SeekType,
    };
}
