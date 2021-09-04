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
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALIASES: HashMap<String, String> = [
<?php foreach ($ALIAS_TO_LANG as $name => $lang) :?>
        ("<?= $name ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}
