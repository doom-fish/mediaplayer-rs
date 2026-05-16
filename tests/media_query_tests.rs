use mediaplayer::MediaQuery;

#[test]
fn media_query_is_explicitly_unavailable_on_macos() {
    assert!(!MediaQuery::is_supported());

    let reason = MediaQuery::unavailable_reason();
    assert!(reason.contains("MPMediaQuery"), "unexpected reason: {reason}");

    let err = MediaQuery::songs_query().expect_err("MPMediaQuery should be unavailable on macOS");
    assert!(err.to_string().contains("MPMediaQuery"));
}
