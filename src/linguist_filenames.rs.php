#!/usr/bin/env php
<?php
$LANGUAGES_FILE = __DIR__ . '/../languages.yml';
$LANGUAGES = yaml_parse_file($LANGUAGES_FILE);
$NAMES = [];
foreach ($LANGUAGES as $lang => $data) {
    if (isset($data['filenames'])) {
        foreach ($data['filenames'] as $name) {
            if (isset($NAME[$name])) {
                die("Duplicate filename $name ($lang)");
            }
            $NAMES[$name] = $lang;
        }
    }
}
?>
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FILENAMES: HashMap<String, String> = [
<?php foreach($NAMES as $name => $lang) :?>
        ("<?= $name ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}

