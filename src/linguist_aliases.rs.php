#!/usr/bin/env php
<?php
$LANGUAGES_FILE = __DIR__ . '/../languages.yml';
$LANGUAGES = yaml_parse_file($LANGUAGES_FILE);

$ALIAS_TO_LANG = [];
foreach ($LANGUAGES as $lang => $data) {
    if (!isset($data['aliases'])) {
        continue;
    }

    foreach ($data['aliases'] as $alias) {
        if (isset($ALIAS_TO_LANG[$alias])) {
            die("Duplicate alias $alias ($lang)");
        }
        $ALIAS_TO_LANG[$alias] = $lang;
    }
    $ALIAS_TO_LANG[strtolower($lang)] = $lang;
}
?>

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALIASES: HashMap<String, String> = [
<?php foreach ($ALIAS_TO_LANG as $name => $lang) :?>
        ("<?= $name ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}
