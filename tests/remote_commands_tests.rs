use mediaplayer::{
    HandlerStatus, RemoteCommandCenter, RepeatType, ShuffleType,
};

#[test]
fn remote_command_configuration_round_trips() {
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

    let _play_token = play.add_handler(|_| HandlerStatus::Success);
    let _bookmark_token = center.on_bookmark(|event| {
        println!("bookmark event at {:.3}", event.timestamp);
        HandlerStatus::Success
    });
}
