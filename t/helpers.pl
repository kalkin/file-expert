:- module(helpers, [expected_results/1, sample_dir/1]).

sample_dir(Path):-
    absolute_file_name('../linguist/samples', Path).

as_choices([H|R], X):-
    X = H ; as_choices(R, X).

expected_results(X):-
    sample_dir(Path),
    file:list_files_recursive(Path, Tmp),
    maplist({Path}/[In, Out]>>(
        atom_concat(Path, Str, In),
        split_string(Str, "/", "", [_, Type|_]),
        Out = [In, Type]
    ), Tmp, Y), as_choices(Y, X).

