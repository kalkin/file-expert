#!/usr/bin/env python3

import yaml


def normalize(name: str) -> str:
    name = name.lower().replace(" ", "_").replace("'", "")
    if name[0] == "1":
        name = "one_" + name[1:]
    name = name.replace("#", "_sharp")
    name = name.replace("+", "_plus")
    return name


with open("languages.yml", 'r') as stream:
    result = yaml.load(stream)
    for name, lang_data in result.items():
        lang_name = normalize(name)
        print('langName(%s, "%s").' % (lang_name, name))
    print("")
    print("")
    for name, lang_data in result.items():
        lang_name = normalize(name)
        if 'extensions' in lang_data:
            for ext in lang_data['extensions']:
                print('fileExtension("%s", %s).' % (ext, lang_name))
        print("")
