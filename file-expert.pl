:- ['github-extensions-kb', 'extra-extensions-kb'].

bang('#!').
bang('#! ').
executablePath('/bin').
executablePath('/usr/bin').
executablePath('/usr/bin/env ').

fileExtension(PATH, EXT):-
    file_base_name(PATH, NAME),
    split_string(NAME, ".", "", LIST),
    reverse(LIST, [EXT_STR|_]),
    not(atom_string(NAME, EXT_STR)),
    atom_concat('.', EXT_STR, EXT).


fileFirstLine(PATH, FIRST_LINE):-
    exists_file(PATH),
    open(PATH, read, Stream),
    read_line_to_codes(Stream, Codes),
    atom_chars(FIRST_LINE, Codes).

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

heuristic(_, _, multiple_possibilities).

fileType(PATH, RESULT):-
    fileExtension(PATH, EXT),
    setof(TYPE, typeExtension(TYPE, EXT), POSSIBLE_TYPES),
    heuristic(PATH, POSSIBLE_TYPES, RESULT), !, true;
    shebangType(PATH, RESULT), !, true;
    RESULT = unknown_type.

main(Argv) :-
        guess(Argv),
        halt(0).

guess([]):-
    write("No files specified"), nl,
    halt(1).

guess([Last]) :- !,
        fileType(Last, Type),
        write(Type), nl.

guess([H|Rest]) :-
        fileType(H, Type),
        write(Type), nl,
        guess(Rest).
