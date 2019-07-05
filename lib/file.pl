:- module(file, [parse_extension/2, read_file/3]).

parse_extension(Path, Ext):-
    file_base_name(Path, Name),
    extension(Name, Ext).

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
