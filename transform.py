#!/usr/bin/env python3
'''

A helper script to generate prolog rules from GitHub linguist YAML data.

There are four kind of rules which are generated from “./languages.yml” file.

⒈ file_extension(Extension, Language).
⒉ file_extension(Extension, Language, RegEx).
⒊ filename(Language, Filename).
⒋ interpreter(Language, Interpreter).
'''

import sys

import yaml


def escape_name(txt: str) -> str:
    ''' Escapes “'” character in language names '''
    return txt.replace("'", "\\'")


def escape_pattern(txt: str) -> str:
    ''' Escapes “'” character in language names '''
    result = txt.replace("\\", "\\\\")
    result = result.replace('"', '\\"')
    return result


RESULT = {}
FILENAMES = []
INTERPRETER = []
IGNORED = {
    'Ruby': ['.spec'],
    'Python': ['.spec'],
    'RPM Spec': ['.spec'],
    'Eagle': ['.sch'],
    'KiCad Schematic': ['.sch'],
}


def add_rule(ext, lang, pattern):
    ''' Add rule to RESULT global '''
    if ext not in RESULT:
        RESULT[ext] = {}

    if lang not in RESULT[ext]:
        RESULT[ext][lang] = []
    RESULT[ext][lang].append(pattern)


def print_rule(ext, lang, pattern=None):
    ''' Prints file_extension rule '''
    if pattern is None:
        if lang in IGNORED and ext in IGNORED[lang]:
            return
        print("file_extension('%s', '%s')." % (ext, escape_name(lang)))
    elif isinstance(pattern, str):
        print("file_extension('%s', '%s', \"%s\")." %
              (ext, escape_name(lang), escape_pattern(pattern)))
    else:
        for pat in pattern:
            print('file_extension(\'%s\', \'%s\', "%s").' %
                  (ext, escape_name(lang), escape_pattern(pat)))


def print_rules():
    ''' Prints all rules '''
    for ext in RESULT:
        for lang in RESULT[ext]:
            if isinstance(RESULT[ext][lang], list):
                for pattern in RESULT[ext][lang]:
                    print_rule(ext, lang, pattern)
    print()

    for ext in RESULT:
        for lang in RESULT[ext]:
            if RESULT[ext][lang] is None:
                print_rule(ext, lang, None)
    print()

    for sth in FILENAMES:
        print("filename('%s', '%s')." % (escape_name(sth[0]), sth[1]))
    print()

    for sth in INTERPRETER:
        print("interpreter('%s', '%s')." % (escape_name(sth[0]), sth[1]))


def parse_heuristics():
    ''' Parse heuristic.yml '''
    with open("heuristics.yml", "r") as heuristic:
        data = yaml.load(heuristic)
        extensions_dict = data['disambiguations']
        named_pattern = data['named_patterns']
        for extensions in extensions_dict:
            for ext in extensions['extensions']:
                for rule in extensions['rules']:
                    if isinstance(rule['language'], list):
                        continue
                    lang = rule['language']
                    if 'named_pattern' in rule:
                        pattern_name = rule['named_pattern']
                        if isinstance(named_pattern[pattern_name], str):
                            pattern = named_pattern[pattern_name]
                            add_rule(ext, lang, pattern)
                        elif isinstance(named_pattern[pattern_name], list):
                            for pattern in named_pattern[pattern_name]:
                                add_rule(ext, lang, pattern)
                        else:
                            print("WTF?")
                    elif 'pattern' not in rule:
                        print(
                            "Error: parsing rule for %s" % lang,
                            file=sys.stderr)
                        continue
                    else:
                        pattern = rule['pattern']
                        add_rule(ext, lang, pattern)


def parse_langs():
    ''' Parse languages.yml '''
    with open("languages.yml", 'r') as lang_stream:
        langs = yaml.load(lang_stream)
        for name, data in langs.items():
            name = name
            if 'extensions' in data:
                for ext in data['extensions']:
                    # print("--> %s %s" % (ext, name))
                    if ext in RESULT and name in RESULT[ext]:
                        continue
                    elif ext in RESULT:
                        RESULT[ext][name] = None
                    else:
                        RESULT[ext] = {name: None}
            if 'filenames' in data:
                for fname in data['filenames']:
                    FILENAMES.append((name, fname))
            if 'interpreters' in data:
                for inter in data['interpreters']:
                    INTERPRETER.append((name, inter))


def filter_rules():
    ''' Filter out results which map to multiple extensions.'''
    for ext in RESULT:
        i = 0
        duplicates = []
        for name, val in RESULT[ext].items():
            if val is not None:
                continue
            if name in IGNORED and ext in IGNORED[name]:
                continue
            i += 1
            if i > 1:
                duplicates.append(name)
        if i > 1:
            for name in duplicates:
                del RESULT[ext][name]


parse_heuristics()
parse_langs()
filter_rules()
print_rules()
