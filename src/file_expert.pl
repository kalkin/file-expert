:- module(file_expert, [guess_file/2]).
:- use_module('../lib/file').

bang('#!').
bang('#! ').
shebang_path('/bin/').
shebang_path('/opt/bin/').
shebang_path('/sbin/').
shebang_path('/usr/bin/').
shebang_path('/usr/bin/env ').
shebang_path('/usr/local/bin/').
shebang_path('/usr/sbin/').


shebang(Cmd, MagicLine):-
    bang(Bang),
    shebang_path(Path),
    atom_concat(Bang, Path, Tmp),
    atom_concat(Tmp, Cmd, MagicLine).

shebang_exec(Path, Type):-
    file:first_line(Path, MagicLine),
    shebang(Cmd, MagicLine),
    interpreter(Type, Cmd).

shebang_exec(Path, Type):-
    file:first_line(Path, MagicLineTmp),
    split_string(MagicLineTmp, " ", "", [MagicLine|_]),
    shebang(Cmd, MagicLine),
    interpreter(Type, Cmd).

match_regex(String, Pattern):-
    re_compile(Pattern, RegEx, [multiline(true)]),
    re_match(RegEx, String),
    re_flush().

guess_file(Path, Language):-
    file_base_name(Path, File), filename(Language, File).

guess_file(Path, Language):-
    MaxLength is 10*1024,
    file:parse_extension(Path, Ext),
    file_extension(Ext, Language, Pattern),
    file:read_file(Path, MaxLength, String),
    match_regex(String, Pattern).

guess_file(Path, Language):-
    file_expert:shebang_exec(Path, Language).

guess_file(Path, Language):-
    parse_extension(Path, Ext),
    file_extension(Ext, Language).

guess_file(_, unknown_type).

