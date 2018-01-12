:- ["github-extensions-kb"].

fileExtension(PATH, EXT):-
    file_base_name(PATH, NAME),
    split_string(NAME, ".", "", LIST),
    reverse(LIST, [EXT_STR|_]),
    string_concat(".", EXT_STR, EXT).


shebangType(PATH, TYPE):-
    exists_file(PATH),
    open(PATH, read, Stream),
    read_line_to_codes(Stream, Codes),
    atom_chars(MAGIC_LINE, Codes),
    shebang(TYPE, MAGIC_LINE).

shebang(TYPE, MAGIC_LINE):-
    shebang("/bin/", TYPE, MAGIC_LINE).

shebang(TYPE, MAGIC_LINE):-
    shebang("/usr/bin/", TYPE, MAGIC_LINE).

shebang(TYPE, MAGIC_LINE):-
    shebang("/usr/bin/env ", TYPE, MAGIC_LINE).

shebang(PATH, TYPE, MAGIC_LINE):-
    interpreter(TYPE, CMD),
    shebang("#! ", PATH, CMD, MAGIC_LINE).

shebang(PATH, TYPE, MAGIC_LINE):-
    interpreter(TYPE, CMD),
    shebang("#! ", PATH, CMD, MAGIC_LINE).

shebang(BANG, PATH, CMD, MAGIC_LINE):-
    string_concat(BANG, PATH, PREFIX),
    string_concat(PREFIX, CMD, MAGIC_LINE).


heuristic(_, [], unknown_type).

heuristic(_, [X], X).

heuristic(_, [_], multiple_posibilities).

fileType(PATH, RESULT):-
    fileExtension(PATH, EXT),
    setof(TYPE, typeExtension(TYPE, EXT), POSSIBLE_TYPES),
    heuristic(PATH, POSSIBLE_TYPES, RESULT);
    shebangType(PATH, RESULT);
    RESULT = unknown_type.
