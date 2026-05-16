//! Configure command state and register handlers without requiring a UI session.

use mediaplayer::prelude::*;

fn main() {
    let center = RemoteCommandCenter::shared();

    let play = center.play_command();
    play.set_enabled(true);

    let skip = center.skip_forward_command();
    skip.set_preferred_intervals(&[15.0, 30.0]);

    let like = center.like_command();
    like.set_active(true);
    like.set_localized_title("Like");
    like.set_localized_short_title("♥");

    let rating = center.rating_command();
    rating.set_minimum_rating(0.0);
    rating.set_maximum_rating(5.0);

    let rate = center.change_playback_rate_command();
    rate.set_supported_playback_rates(&[1.0, 1.5, 2.0]);

    let shuffle = center.change_shuffle_mode_command();
    shuffle.set_current_shuffle_type(ShuffleType::Items);

    let repeat = center.change_repeat_mode_command();
    repeat.set_current_repeat_type(RepeatType::All);

    let _play_token = play.add_handler(|event| {
        println!("play at {:.3}", event.timestamp);
        HandlerStatus::Success
    });
    let _rating_token = center.on_rating(|event| {
        println!("rating = {:?}", event.rating);
        HandlerStatus::Success
    });
    let _language_token = center.on_enable_language_option(|event| {
        println!("language option = {:?}", event.language_option_setting);
        HandlerStatus::Success
    });

    let enabled = play.is_enabled();
    let skip_intervals = skip.preferred_intervals();
    let like_active = like.is_active();
    let supported_rates = rate.supported_playback_rates();
    println!("enabled={enabled} skip={skip_intervals:?} like={like_active} rates={supported_rates:?}");
}
