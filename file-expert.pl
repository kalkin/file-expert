
bang('#!').
bang('#! ').
executablePath('/bin').
executablePath('/usr/bin').
executablePath('/usr/bin/env ').


shebangType(PATH, TYPE):-
    fileFirstLine(PATH, MagicLine),
    shebang(Cmd, MagicLine),
    interpreter(Cmd, TYPE).

shebang(Cmd, MagicLine):-
    bang(Bang),
    executablePath(Path),
    atom_concat(Bang, Path, Tmp),
    atom_concat(Tmp, Cmd, MagicLine).

heuristic(_, [], unknown_type).

heuristic(_, [X], X).

heuristic(_, [_], multiple_posibilities).

fileType(PATH, RESULT):-
    fileExtension(PATH, EXT),
    setof(TYPE, typeExtension(TYPE, EXT), POSSIBLE_TYPES),
    heuristic(PATH, POSSIBLE_TYPES, RESULT), !, true;
    shebangType(PATH, RESULT), !, true;
    RESULT = unknown_type.
