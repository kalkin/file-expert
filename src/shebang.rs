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

// TODO Add handling for multiline `/usr/bin/env\nexec ruby` shebangs
pub fn interpreter(first_line: &str) -> Option<String> {
    if !first_line.starts_with("#!") {
        return None;
    }
    let line = (&first_line[2..]).trim();
    let valid_paths = vec![
        "/bin/",
        "/opt/bin/",
        "/sbin/",
        "/usr/bin/env ",
        "/usr/bin/",
        "/usr/local/bin/",
        "/usr/sbin/",
    ];

    let mut interpreter_starts = None;
    for p in valid_paths {
        if line.starts_with(p) {
            interpreter_starts = Some(p.len());
            break;
        }
    }
    interpreter_starts?;
    let start = interpreter_starts.unwrap();
    if line.len() == start {
        return None;
    }
    return line[start..]
        .split(' ')
        .map(std::string::ToString::to_string)
        .find(|s| !s.starts_with('-') && !s.is_empty() && !s.contains('='));
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test {
    use crate::shebang::interpreter;

    #[test]
    fn simple() {
        assert_eq!(interpreter("Lore Ipsum Dolores"), None);
        assert_eq!(interpreter("#!/usr/bin/perl"), Some("perl".to_string()));
        assert_eq!(interpreter("#! /usr/bin/perl"), Some("perl".to_string()));
        assert_eq!(interpreter("#!/usr/bin/jq -fr"), Some("jq".to_string()));
        assert_eq!(interpreter("#!/var/foo/bin/python"), None);
        assert_eq!(interpreter("#! /sbin/"), None);
        assert_eq!(interpreter("#! /sbin/ -fr"), None);
        assert_eq!(interpreter("#!/usr/bin/env perl"), Some("perl".to_string()));
        assert_eq!(
            interpreter("#!/usr/bin/env  perl"),
            Some("perl".to_string())
        );
        assert_eq!(
            interpreter("#!/usr/bin/env  perl -n"),
            Some("perl".to_string())
        );
        assert_eq!(
            interpreter("#!/usr/bin/env -vS ruby -w -Ilib:test"),
            Some("ruby".to_string())
        );
        assert_eq!(
            interpreter("#!/usr/bin/env -vS ruby -wKU"),
            Some("ruby".to_string())
        );
        assert_eq!(
            interpreter("#!/usr/bin/env --split-string sed -f"),
            Some("sed".to_string())
        );
        assert_eq!(
            interpreter("#!/usr/bin/env -S GH_TOKEN=ghp_*** deno run --allow-net"),
            Some("deno".to_string())
        );
        assert_eq!(
            interpreter(
                "#! /usr/bin/env A=003 B=149 C=150 D=xzd E=base64 F=tar G=gz H=head I=tail sh"
            ),
            Some("sh".to_string())
        );
    }
}
