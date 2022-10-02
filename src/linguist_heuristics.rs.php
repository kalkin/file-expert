#!/usr/bin/env php
<?php
$PATTERNS_FILE = __DIR__ . '/../named_patterns.yml';
$HEURISTICS_FILE = __DIR__ . '/../disambiguations.yml';

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
    return strtoupper($tmp);
}

$PATTERNS = yaml_parse_file($PATTERNS_FILE);
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
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
<?php
foreach ($PATTERNS as $name => $pattern) :
    $escaped_name = escape_name($name);
    $pattern = str_replace('"', '\"', $pattern);
    ?>
    static ref <?= $escaped_name ?>: Regex = Regex::new(r#"<?= $pattern ?>"#).expect("Valid RegEx");
<?php endforeach ?>
}
<?php
$HEURISTICS = yaml_parse_file($HEURISTICS_FILE);
?>

#[allow(clippy::too_many_lines)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::if_same_then_else)]
#[allow(clippy::if_then_some_else_none)]
pub fn linguist_heuristic(ext: &str, content: &[String]) -> Option<&'static str> {
    match ext {
<?php
foreach ($HEURISTICS as $extRules) :
    $noelse = false;
    $extensions = implode(' | ', array_map(fn($n) => "\"$n\"", $extRules['extensions']))
    ?>
    <?= $extensions ?> => {
    <?php
    $first = true;
    foreach ($extRules['rules'] as $rule) :
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
        $patterns = array_map(fn($e) => "match_lines(&$e, content)", $patterns);

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
fn match_lines(regex: &Regex, lines: &[String]) -> bool {
    for line in lines {
        if regex.is_match(line).expect("Valid RegEx") {
            return true;
        }
    }
    false
}


