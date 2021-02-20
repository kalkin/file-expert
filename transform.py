#!/usr/bin/env python3
'''

A helper script to generate prolog rules from GitHub linguist YAML data.

There are four kind of rules which are generated from “./languages.yml” file.

⒈ eilename(File, Language).
⒉ extension(Extension, Language).
⒊ heuristic(File, Ext, Language).
⒋ interpreter(Interpreter, Language).
'''

import yaml


def escape_name(txt: str) -> str:
    ''' Escapes “'” character in language names '''
    return "'" + txt.replace("'", "\\'") + "'"


def escape_pattern(txt: str) -> str:
    ''' Escapes characters in regex patterns '''
    result = txt.replace("\\", "\\\\")
    result = result.replace('"', '\\"')
    return '"' + result + '"'


def pattern_match_rule(pattern) -> str:
    ''' Generate one or multiple file:match_regex() rules. '''
    if isinstance(pattern, list):
        rules = [
            '\t\t\tfile:match_regex(File, %s)' % escape_pattern(pat)
            for pat in pattern
        ]
        return '\t\t(\n' + ';\n'.join(rules).strip(';\n') + '\n\t\t).'
    return '\t\tfile:match_regex(File, %s), !.' % escape_pattern(pattern)


print(":- discontiguous extension/2.")
print(":- discontiguous filename/2.")
print(":- discontiguous interpreter/2.")
with open("languages.yml", 'r') as lang_stream:
    LANGS = yaml.safe_load(lang_stream)
    for name, data in LANGS.items():
        language = escape_name(name)
        if 'extensions' in data:
            for ext in data['extensions']:
                ext = escape_name(ext)
                print('extension(%s, %s).' % (ext, language))
        if 'interpreters' in data:
            for inter in data['interpreters']:
                inter = escape_name(inter)
                print('interpreter(%s, %s).' % (inter, language))
        if 'filenames' in data:
            for fname in data['filenames']:
                file_name = escape_name(fname)
                print('filename(%s, %s).' % (file_name, language))

with open("heuristics.yml", "r") as heuristic:
    HEURISTIC = yaml.safe_load(heuristic)
    DISAMBIGUATIONS = HEURISTIC['disambiguations']
    NAMED_PATTERN = HEURISTIC['named_patterns']
    for entries in DISAMBIGUATIONS:
        for ext in entries['extensions']:
            ext = escape_name(ext)
            for rule in entries['rules']:
                if isinstance(rule['language'], list):
                    continue
                lang = escape_name(rule['language'])
                goal = 'heuristic(File, %s, %s)' % (ext, lang)
                if 'named_pattern' in rule:
                    pattern_name = rule['named_pattern']
                    if isinstance(NAMED_PATTERN[pattern_name], str):
                        goal += ':-\n' + pattern_match_rule(
                            NAMED_PATTERN[pattern_name])
                    elif isinstance(NAMED_PATTERN[pattern_name], list):
                        goal += ':-\n'
                        goal += pattern_match_rule(NAMED_PATTERN[pattern_name])
                elif 'pattern' not in rule:
                    continue
                else:
                    goal += ':-\n' + pattern_match_rule(rule['pattern'])
                print(goal)
