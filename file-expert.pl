shebangType(PATH, TYPE):-
    fileFirstLine(PATH, MAGIC_LINE),
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
    my_concat(BANG, PATH, CMD, MAGIC_LINE).

heuristic(_, [], unknown_type).

heuristic(_, [X], X).

heuristic(_, [_], multiple_posibilities).

fileType(PATH, RESULT):-
    fileExtension(PATH, EXT),
    setof(TYPE, typeExtension(TYPE, EXT), POSSIBLE_TYPES),
    heuristic(PATH, POSSIBLE_TYPES, RESULT);
    shebangType(PATH, RESULT);
    RESULT = unknown_type.
