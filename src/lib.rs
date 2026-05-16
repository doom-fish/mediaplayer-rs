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
pub mod content_item;
pub mod error;
pub mod ffi;
pub mod media_item;
pub mod media_item_collection;
pub mod media_library;
pub mod media_playlist;
pub mod media_query;
pub mod music_player;
pub mod now_playing;
pub mod playable_content_data_source;
pub mod remote_commands;
pub mod system_music_player;
pub mod volume_view;

mod unsupported;

pub use artwork::{AnimatedArtwork, Artwork};
pub use content_item::ContentItem;
pub use error::{ErrorCode, MediaPlayerError, ERROR_DOMAIN};
pub use media_item::{MediaEntityPersistentId, MediaItem, MediaType};
pub use media_item_collection::MediaItemCollection;
pub use media_library::MediaLibrary;
pub use media_playlist::MediaPlaylist;
pub use media_query::MediaQuery;
pub use music_player::MusicPlayer;
pub use now_playing::{
    LanguageOption, LanguageOptionGroup, LanguageOptionType, NowPlayingInfo, NowPlayingInfoCenter,
    NowPlayingMediaType, PlaybackState,
};
pub use playable_content_data_source::PlayableContentDataSource;
pub use remote_commands::{
    ChangePlaybackPositionCommand, ChangePlaybackRateCommand, ChangeRepeatModeCommand,
    ChangeShuffleModeCommand, Command, CommandEvent, CommandToken, FeedbackCommand, HandlerStatus,
    LanguageOptionSetting, RatingCommand, RemoteCommand, RemoteCommandCenter, RepeatType, SeekType,
    ShuffleType, SkipIntervalCommand,
};
pub use system_music_player::SystemMusicPlayer;
pub use volume_view::VolumeView;

pub mod prelude {
    pub use crate::artwork::{AnimatedArtwork, Artwork};
    pub use crate::content_item::ContentItem;
    pub use crate::error::{ErrorCode, MediaPlayerError, ERROR_DOMAIN};
    pub use crate::media_item::{MediaEntityPersistentId, MediaItem, MediaType};
    pub use crate::media_item_collection::MediaItemCollection;
    pub use crate::media_library::MediaLibrary;
    pub use crate::media_playlist::MediaPlaylist;
    pub use crate::media_query::MediaQuery;
    pub use crate::music_player::MusicPlayer;
    pub use crate::now_playing::{
        LanguageOption, LanguageOptionGroup, LanguageOptionType, NowPlayingInfo,
        NowPlayingInfoCenter, NowPlayingMediaType, PlaybackState,
    };
    pub use crate::playable_content_data_source::PlayableContentDataSource;
    pub use crate::remote_commands::{
        ChangePlaybackPositionCommand, ChangePlaybackRateCommand, ChangeRepeatModeCommand,
        ChangeShuffleModeCommand, Command, CommandEvent, CommandToken, FeedbackCommand,
        HandlerStatus, LanguageOptionSetting, RatingCommand, RemoteCommand, RemoteCommandCenter,
        RepeatType, SeekType, ShuffleType, SkipIntervalCommand,
    };
    pub use crate::system_music_player::SystemMusicPlayer;
    pub use crate::volume_view::VolumeView;
}
