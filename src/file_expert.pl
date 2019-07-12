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

