//! Smoke example for the explicit macOS-unavailable `MPMediaQuery` wrapper.

use mediaplayer::MediaQuery;

fn main() {
    assert!(!MediaQuery::is_supported());
    println!("{}", MediaQuery::unavailable_reason());
    let err = MediaQuery::songs_query().expect_err("MPMediaQuery should be unavailable on macOS");
    println!("{err}");
}
