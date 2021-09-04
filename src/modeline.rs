//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref VI_REGEX: Regex =
        Regex::new(r#"\bvim?\b.*\b(?:filetype|ft|syntax|syn)=(.+?)\b"#).expect("Valid Regex");
    static ref EMACS_REGEX: Regex =
        Regex::new(r#"\s-\*-\s+(?:mode:\s+)?(\w+)\b.*\s-\*-"#).expect("Valid Regex");
}

pub fn parse(line: &str) -> Option<&str> {
    if let Some(caps) = VI_REGEX.captures(line).unwrap() {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str());
        }
    } else if let Some(caps) = EMACS_REGEX.captures(line).unwrap() {
        if let Some(m) = caps.get(1) {
            return Some(m.as_str());
        }
    }
    None
}

#[test]
fn vim_modeline() {
    let expected = Some("perl6");

    let actual = parse("# vim: set ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse("/* vim: ft=perl6 */");
    assert_eq!(expected, actual);

    let actual = parse("// vim:ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse("(* vim: set filetype=perl6: *)");
    assert_eq!(expected, actual);

    let actual = parse("-- vim: filetype=perl6");
    assert_eq!(expected, actual);

    let actual = parse("vim: set filetype=perl6");
    assert_eq!(expected, actual);

    let actual = parse("vim: noexpandtab: ft=perl6");
    assert_eq!(expected, actual);

    let actual = parse(":vim:tw=78:ts=8:ft=perl6:norl:fen:fdl=0:fdm=marker:");
    assert_eq!(expected, actual);
}

#[test]
fn emacs_modeline() {
    let expected = Some("Lisp");
    let actual = parse("# -*- Lisp -*-");
    assert_eq!(expected, actual);

    let expected = Some("Lisp");
    let actual = parse("# -*- mode: Lisp; eval: (auto-fill-mode 1); -*-");
    assert_eq!(expected, actual);
}
