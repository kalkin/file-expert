#!/usr/bin/env python3
'''

A helper script to generate prolog rules from GitHub linguist YAML data.

There are three kind of rules which are generated from “./languages.yml” file.

⒈ typeExtension(Language, Extension).
⒉ filename(Language, Filename).
⒊ interpreter(Interpreter, Language).
'''

import yaml


def escape_name(txt: str) -> str:
    ''' Escapes “'” character in language names '''
    return txt.replace("'", "\\'")


with open("languages.yml", 'r') as stream:
    RESULT = yaml.load(stream)
    for name, lang_data in RESULT.items():
        lang_name = escape_name(name)
        if 'extensions' in lang_data:
            for ext in lang_data['extensions']:
                print("typeExtension('%s', '%s')." % (lang_name, ext))
        print("")
    for name, lang_data in RESULT.items():
        lang_name = escape_name(name)
        if 'interpreters' in lang_data:
            for inter in lang_data['interpreters']:
                print("interpreter('%s', '%s')." % (inter, lang_name))
    print("")
