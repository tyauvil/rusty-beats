extern crate rusty_beats;
use regex::Regex;

#[test]
fn test_beats_format() {
    let re = Regex::new(r"@\d{3}.beats").unwrap();
    let beats = rusty_beats::beats();
    assert!(re.is_match(&beats));
}
