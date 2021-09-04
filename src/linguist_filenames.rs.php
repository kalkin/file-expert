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
    pub static ref FILENAMES: HashMap<String, String> = [
<?php foreach ($NAMES as $name => $lang) :?>
        ("<?= $name ?>".to_string(), "<?= $lang ?>".to_string()),
<?php endforeach ?>
    ].iter().cloned().collect();
}

