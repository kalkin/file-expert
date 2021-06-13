#!/usr/bin/env php
<?php
$LANGUAGES_FILE = __DIR__ . '/../languages.yml';
$LANGUAGES = yaml_parse_file($LANGUAGES_FILE);
$INTERPRETERS = [];
foreach ($LANGUAGES as $lang => $data) {
    if (isset($data['interpreters'])) {
        foreach ($data['interpreters'] as $name) {
            if (isset($INTERPRETERS[$name])) {
                die("Duplicate interpreter $name ($lang)");
            }
            $INTERPRETERS[$name] = $lang;
        }
    }
}
?>
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INTERPRETERS: HashMap<String, String> = [
<?php foreach ($INTERPRETERS as $name => $lang) :?>
        ("<?= $name ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}
