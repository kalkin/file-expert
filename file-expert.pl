%  Copyright © 2019 Bahtiar `kalkin` Gadimov
%
%    This file is part of file-expert.
%
%  file-expert is free software: you can redistribute it and/or modify
%  it under the terms of the GNU Affero General Public License as published by
%  the Free Software Foundation, either version 3 of the License, or
%  (at your option) any later version.
%
%  file-expert is distributed in the hope that it will be useful,
%  but WITHOUT ANY WARRANTY; without even the implied warranty of
%  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
%  GNU Affero General Public License for more details.
%
%  You should have received a copy of the GNU Affero General Public License
%  along with file-expert.  If not, see <https://www.gnu.org/licenses/>.

:- ['extra-extensions-kb', 'github-extensions-kb'].

bang('#!').
bang('#! ').
shebang_path('/bin/').
shebang_path('/opt/bin/').
shebang_path('/sbin/').
shebang_path('/usr/bin/').
shebang_path('/usr/bin/env ').
shebang_path('/usr/local/bin/').
shebang_path('/usr/sbin/').

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr1, ExtStr2|_]),
    not(atom_string(Name, ExtStr1)),
    not(atom_string(Name, ExtStr2)),
    atom_concat('.', ExtStr1, ExtTmp1),
    atom_concat('.', ExtStr2, ExtTmp2),
    atom_concat(ExtTmp2, ExtTmp1, Ext).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr|_]),
    not(atom_string(Name, ExtStr)),
    atom_concat('.', ExtStr, Ext).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr1, ExtStr2|_]),
    not(atom_string(Name, ExtStr1)),
    not(atom_string(Name, ExtStr2)),
    atom_concat('.', ExtStr1, ExtTmp1),
    atom_concat('.', ExtStr2, ExtTmp2),
    atom_concat(ExtTmp2, ExtTmp1, ExtRes),
    downcase_atom(ExtRes, Ext).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr|_]),
    not(atom_string(Name, ExtStr)),
    atom_concat('.', ExtStr, ExtTmp),
    downcase_atom(ExtTmp, Ext).

read_file(Path, String):-
    open(Path, read, Stream, []),
    Length is 50*1024,
    read_string(Stream, Length, String),
    close(Stream).


first_line(Path, FirstLine):-
    exists_file(Path),
    open(Path, read, Stream),
    read_line_to_string(Stream, FirstLine),
    close(Stream).

shebang_exec(Path, Type):-
    first_line(Path, MagicLine),
    shebang(Cmd, MagicLine),
    interpreter(Type, Cmd).

shebang_exec(Path, Type):-
    first_line(Path, MagicLineTmp),
    split_string(MagicLineTmp, " ", "", [MagicLine|_]),
    shebang(Cmd, MagicLine),
    interpreter(Type, Cmd).

shebang(Cmd, MagicLine):-
    bang(Bang),
    shebang_path(Path),
    atom_concat(Bang, Path, Tmp),
    atom_concat(Tmp, Cmd, MagicLine).

match_regex(String, Pattern):-
    re_compile(Pattern, RegEx, [multiline(true)]),
    re_match(RegEx, String),
    re_flush().

guess_file(Path, Language):-
    file_base_name(Path, File), filename(Language, File).

guess_file(Path, Language):-
    parse_extension(Path, Ext),
    file_extension(Ext, Language, Pattern),
    read_file(Path, String),
    match_regex(String, Pattern).

guess_file(Path, Language):-
    shebang_exec(Path, Language).

guess_file(Path, Language):-
    parse_extension(Path, Ext),
    file_extension(Ext, Language).

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
    guess([H]),
    read_args(T).

main([]) :-
    prompt(_, ''),
    read_args(Argv),
    halt(0).

main(Argv) :-
        guess(Argv),
        halt(0).
