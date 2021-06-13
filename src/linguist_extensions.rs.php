#!/usr/bin/env php
<?php
$LANGUAGES_FILE = __DIR__ . '/../languages.yml';
$LANGUAGES = yaml_parse_file($LANGUAGES_FILE);

$EXT_TO_LANG = [];
foreach ($LANGUAGES as $lang => $data) {
    if (!isset($data['extensions'])) {
        continue;
    }
    foreach ($data['extensions'] as $ext) {
        if (!isset($EXT_TO_LANG[$ext])) {
            $EXT_TO_LANG[$ext] = [];
        }
        $EXT_TO_LANG[$ext][] = $lang;
    }
}
$UNIQ_EXTENSIONS = [];
foreach ($EXT_TO_LANG as $ext => $languages) {
    if (count($languages) === 1) {
        $UNIQ_EXTENSIONS[$ext] = $languages[0];
    }
}
?>
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EXTENSIONS: HashMap<String, String> = [
<?php foreach ($UNIQ_EXTENSIONS as $ext => $lang) :?>
        ("<?= strtolower($ext) ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}
