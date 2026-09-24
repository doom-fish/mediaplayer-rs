#![allow(dead_code)]

use std::mem::ManuallyDrop;
use std::sync::{Mutex, MutexGuard, PoisonError};

use mediaplayer::{
    Command, NowPlayingInfo, NowPlayingInfoCenter, NowPlayingValue, PlaybackState,
    RemoteCommandCenter,
};

static SHARED_CENTERS: Mutex<()> = Mutex::new(());

pub fn live_tests_enabled(test: &str) -> bool {
    let enabled = std::env::var("MEDIAPLAYER_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!(
            "{test}: skipped; set MEDIAPLAYER_LIVE_TESTS=1 to use Now Playing and remote commands"
        );
    }
    enabled
}

fn exclusive() -> MutexGuard<'static, ()> {
    SHARED_CENTERS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

pub struct LiveNowPlaying {
    previous_info: Option<NowPlayingInfo>,
    previous_state: PlaybackState,
    _exclusive: MutexGuard<'static, ()>,
}

impl LiveNowPlaying {
    pub fn acquire(test: &str) -> Option<Self> {
        if !live_tests_enabled(test) {
            return None;
        }
        let exclusive = exclusive();
        let center = ManuallyDrop::new(NowPlayingInfoCenter::default_center());
        let previous_info = center.now_playing_info().map(|values| {
            values
                .into_iter()
                .fold(NowPlayingInfo::new(), |info, (key, value)| {
                    assert!(
                        !matches!(value, NowPlayingValue::Other(_)),
                        "{test}: can't restore the current Now Playing {key} value"
                    );
                    info.value(key, value)
                })
        });
        Some(Self {
            previous_info,
            previous_state: center.playback_state(),
            _exclusive: exclusive,
        })
    }
}

impl Drop for LiveNowPlaying {
    fn drop(&mut self) {
        let center = ManuallyDrop::new(NowPlayingInfoCenter::default_center());
        match &self.previous_info {
            Some(info) => {
                if let Err(error) = center.set_now_playing_info(info) {
                    eprintln!("could not restore the previous Now Playing info: {error}");
                    center.clear();
                }
            }
            None => center.clear(),
        }
        center.set_playback_state(self.previous_state);
    }
}

pub struct LiveRemoteCommands {
    commands: Vec<Command>,
    _exclusive: MutexGuard<'static, ()>,
}

impl LiveRemoteCommands {
    pub fn acquire(test: &str, commands: &[Command]) -> Option<Self> {
        if !live_tests_enabled(test) {
            return None;
        }
        Some(Self {
            commands: commands.to_vec(),
            _exclusive: exclusive(),
        })
    }
}

impl Drop for LiveRemoteCommands {
    fn drop(&mut self) {
        let center = RemoteCommandCenter::shared();
        for command in &self.commands {
            center.command(*command).set_enabled(false);
        }
    }
}
