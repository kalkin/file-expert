:- ['github-extensions-kb', 'extra-extensions-kb'].

bang('#!').
bang('#! ').
executablePath('/bin/').
executablePath('/opt/bin/').
executablePath('/sbin/').
executablePath('/usr/bin/').
executablePath('/usr/bin/env ').
executablePath('/usr/local/bin/').
executablePath('/usr/sbin/').

parse_extension(PATH, EXT):-
    file_base_name(PATH, NAME),
    split_string(NAME, ".", "", LIST),
    reverse(LIST, [EXT_STR|_]),
    not(atom_string(NAME, EXT_STR)),
    atom_concat('.', EXT_STR, EXT).

read_file(Path, String):-
    open(Path, read, Stream, []),
    Length is 50*1024,
    read_string(Stream, Length, String).


fileFirstLine(PATH, FIRST_LINE):-
    exists_file(PATH),
    open(PATH, read, Stream),
    read_line_to_string(Stream, FIRST_LINE),
    close(Stream).

shebangType(PATH, TYPE):-
    fileFirstLine(PATH, MagicLine),
    shebang(Cmd, MagicLine),
    interpreter(TYPE, Cmd).

shebang(Cmd, MagicLine):-
    bang(Bang),
    executablePath(Path),
    atom_concat(Bang, Path, Tmp),
    atom_concat(Tmp, Cmd, MagicLine).

guess_file(Path, Language):-
    file_base_name(Path, File), filename(Language, File).

guess_file(Path, Language):-
    parse_extension(Path, Ext),
    file_extension(Ext, Language, Pattern),
    re_compile(Pattern, RegEx, [multiline(true)]),
    read_file(Path, String),
    re_match(RegEx, String).

guess_file(Path, Language):-
    parse_extension(Path, Ext),
    file_extension(Ext, Language).

guess_file(Path, Language):-
    shebangType(Path, Language).

guess_file(_, unknown_type).

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
        guess_file(Last, Type),
        say(Last, Type).

guess([H|Rest]) :-
        guess_file(H, Type),
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
