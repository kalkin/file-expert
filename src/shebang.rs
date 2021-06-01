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
    let mut interpreter = String::new();
    for c in line[start..].chars() {
        if c == ' ' {
            break;
        }
        interpreter.push(c);
    }
    if !interpreter.is_empty() {
        return Some(interpreter);
    }
    None
}

#[cfg(test)]
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
    }
}
