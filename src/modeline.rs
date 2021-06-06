use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref VI_REGEX: Regex =
        Regex::new(r#"\bvim?\b.*\b(?:filetype|ft)=(.+?)\b"#).expect("Valid Regex");
}

pub fn parse_modeline(line: &str) -> Option<&str> {
    if let Some(caps) = VI_REGEX.captures(line).unwrap() {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str());
        }
    }
    None
}

#[test]
fn test() {
    let expected = Some("perl6");

    let actual = parse_modeline("# vim: set ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse_modeline("/* vim: ft=perl6 */");
    assert_eq!(expected, actual);

    let actual = parse_modeline("// vim:ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse_modeline("(* vim: set filetype=perl6: *)");
    assert_eq!(expected, actual);

    let actual = parse_modeline("-- vim: filetype=perl6");
    assert_eq!(expected, actual);

    let actual = parse_modeline("vim: set filetype=perl6");
    assert_eq!(expected, actual);

    let actual = parse_modeline("vim: noexpandtab: ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse_modeline(":vim:tw=78:ts=8:ft=perl6:norl:fen:fdl=0:fdm=marker:");
    assert_eq!(expected, actual);
}
