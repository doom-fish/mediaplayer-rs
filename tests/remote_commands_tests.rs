mod common;

use std::sync::mpsc::{self, TryRecvError};

use common::LiveRemoteCommands;
use mediaplayer::{
    Command, HandlerStatus, RemoteCommandCenter, RepeatType, ShuffleType,
};

const EVERY_COMMAND: [Command; 20] = [
    Command::Play,
    Command::Pause,
    Command::Stop,
    Command::TogglePlayPause,
    Command::NextTrack,
    Command::PreviousTrack,
    Command::SkipForward,
    Command::SkipBackward,
    Command::SeekForward,
    Command::SeekBackward,
    Command::ChangePlaybackPosition,
    Command::EnableLanguageOption,
    Command::DisableLanguageOption,
    Command::ChangePlaybackRate,
    Command::ChangeRepeatMode,
    Command::ChangeShuffleMode,
    Command::Rating,
    Command::Like,
    Command::Dislike,
    Command::Bookmark,
];

#[test]
fn remote_command_configuration_round_trips() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "remote_command_configuration_round_trips",
        &[
            Command::Play,
            Command::SkipForward,
            Command::Like,
            Command::Rating,
            Command::ChangePlaybackRate,
            Command::ChangeShuffleMode,
            Command::ChangeRepeatMode,
            Command::Bookmark,
        ],
    ) else {
        return;
    };
    let center = RemoteCommandCenter::shared();

    let play = center.play_command();
    play.set_enabled(true);
    assert!(play.is_enabled());

    let skip = center.skip_forward_command();
    skip.set_preferred_intervals(&[10.0, 30.0]);
    let preferred_intervals = skip.preferred_intervals();
    assert_eq!(preferred_intervals.len(), 2);
    assert!((preferred_intervals[0] - 10.0).abs() < f64::EPSILON);
    assert!((preferred_intervals[1] - 30.0).abs() < f64::EPSILON);

    let like = center.like_command();
    like.set_active(true);
    like.set_localized_title("Like");
    like.set_localized_short_title("♥");
    assert!(like.is_active());
    assert_eq!(like.localized_title(), "Like");
    assert_eq!(like.localized_short_title(), "♥");

    let rating = center.rating_command();
    rating.set_minimum_rating(0.0);
    rating.set_maximum_rating(5.0);
    assert!((rating.minimum_rating() - 0.0).abs() < f64::EPSILON);
    assert!((rating.maximum_rating() - 5.0).abs() < f64::EPSILON);

    let playback_rate = center.change_playback_rate_command();
    playback_rate.set_supported_playback_rates(&[1.0, 1.5, 2.0]);
    let supported_rates = playback_rate.supported_playback_rates();
    assert_eq!(supported_rates.len(), 3);
    assert!((supported_rates[0] - 1.0).abs() < f64::EPSILON);
    assert!((supported_rates[1] - 1.5).abs() < f64::EPSILON);
    assert!((supported_rates[2] - 2.0).abs() < f64::EPSILON);

    let shuffle = center.change_shuffle_mode_command();
    shuffle.set_current_shuffle_type(ShuffleType::Items);
    assert_eq!(shuffle.current_shuffle_type(), ShuffleType::Items);

    let repeat = center.change_repeat_mode_command();
    repeat.set_current_repeat_type(RepeatType::All);
    assert_eq!(repeat.current_repeat_type(), RepeatType::All);

    let _play_token = play
        .add_handler(|_| HandlerStatus::Success)
        .expect("play handler should register");
    let _bookmark_token = center
        .on_bookmark(|event| {
            println!("bookmark event at {:.3}", event.timestamp);
            HandlerStatus::Success
        })
        .expect("bookmark handler should register");
}

#[test]
fn every_command_accepts_a_handler() {
    let Some(_commands) =
        LiveRemoteCommands::acquire("every_command_accepts_a_handler", &EVERY_COMMAND)
    else {
        return;
    };
    let center = RemoteCommandCenter::shared();
    let tokens = EVERY_COMMAND.map(|command| {
        center
            .add_handler(command, |_| HandlerStatus::Success)
            .unwrap_or_else(|error| panic!("{command:?} should register: {error}"))
    });
    assert_eq!(tokens.len(), 20);
}

#[test]
fn dropping_a_command_token_releases_its_handler() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "dropping_a_command_token_releases_its_handler",
        &[Command::Play],
    ) else {
        return;
    };
    let (sender, receiver) = mpsc::channel::<Command>();
    let token = RemoteCommandCenter::shared()
        .on_play(move |event| {
            let _ = sender.send(event.command);
            HandlerStatus::Success
        })
        .expect("play handler should register");
    assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
    drop(token);
    assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
}

#[test]
fn command_tokens_can_be_dropped_on_another_thread() {
    let Some(_commands) = LiveRemoteCommands::acquire(
        "command_tokens_can_be_dropped_on_another_thread",
        &[Command::Pause],
    ) else {
        return;
    };
    let (sender, receiver) = mpsc::channel::<Command>();
    let token = RemoteCommandCenter::shared()
        .on_pause(move |event| {
            let _ = sender.send(event.command);
            HandlerStatus::Success
        })
        .expect("pause handler should register");
    std::thread::spawn(move || drop(token))
        .join()
        .expect("token should drop on a secondary thread");
    assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
}
