#!/usr/bin/env php
<?php
$SKIP_FILE = __DIR__ . '/../skipped.yml';
$SKIPPED = yaml_parse_file($SKIP_FILE);

function escape_name(String $text): String {
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

function print_test(String $type, int $i, String $path, array $skipped) {
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
        let actual = expert(&path).unwrap();
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
}?>
#![cfg(test)]
#![allow(non_snake_case)]

<?php foreach ($types as $val) : ?>
    <?php $type = basename($val) ?>
    <?php $escaped_type = escape_name($type); ?>

mod <?= $escaped_type ?> {
    use std::path::Path;
    use file_expert::Guess;
    use file_expert::expert;

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
