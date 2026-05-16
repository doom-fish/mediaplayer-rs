//! Wrapper for `MPRemoteCommandCenter` and associated types.

use core::ffi::{c_double, c_int, c_void};
use std::marker::PhantomData;

use crate::ffi::{self, MpCommandCallback};

// ── Command ───────────────────────────────────────────────────────────────────

/// Remote command identifiers, matching the `command_id` constants used
/// across the Swift bridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[non_exhaustive]
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
            _ => None,
        }
    }
}

// ── HandlerStatus ─────────────────────────────────────────────────────────────

/// Return this from a command handler to report the outcome to the system.
///
/// Maps to `MPRemoteCommandHandlerStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
#[non_exhaustive]
pub enum HandlerStatus {
    #[default]
    Success = 0,
    NoSuchContent = 100,
    NoActionableNowPlayingItem = 110,
    /// Requires macOS 14+; treated as `CommandFailed` on earlier releases.
    DeviceNotFound = 120,
    CommandFailed = 200,
}

// ── SeekType ──────────────────────────────────────────────────────────────────

/// Maps to `MPSeekCommandEventType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum SeekType {
    #[default]
    BeginSeeking = 0,
    EndSeeking = 1,
}

impl SeekType {
    #[must_use]
    fn from_raw(raw: i32) -> Self {
        if raw == 1 { Self::EndSeeking } else { Self::BeginSeeking }
    }
}

// ── CommandEvent ──────────────────────────────────────────────────────────────

/// Data delivered to a command handler.
#[derive(Debug, Clone)]
pub struct CommandEvent {
    /// Which command was triggered.
    pub command: Command,
    /// System timestamp of the event (seconds).
    pub timestamp: f64,
    /// Interval to skip, for [`Command::SkipForward`] / [`Command::SkipBackward`].
    pub skip_interval: Option<f64>,
    /// Seek direction, for [`Command::SeekForward`] / [`Command::SeekBackward`].
    pub seek_type: Option<SeekType>,
    /// Target playback position, for [`Command::ChangePlaybackPosition`].
    pub position: Option<f64>,
}

impl CommandEvent {
    fn from_raw(command_id: i32, timestamp: f64, extra: f64, seek_type_raw: i32) -> Self {
        let command = Command::from_id(command_id).unwrap_or(Command::Play);
        let skip_interval = matches!(command, Command::SkipForward | Command::SkipBackward)
            .then_some(extra);
        let seek_type = matches!(command, Command::SeekForward | Command::SeekBackward)
            .then(|| SeekType::from_raw(seek_type_raw));
        let position = matches!(command, Command::ChangePlaybackPosition).then_some(extra);
        Self {
            command,
            timestamp,
            skip_interval,
            seek_type,
            position,
        }
    }
}

// ── Trampoline ────────────────────────────────────────────────────────────────

type HandlerBox = Box<dyn FnMut(CommandEvent) -> HandlerStatus + Send>;

unsafe extern "C" fn command_trampoline(
    refcon: *mut c_void,
    command_id: c_int,
    timestamp: c_double,
    extra: c_double,
    seek_type: c_int,
) -> c_int {
    if refcon.is_null() {
        return HandlerStatus::CommandFailed as i32;
    }
    let handler = &mut *(refcon.cast::<HandlerBox>());
    let event = CommandEvent::from_raw(command_id, timestamp, extra, seek_type);
    handler(event) as i32
}

// ── CommandToken ──────────────────────────────────────────────────────────────

/// RAII guard that keeps a remote command handler registered.
///
/// Dropping the token deregisters the handler and frees all associated
/// resources.  The closure will not be called after drop returns.
pub struct CommandToken {
    token_ptr: *mut c_void,
    closure_ptr: *mut HandlerBox,
    _not_sync: PhantomData<*mut ()>,
}

// SAFETY: The token_ptr and closure_ptr are owned by this struct.
// The Swift bridge is thread-safe and the closure is required to be Send.
unsafe impl Send for CommandToken {}

impl Drop for CommandToken {
    fn drop(&mut self) {
        if self.token_ptr.is_null() {
            // Handler registration failed at construction time; just free closure.
            if !self.closure_ptr.is_null() {
                unsafe { drop(Box::from_raw(self.closure_ptr)) }
            }
            return;
        }
        unsafe {
            // Remove before freeing the closure so the trampoline can't fire
            // with a dangling pointer.
            ffi::mp_remote_command_remove_handler(self.token_ptr);
            ffi::mp_command_token_release(self.token_ptr);
            drop(Box::from_raw(self.closure_ptr));
        }
    }
}

// ── RemoteCommandCenter ───────────────────────────────────────────────────────

/// Safe wrapper around `MPRemoteCommandCenter.shared()`.
///
/// Zero-sized; cheap to construct via [`RemoteCommandCenter::shared`].
#[derive(Debug, Clone, Copy)]
pub struct RemoteCommandCenter;

impl RemoteCommandCenter {
    /// Obtain a handle to the shared remote command center.
    #[must_use]
    pub fn shared() -> Self {
        Self
    }

    /// Register `handler` for the given `command`.
    ///
    /// Returns a [`CommandToken`] whose [`Drop`] implementation automatically
    /// deregisters the handler.  Keep the token alive for as long as you want
    /// the handler to remain active.
    pub fn add_handler<F>(&self, command: Command, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        // Double-box to get a thin pointer (Box<dyn Trait> is fat).
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

    // ── Convenience wrappers ──────────────────────────────────────────────────

    pub fn on_play<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::Play, handler)
    }

    pub fn on_pause<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::Pause, handler)
    }

    pub fn on_stop<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::Stop, handler)
    }

    pub fn on_toggle_play_pause<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::TogglePlayPause, handler)
    }

    pub fn on_next_track<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::NextTrack, handler)
    }

    pub fn on_previous_track<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::PreviousTrack, handler)
    }

    pub fn on_skip_forward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::SkipForward, handler)
    }

    pub fn on_skip_backward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::SkipBackward, handler)
    }

    pub fn on_seek_forward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::SeekForward, handler)
    }

    pub fn on_seek_backward<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::SeekBackward, handler)
    }

    pub fn on_change_playback_position<F>(&self, handler: F) -> CommandToken
    where
        F: FnMut(CommandEvent) -> HandlerStatus + Send + 'static,
    {
        self.add_handler(Command::ChangePlaybackPosition, handler)
    }
}
