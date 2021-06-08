#!/usr/bin/env php
<?php
$PATTERNS_FILE = __DIR__ . '/../named_patterns.yml';
$HEURISTICS_FILE = __DIR__ . '/../disambiguations.yml';

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
    return strtoupper($tmp);
}

$PATTERNS = yaml_parse_file($PATTERNS_FILE);
?>
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
<?php 
foreach ($PATTERNS as $name => $pattern):
    $escaped_name = escape_name($name);
    $pattern = str_replace('"', '\"', $pattern);
?>
    static ref <?= $escaped_name ?>: Regex = Regex::new(r#"<?= $pattern ?>"#).unwrap();
<?php endforeach ?>
}
<?php
$HEURISTICS = yaml_parse_file($HEURISTICS_FILE);
?>
pub fn linguist_heuristic(ext: &str, content: &str) -> Option<&'static str> {
    match ext {
<?php
    foreach ($HEURISTICS as $extRules) :
        $noelse = false;
        $extensions = implode(' | ', array_map(fn($n) => "\"$n\"", $extRules['extensions']))
?>
    <?= $extensions ?> => {
<?php
        $first = true;
        foreach ($extRules['rules'] as $rule):
            if ($first) {
                echo "        if ";
                $first = false;
            } elseif (!isset($rule['and']) && !isset($rule['or'])) {
                echo "        else ";
            } else {
                echo "        else if ";
            }

            $lang = $rule['language'];
            $patterns = [];
            $joiner = "";
            if (isset($rule['and'])) {
                $patterns = $rule['and'];
                $joiner = ' && ';
            } elseif (isset($rule['or'])) {
                $patterns = $rule['or'];
                $joiner = ' || ';
            } else {
                $noelse = true;
            }
            $patterns = array_map(fn($e) => "match_lines(&$e, &content)", $patterns);

            if (!empty($patterns) && !empty($joiner)) {
                echo implode($joiner, $patterns);
            }
            echo " {\n";
?>
            Some("<?= $lang ?>")
        }
<?php   endforeach ?>
<?php   if (!$noelse) :?>
        else {
            None
        }
<?php   endif ?>
    },
<?php endforeach ?>
    _ => None
    }
}
fn match_lines(regex: &Regex, text: &str) -> bool {
    let lines: Vec<&str> = text.lines().collect::<Vec<_>>();
    for line in lines {
        if regex.is_match(line).unwrap() {
            return true;
        }
    }
    return false;
}

