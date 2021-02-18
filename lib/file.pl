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

:- module(file, [parse_extension/2, read_file/3, first_line/2,
                 list_files_recursive/2, file_type/2]).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    split_string(Name, ".", "", List),
    extension_from_name(List, Tmp),
    (downcase_atom(Tmp, Ext); Ext = Tmp).

extension_from_name([""|Tail], Ext):- !,
    length(Tail, Length),
    Length > 1,
    extension_from_name(Tail, Ext).

extension_from_name([_|Tail], Ext):-
    length(Tail, Length),
    Length > 0,
    construct_extension(Tail, Ext).

construct_extension([One], Ext):-
    atom_concat('.', One, Ext).

construct_extension([Head|Tail], Ext):-
    atomic_list_concat([Head|Tail], '.', NewNameAtomic),
    atom_concat('.', NewNameAtomic, Ext); construct_extension(Tail, Ext).

read_file(Path, MaxLength, String):-
    exists_file(Path),
    open(Path, read, Stream, []),
    read_string(Stream, MaxLength, String),
    close(Stream).

first_line(Path, FirstLine):-
    exists_file(Path),
    open(Path, read, Stream),
    read_line_to_string(Stream, FirstLine),
    close(Stream).


list_directory(Path, Dirs, Files):-
    directory_files(Path, Tmp),
    exclude([In]>>member(In, ['..', '.']), Tmp, Filtered),
    maplist({Path}/[In,Out]>>(
        atom_concat(Path, '/', PathTmp),
        atom_concat(PathTmp, In, Out)
    ), Filtered, Result),
    include(exists_directory, Result, Dirs),
    include(exists_file, Result, Files).

list_files_recursive(Path, Files):-
    list_directory(Path, Dirs, CurFiles),
    maplist(list_files_recursive, Dirs, MoreFiles),
    append(CurFiles, MoreFiles, Tree),
    flatten(Tree, Files).

is_binary_file(Stream):-
    read_stream_to_codes(Stream, Codes),
    memberchk(0, Codes).

file_type(Stream, Type):-
    is_binary_file(Stream) -> Type = "Binary"; Type = "Generic".

match_regex(Path, Pattern):-
    MaxLength is 10*1024,
    read_file(Path, MaxLength, String),
    match_regex_(String, Pattern).


match_regex_(String, Pattern):-
    re_compile(Pattern, RegEx, [multiline(true)]),
    re_match(RegEx, String),
    re_flush().
