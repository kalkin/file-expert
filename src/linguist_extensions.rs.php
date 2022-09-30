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
    pub static ref EXTENSIONS: HashMap<String, String> = [
<?php foreach ($UNIQ_EXTENSIONS as $ext => $lang) :?>
        ("<?= strtolower($ext) ?>".to_owned(), "<?= $lang ?>".to_owned()),
<?php endforeach ?>
    ].iter().cloned().collect();
}
