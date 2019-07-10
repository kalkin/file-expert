:- module(file, [parse_extension/2, read_file/3, first_line/2,
    list_files_recursive/2]).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    extension(Name, Ext).

extension(Name, _):-
    split_string(Name, ".", "", ["", _]), !, false.

extension(Name, Ext):-
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr1, ExtStr2|_]),
    not(atom_string(Name, ExtStr1)),
    not(atom_string(Name, ExtStr2)),
    atom_concat('.', ExtStr1, ExtTmp1),
    atom_concat('.', ExtStr2, ExtTmp2),
    atom_concat(ExtTmp2, ExtTmp1, Ext).

extension(Name, Ext):-
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr|_]),
    not(atom_string(Name, ExtStr)),
    atom_concat('.', ExtStr, Ext).

extension(Name, Ext):-
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr1, ExtStr2|_]),
    not(atom_string(Name, ExtStr1)),
    not(atom_string(Name, ExtStr2)),
    atom_concat('.', ExtStr1, ExtTmp1),
    atom_concat('.', ExtStr2, ExtTmp2),
    atom_concat(ExtTmp2, ExtTmp1, ExtRes),
    downcase_atom(ExtRes, Ext).

extension(Name, Ext):-
    split_string(Name, ".", "", List),
    reverse(List, [ExtStr|_]),
    not(atom_string(Name, ExtStr)),
    atom_concat('.', ExtStr, ExtTmp),
    downcase_atom(ExtTmp, Ext).

read_file(Path, MaxLength, String):-
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

list_files_recursive(Dir, Files):-
    absolute_file_name(Dir, Path),
    list_directory(Path, Dirs, CurFiles),
    maplist(list_files_recursive, Dirs, MoreFiles),
    append(CurFiles, MoreFiles, Tree),
    flatten(Tree, Files).
