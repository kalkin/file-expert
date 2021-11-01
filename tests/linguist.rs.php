#!/usr/bin/env php
<?php
$SKIP_FILE = __DIR__ . '/../skipped.yml';
$SKIPPED = yaml_parse_file($SKIP_FILE);

const SAMPLES_DIR = __DIR__ . DIRECTORY_SEPARATOR . '..' . DIRECTORY_SEPARATOR . 'samples';

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

function print_test(string $type, int $i, SplFileInfo $info, array $skipped)
{
    $ignore = "\n";
    $testName = "test_$i";
    if (isset($skipped[$type])) {
        $name = $info->getBasename();
        if (in_array($name, $skipped[$type], true)) {
            $ignore .= "#[ignore]\n";
        }
    }
    $path = explode(realpath(SAMPLES_DIR) . DIRECTORY_SEPARATOR, $info->getPathname(), 2)[1];

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

/**
 * Returns all sample files sorted by path
 *
 * @return SplFileInfo[]
 */
function getSampleFiles(): array {
    $dirIterator = new RecursiveDirectoryIterator(realpath(SAMPLES_DIR), RecursiveDirectoryIterator::SKIP_DOTS);
    $iterator  = new RecursiveIteratorIterator($dirIterator);
    $sampleFiles = array_values(iterator_to_array($iterator));
    usort($sampleFiles, static fn(SplFileInfo $a, SplFileInfo $b):int => $a->getPath() <=> $b->getPath());
    return array_filter($sampleFiles, static fn(SplFileInfo $info): bool => !$info->isDir());
}

/**
 * @param SplFileInfo[] $sampleFiles
 *
 * @return array<string,SplFileInfo[]>
 */
function buildLangSamplesTree(array $sampleFiles): array {
    $result = [];
    foreach($sampleFiles as $info) {
        assert($info instanceof SplFileInfo);
        $path = $info->getPath();
        $lang = explode(DIRECTORY_SEPARATOR, explode(realpath(SAMPLES_DIR) . DIRECTORY_SEPARATOR, $path, 2)[1], 2)[0];
        $result[$lang][] = $info;
    }
    return $result;
}


$languages = buildLangSamplesTree(getSampleFiles());

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

<?php foreach ($languages as $type => $infos) : ?>
    <?php $escaped_type = escape_name($type); ?>

mod <?= $escaped_type ?> {
    use std::path::Path;
    use file_expert::Guess;
    use file_expert::guess;

    <?php
    foreach ($infos as $i => $info) {
        assert($info instanceof SplFileInfo);
        $filename = $info->getBasename();
        if ($type === "Fstar") {
            print_test('F*', $i, $info, $SKIPPED);
        } else {
            print_test($type, $i, $info, $SKIPPED);
        }
    }?>
}
<?php endforeach ?>
