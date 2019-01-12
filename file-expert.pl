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
    read_line_to_string(Stream, FIRST_LINE).

shebangType(PATH, TYPE):-
    fileFirstLine(PATH, MagicLine),
    shebang(Cmd, MagicLine),
    interpreter(TYPE, Cmd).

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
    heuristic(PATH, POSSIBLE_TYPES, RESULT);
    shebangType(PATH, RESULT);
    RESULT = unknown_type.

say(File, unknown_type):-
    write(File), write('\t'), write('Unknown file'), nl.

say(File, multiple_possibilities):-
    write(File), write('\t'), write('Unknown file'), nl.

say(File, Type):-
    write(File), write('\t'), write(Type), nl.

guess([]):-
    write("No files specified"), nl,
    halt(1).

guess([Last]) :- !,
        fileType(Last, Type),
        say(Last, Type).

guess([H|Rest]) :-
        fileType(H, Type),
        say(H, Type),
        guess(Rest).

read_args([]):-
    at_end_of_stream(user_input).

read_args([H|T]):-
    \+ at_end_of_stream(user_input),
    read_line_to_string(user_input, H),
    read_args(T).

main([]) :-
    prompt(_, ''),
    read_args(Argv),
    guess(Argv),
    halt(0).

main(Argv) :-
        guess(Argv),
        halt(0).
