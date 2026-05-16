//! Wrapper for `MPRemoteCommandCenter` and associated command types.

use core::ffi::{c_char, c_double, c_int, c_void};
use std::ffi::CString;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::ffi::{self, MpCommandCallback};
use crate::now_playing::LanguageOption;
use crate::unsupported;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[non_exhaustive]
/// Remote command identifiers.
pub enum Command {
    Play = 0,
    Pause = 1,
    Stop = 2,
    TogglePlayPause = 3,
    NextTrack = 4,
    PreviousTrack = 5,
    SkipForward = 6,
    SkipBackward = 7,
    SeekForward = 8,
    SeekBackward = 9,
    ChangePlaybackPosition = 10,
    EnableLanguageOption = 11,
    DisableLanguageOption = 12,
    ChangePlaybackRate = 13,
    ChangeRepeatMode = 14,
    ChangeShuffleMode = 15,
    Rating = 16,
    Like = 17,
    Dislike = 18,
    Bookmark = 19,
}

impl Command {
    #[must_use]
    fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Play),
            1 => Some(Self::Pause),
            2 => Some(Self::Stop),
            3 => Some(Self::TogglePlayPause),
            4 => Some(Self::NextTrack),
            5 => Some(Self::PreviousTrack),
            6 => Some(Self::SkipForward),
            7 => Some(Self::SkipBackward),
            8 => Some(Self::SeekForward),
            9 => Some(Self::SeekBackward),
            10 => Some(Self::ChangePlaybackPosition),
            11 => Some(Self::EnableLanguageOption),
            12 => Some(Self::DisableLanguageOption),
            13 => Some(Self::ChangePlaybackRate),
            14 => Some(Self::ChangeRepeatMode),
            15 => Some(Self::ChangeShuffleMode),
            16 => Some(Self::Rating),
            17 => Some(Self::Like),
            18 => Some(Self::Dislike),
            19 => Some(Self::Bookmark),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
#[non_exhaustive]
/// Maps to `MPRemoteCommandHandlerStatus`.
pub enum HandlerStatus {
    #[default]
    Success = 0,
    NoSuchContent = 100,
    NoActionableNowPlayingItem = 110,
    DeviceNotFound = 120,
    CommandFailed = 200,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
/// Maps to `MPSeekCommandEventType`.
pub enum SeekType {
    #[default]
    BeginSeeking = 0,
    EndSeeking = 1,
}

impl SeekType {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        if raw == 1 {
            Self::EndSeeking
        } else {
            Self::BeginSeeking
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
/// Maps to `MPShuffleType`.
pub enum ShuffleType {
    #[default]
    Off = 0,
    Items = 1,
    Collections = 2,
}

impl ShuffleType {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Items,
            2 => Self::Collections,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
/// Maps to `MPRepeatType`.
pub enum RepeatType {
    #[default]
    Off = 0,
    One = 1,
    All = 2,
}

impl RepeatType {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::One,
            2 => Self::All,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
/// Maps to `MPChangeLanguageOptionSetting`.
pub enum LanguageOptionSetting {
    #[default]
    None = 0,
    NowPlayingItemOnly = 1,
    Permanent = 2,
}

impl LanguageOptionSetting {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::NowPlayingItemOnly,
            2 => Self::Permanent,
            _ => Self::None,
        }
    }
}

#[derive(Debug)]
/// Data delivered to a remote command handler.
pub struct CommandEvent {
    pub command: Command,
    pub timestamp: f64,
    pub skip_interval: Option<f64>,
    pub seek_type: Option<SeekType>,
    pub position: Option<f64>,
    pub rating: Option<f64>,
    pub playback_rate: Option<f64>,
    pub feedback_negative: Option<bool>,
    pub shuffle_type: Option<ShuffleType>,
    pub repeat_type: Option<RepeatType>,
    pub preserves_shuffle_mode: Option<bool>,
    pub preserves_repeat_mode: Option<bool>,
    pub language_option: Option<LanguageOption>,
    pub language_option_setting: Option<LanguageOptionSetting>,
}

type HandlerBox = Box<dyn FnMut(CommandEvent) -> HandlerStatus + Send>;

unsafe extern "C" fn command_trampoline(
    refcon: *mut c_void,
    command_id: c_int,
    timestamp: c_double,
    extra: c_double,
    seek_type: c_int,
    rating: c_double,
    playback_rate: c_double,
    negative: c_int,
    shuffle_type: c_int,
    repeat_type: c_int,
    preserves_shuffle_mode: c_int,
    preserves_repeat_mode: c_int,
    language_option_ptr: *mut c_void,
    language_option_setting: c_int,
) -> c_int {
    if refcon.is_null() {
        return HandlerStatus::CommandFailed as i32;
    }

    let handler = &mut *(refcon.cast::<HandlerBox>());
    let command = Command::from_id(command_id).unwrap_or(Command::Play);
    let event = CommandEvent {
        command,
        timestamp,
        skip_interval: matches!(command, Command::SkipForward | Command::SkipBackward)
            .then_some(extra)
            .filter(|value| !value.is_nan()),
        seek_type: matches!(command, Command::SeekForward | Command::SeekBackward)
            .then(|| SeekType::from_raw(seek_type))
            .filter(|_| seek_type >= 0),
        position: matches!(command, Command::ChangePlaybackPosition)
            .then_some(extra)
            .filter(|value| !value.is_nan()),
        rating: (!rating.is_nan()).then_some(rating),
        playback_rate: (!playback_rate.is_nan()).then_some(playback_rate),
        feedback_negative: (negative >= 0).then_some(negative != 0),
        shuffle_type: (shuffle_type >= 0).then(|| ShuffleType::from_raw(shuffle_type)),
        repeat_type: (repeat_type >= 0).then(|| RepeatType::from_raw(repeat_type)),
        preserves_shuffle_mode: (preserves_shuffle_mode >= 0).then_some(preserves_shuffle_mode != 0),
        preserves_repeat_mode: (preserves_repeat_mode >= 0).then_some(preserves_repeat_mode != 0),
        language_option: (!language_option_ptr.is_null())
            .then(|| unsafe { LanguageOption::from_raw(language_option_ptr) }),
        language_option_setting: (language_option_setting >= 0)
            .then(|| LanguageOptionSetting::from_raw(language_option_setting)),
    };

    handler(event) as i32
}

/// RAII guard that keeps a remote command handler registered.
pub struct CommandToken {
    token_ptr: *mut c_void,
    closure_ptr: *mut HandlerBox,
    _not_sync: PhantomData<*mut ()>,
}

// SAFETY: The token owns the registration and a `Send` Rust closure.
unsafe impl Send for CommandToken {}

impl Drop for CommandToken {
    fn drop(&mut self) {
        if self.token_ptr.is_null() {
            if !self.closure_ptr.is_null() {
                unsafe { drop(Box::from_raw(self.closure_ptr)) }
            }
            return;
        }

        unsafe {
            ffi::mp_remote_command_remove_handler(self.token_ptr);
            ffi::mp_command_token_release(self.token_ptr);
            drop(Box::from_raw(self.closure_ptr));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkipIntervalCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RatingCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePlaybackRateCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePlaybackPositionCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeShuffleModeCommand {
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeRepeatModeCommand {
    command: Command,
}

macro_rules! impl_command_common {
    ($ty:ident) => {
        impl $ty {
            #[must_use]
            pub fn command(&self) -> Command {
                self.command
            }

            #[must_use]
            pub fn is_enabled(&self) -> bool {
                unsafe { ffi::mp_remote_command_is_enabled(self.command as i32) != 0 }
            }

            pub fn set_enabled(&self, enabled: bool) {
                unsafe { ffi::mp_remote_command_set_enabled(self.command as i32, i32::from(enabled)) }
            }

            pub fn add_handler<F>(&self, handler: F) -> CommandToken
            where
                F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
            {
                register_handler(self.command, handler)
            }
        }
    };
}

impl_command_common!(RemoteCommand);
impl_command_common!(SkipIntervalCommand);
impl_command_common!(FeedbackCommand);
impl_command_common!(RatingCommand);
impl_command_common!(ChangePlaybackRateCommand);
impl_command_common!(ChangePlaybackPositionCommand);
impl_command_common!(ChangeShuffleModeCommand);
impl_command_common!(ChangeRepeatModeCommand);

impl SkipIntervalCommand {
    #[must_use]
    pub fn preferred_intervals(&self) -> Vec<f64> {
        copy_parsed_lines::<f64>(unsafe { ffi::mp_skip_command_copy_preferred_intervals(self.command as i32) })
    }

    pub fn set_preferred_intervals(&self, intervals: &[f64]) {
        unsafe {
            ffi::mp_skip_command_set_preferred_intervals(
                self.command as i32,
                intervals.as_ptr(),
                intervals.len(),
            );
        }
    }
}

impl FeedbackCommand {
    #[must_use]
    pub fn is_active(&self) -> bool {
        unsafe { ffi::mp_feedback_command_is_active(self.command as i32) != 0 }
    }

    pub fn set_active(&self, active: bool) {
        unsafe { ffi::mp_feedback_command_set_active(self.command as i32, i32::from(active)) }
    }

    #[must_use]
    pub fn localized_title(&self) -> String {
        copy_string(unsafe { ffi::mp_feedback_command_copy_localized_title(self.command as i32) })
    }

    pub fn set_localized_title(&self, title: &str) {
        let title = CString::new(title).unwrap_or_default();
        unsafe { ffi::mp_feedback_command_set_localized_title(self.command as i32, title.as_ptr()) }
    }

    #[must_use]
    pub fn localized_short_title(&self) -> String {
        copy_string(unsafe { ffi::mp_feedback_command_copy_localized_short_title(self.command as i32) })
    }

    pub fn set_localized_short_title(&self, title: &str) {
        let title = CString::new(title).unwrap_or_default();
        unsafe {
            ffi::mp_feedback_command_set_localized_short_title(self.command as i32, title.as_ptr());
        }
    }
}

impl RatingCommand {
    #[must_use]
    pub fn minimum_rating(&self) -> f64 {
        unsafe { ffi::mp_rating_command_get_minimum_rating(self.command as i32) }
    }

    pub fn set_minimum_rating(&self, rating: f64) {
        unsafe { ffi::mp_rating_command_set_minimum_rating(self.command as i32, rating) }
    }

    #[must_use]
    pub fn maximum_rating(&self) -> f64 {
        unsafe { ffi::mp_rating_command_get_maximum_rating(self.command as i32) }
    }

    pub fn set_maximum_rating(&self, rating: f64) {
        unsafe { ffi::mp_rating_command_set_maximum_rating(self.command as i32, rating) }
    }
}

impl ChangePlaybackRateCommand {
    #[must_use]
    pub fn supported_playback_rates(&self) -> Vec<f64> {
        copy_parsed_lines::<f64>(unsafe {
            ffi::mp_change_playback_rate_copy_supported_rates(self.command as i32)
        })
    }

    pub fn set_supported_playback_rates(&self, rates: &[f64]) {
        unsafe {
            ffi::mp_change_playback_rate_set_supported_rates(
                self.command as i32,
                rates.as_ptr(),
                rates.len(),
            );
        }
    }
}

impl ChangeShuffleModeCommand {
    #[must_use]
    pub fn current_shuffle_type(&self) -> ShuffleType {
        ShuffleType::from_raw(unsafe {
            ffi::mp_change_shuffle_mode_get_current_shuffle_type(self.command as i32)
        })
    }

    pub fn set_current_shuffle_type(&self, shuffle_type: ShuffleType) {
        unsafe {
            ffi::mp_change_shuffle_mode_set_current_shuffle_type(
                self.command as i32,
                shuffle_type as i32,
            );
        }
    }
}

impl ChangeRepeatModeCommand {
    #[must_use]
    pub fn current_repeat_type(&self) -> RepeatType {
        RepeatType::from_raw(unsafe {
            ffi::mp_change_repeat_mode_get_current_repeat_type(self.command as i32)
        })
    }

    pub fn set_current_repeat_type(&self, repeat_type: RepeatType) {
        unsafe {
            ffi::mp_change_repeat_mode_set_current_repeat_type(self.command as i32, repeat_type as i32);
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Safe wrapper around `MPRemoteCommandCenter.shared()`.
pub struct RemoteCommandCenter;

impl RemoteCommandCenter {
    #[must_use]
    pub fn shared() -> Self {
        Self
    }

    #[must_use]
    pub fn command(&self, command: Command) -> RemoteCommand {
        RemoteCommand { command }
    }

    #[must_use]
    pub fn play_command(&self) -> RemoteCommand {
        self.command(Command::Play)
    }

    #[must_use]
    pub fn pause_command(&self) -> RemoteCommand {
        self.command(Command::Pause)
    }

    #[must_use]
    pub fn stop_command(&self) -> RemoteCommand {
        self.command(Command::Stop)
    }

    #[must_use]
    pub fn toggle_play_pause_command(&self) -> RemoteCommand {
        self.command(Command::TogglePlayPause)
    }

    #[must_use]
    pub fn next_track_command(&self) -> RemoteCommand {
        self.command(Command::NextTrack)
    }

    #[must_use]
    pub fn previous_track_command(&self) -> RemoteCommand {
        self.command(Command::PreviousTrack)
    }

    #[must_use]
    pub fn skip_forward_command(&self) -> SkipIntervalCommand {
        SkipIntervalCommand {
            command: Command::SkipForward,
        }
    }

    #[must_use]
    pub fn skip_backward_command(&self) -> SkipIntervalCommand {
        SkipIntervalCommand {
            command: Command::SkipBackward,
        }
    }

    #[must_use]
    pub fn seek_forward_command(&self) -> RemoteCommand {
        self.command(Command::SeekForward)
    }

    #[must_use]
    pub fn seek_backward_command(&self) -> RemoteCommand {
        self.command(Command::SeekBackward)
    }

    #[must_use]
    pub fn change_playback_position_command(&self) -> ChangePlaybackPositionCommand {
        ChangePlaybackPositionCommand {
            command: Command::ChangePlaybackPosition,
        }
    }

    #[must_use]
    pub fn enable_language_option_command(&self) -> RemoteCommand {
        self.command(Command::EnableLanguageOption)
    }

    #[must_use]
    pub fn disable_language_option_command(&self) -> RemoteCommand {
        self.command(Command::DisableLanguageOption)
    }

    #[must_use]
    pub fn change_playback_rate_command(&self) -> ChangePlaybackRateCommand {
        ChangePlaybackRateCommand {
            command: Command::ChangePlaybackRate,
        }
    }

    #[must_use]
    pub fn change_repeat_mode_command(&self) -> ChangeRepeatModeCommand {
        ChangeRepeatModeCommand {
            command: Command::ChangeRepeatMode,
        }
    }

    #[must_use]
    pub fn change_shuffle_mode_command(&self) -> ChangeShuffleModeCommand {
        ChangeShuffleModeCommand {
            command: Command::ChangeShuffleMode,
        }
    }

    #[must_use]
    pub fn rating_command(&self) -> RatingCommand {
        RatingCommand {
            command: Command::Rating,
        }
    }

    #[must_use]
    pub fn like_command(&self) -> FeedbackCommand {
        FeedbackCommand {
            command: Command::Like,
        }
    }

    #[must_use]
    pub fn dislike_command(&self) -> FeedbackCommand {
        FeedbackCommand {
            command: Command::Dislike,
        }
    }

    #[must_use]
    pub fn bookmark_command(&self) -> FeedbackCommand {
        FeedbackCommand {
            command: Command::Bookmark,
        }
    }

    pub fn add_handler<F>(&self, command: Command, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        register_handler(command, handler)
    }

    pub fn on_play<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.play_command().add_handler(handler)
    }

    pub fn on_pause<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.pause_command().add_handler(handler)
    }

    pub fn on_stop<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.stop_command().add_handler(handler)
    }

    pub fn on_toggle_play_pause<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.toggle_play_pause_command().add_handler(handler)
    }

    pub fn on_next_track<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.next_track_command().add_handler(handler)
    }

    pub fn on_previous_track<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.previous_track_command().add_handler(handler)
    }

    pub fn on_skip_forward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.skip_forward_command().add_handler(handler)
    }

    pub fn on_skip_backward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.skip_backward_command().add_handler(handler)
    }

    pub fn on_seek_forward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.seek_forward_command().add_handler(handler)
    }

    pub fn on_seek_backward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.seek_backward_command().add_handler(handler)
    }

    pub fn on_change_playback_position<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.change_playback_position_command().add_handler(handler)
    }

    pub fn on_enable_language_option<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.enable_language_option_command().add_handler(handler)
    }

    pub fn on_disable_language_option<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.disable_language_option_command().add_handler(handler)
    }

    pub fn on_change_playback_rate<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.change_playback_rate_command().add_handler(handler)
    }

    pub fn on_change_repeat_mode<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.change_repeat_mode_command().add_handler(handler)
    }

    pub fn on_change_shuffle_mode<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.change_shuffle_mode_command().add_handler(handler)
    }

    pub fn on_rating<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.rating_command().add_handler(handler)
    }

    pub fn on_like<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.like_command().add_handler(handler)
    }

    pub fn on_dislike<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.dislike_command().add_handler(handler)
    }

    pub fn on_bookmark<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.bookmark_command().add_handler(handler)
    }
}

fn register_handler<F>(command: Command, handler: F) -> CommandToken
where
    F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
{
    let boxed: Box<HandlerBox> = Box::new(Box::new(handler));
    let closure_ptr = Box::into_raw(boxed);

    let token_ptr = unsafe {
        ffi::mp_remote_command_add_handler(
            command as i32,
            Some(command_trampoline as MpCommandCallback),
            closure_ptr.cast::<c_void>(),
        )
    };

    CommandToken {
        token_ptr,
        closure_ptr,
        _not_sync: PhantomData,
    }
}

fn copy_string(ptr: *mut c_char) -> String {
    unsafe { unsupported::take_string(ptr) }.unwrap_or_default()
}

fn copy_parsed_lines<T>(ptr: *mut c_char) -> Vec<T>
where
    T: FromStr,
{
    copy_string(ptr)
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}
