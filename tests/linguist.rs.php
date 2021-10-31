#!/usr/bin/env php
<?php
$SKIP_FILE = __DIR__ . '/../skipped.yml';
$SKIPPED = yaml_parse_file($SKIP_FILE);

function escape_name(string $text): string
{
    if (is_numeric($text[0])) {
        $text = "_$text";
    }
    $tmp = str_replace([' ', '-', '(', ')'], '_', $text);
    $tmp = str_replace('+', '_plus_', $tmp);
    $tmp = str_replace('*', '_star_', $tmp);
    $tmp = str_replace('#', '_sharp_', $tmp);
    $tmp = str_replace('\'', '_quote_', $tmp);
    $tmp = str_replace('.', '_dot_', $tmp);
    return strtolower($tmp);
}

function print_test(string $type, int $i, string $path, array $skipped)
{
    $ignore = "\n";
    $testName = "test_$i";
    if (isset($skipped[$type])) {
        $name = basename($path);
        if (in_array($name, $skipped[$type], true)) {
            $ignore .= "#[ignore]\n";
        }
    }

    echo <<<EOD
    $ignore
    #[test]
    fn $testName() {
        let path = Path::new(&"./samples/$path");
        let actual = guess(&path).unwrap();
        let expected = Guess::Kind("$type".to_string());
        assert_eq!(actual, expected);
    }

EOD;
}

$SAMPLES_DIR = __DIR__ . '/../samples';

$types = [];
if ($handle = opendir($SAMPLES_DIR)) {
    while (false !== ($entry = readdir($handle))) {
        if ($entry != "." && $entry != ".." && is_dir("$SAMPLES_DIR/$entry")) {
            $types[] = "$SAMPLES_DIR/$entry";
        }
    }
    closedir($handle);
} else {
    print("Failed to open samples dir: $SAMPLES_DIR\n");
    exit(1);
}

sort($types);
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
#![cfg(test)]
#![allow(non_snake_case)]

<?php foreach ($types as $val) : ?>
    <?php $type = basename($val) ?>
    <?php $escaped_type = escape_name($type); ?>

mod <?= $escaped_type ?> {
    use std::path::Path;
    use file_expert::Guess;
    use file_expert::guess;

    <?php
    $paths = new RecursiveDirectoryIterator($val, RecursiveDirectoryIterator::SKIP_DOTS);
    $i = 0;
    foreach ($paths as $p) {
        $filename = basename($p);
        if (is_dir($p)) {
            $sub_dir = basename($p);
            $sub = new RecursiveDirectoryIterator($p, RecursiveDirectoryIterator::SKIP_DOTS);
            foreach ($sub as $sp) {
                $filename = basename($sp);
                print_test($type, $i, "$type/$sub_dir/$filename", $SKIPPED);
                $i++;
            }
        } else {
            if ($type === "Fstar") {
                print_test('F*', $i, "$type/$filename", $SKIPPED);
            } else {
                print_test($type, $i, "$type/$filename", $SKIPPED);
            }
            $i++;
        }
    }?>
}
<?php endforeach ?>
