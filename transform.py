#!/usr/bin/env python3
import yaml

with open('heuristics.yml', 'r') as stream:
    DATA = yaml.safe_load(stream)


def escape_name(name: str) -> str:
    return name.replace(' ', '_') \
        .replace(" ", "_") \
        .replace("-", "_") \
        .replace("+", "_plus_") \
        .replace("*", "_star_") \
        .replace("#", "_sharp_") \
        .replace("'", "_quote_") \
        .replace(".", "_dot_") \
        .replace("(", "_") \
        .replace(")", "_").upper()


NAMED_PATTERNS = {}
LANG_REGEX_NUMBER = {}
OLD_NAMED_TO_NEW = {}
for key, value in DATA['named_patterns'].items():
    if key not in LANG_REGEX_NUMBER:
        LANG_REGEX_NUMBER[key] = 0

    if isinstance(value, str):
        value = [value]

    i = LANG_REGEX_NUMBER[key]
    if key not in OLD_NAMED_TO_NEW:
        OLD_NAMED_TO_NEW[key] = []
    for v in value:
        i += 1
        pattern_name = escape_name("%s_%d" % (key, i))
        NAMED_PATTERNS[pattern_name] = v
        OLD_NAMED_TO_NEW[key].append(pattern_name)
    LANG_REGEX_NUMBER[key] = i

DISAMBIGUATIONS = []
for ext_rules in DATA['disambiguations']:
    for rule in ext_rules['rules']:
        lang = rule['language']

        # handle the case when language is a list
        # happens only for *.mod Linux Kernel Module & AMPL
        if isinstance(lang, list):
            lang = lang[0]
            rule['language'] = lang

        if lang not in LANG_REGEX_NUMBER:
            LANG_REGEX_NUMBER[lang] = 0

        rule['or'] = []
        if 'pattern' in rule:
            if isinstance(rule['pattern'], str):
                rule['pattern'] = [rule['pattern']]

            for p in rule['pattern']:
                i = LANG_REGEX_NUMBER[lang]
                i += 1
                key = escape_name("%s_%d" % (lang, i))
                NAMED_PATTERNS[key] = p
                rule['or'].append(key)
                LANG_REGEX_NUMBER[lang] = i
            del rule['pattern']

        if 'named_pattern' in rule:
            if isinstance(rule['named_pattern'], str):
                rule['named_pattern'] = [rule['named_pattern']]

            for key in rule['named_pattern']:
                names = OLD_NAMED_TO_NEW[key]
                rule['or'] += names
            del rule['named_pattern']
        if not rule['or']:
            del rule['or']

        if 'and' in rule:
            new_and = []
            for obj in rule['and']:
                if 'pattern' in obj:
                    i = LANG_REGEX_NUMBER[lang]
                    i+=1
                    pattern_name = escape_name("%s_%d" % (lang, i))
                    NAMED_PATTERNS[pattern_name] = obj['pattern']
                    new_and.append(pattern_name)
                    LANG_REGEX_NUMBER[lang] = i

                if 'named_pattern' in obj:
                    names = OLD_NAMED_TO_NEW[obj['named_pattern']]
                    new_and += names

            rule['and'] = new_and

    DISAMBIGUATIONS.append(ext_rules)

with open('disambiguations.yml', 'w') as stream:
    yaml.safe_dump(DISAMBIGUATIONS, stream)

with open('named_patterns.yml', 'w') as stream:
    yaml.safe_dump(NAMED_PATTERNS, stream)
