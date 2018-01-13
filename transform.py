#!/usr/bin/env python3

import yaml


def normalize(name: str) -> str:
    return name.replace("'", "\\'")


with open("languages.yml", 'r') as stream:
    result = yaml.load(stream)
    for name, lang_data in result.items():
        lang_name = normalize(name)
        if 'extensions' in lang_data:
            for ext in lang_data['extensions']:
                print("typeExtension('%s', '%s')." % (lang_name, ext))
        print("")
    print("")
    for name, lang_data in result.items():
        lang_name = normalize(name)
        if 'interpreters' in lang_data:
            for inter in lang_data['interpreters']:
                print("interpreter('%s', '%s')." % (inter, lang_name))
    print("")
