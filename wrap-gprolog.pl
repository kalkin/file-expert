:- set_prolog_flag(double_quotes, codes).

fileExtension(PATH, EXT):-
    atom_codes(Codes, PATH),
    decompose_file_name(Codes, _, _, EXT).

fileFirstLine(PATH, FIRST_LINE):-
    atom_codes(Codes, PATH),
    file_permission(Codes, read),
    open(Codes, read, Stream),
    read_line_codes(Stream, [], R),
    atom_codes(FIRST_LINE, R).

read_line_codes(S, SoFar, L) :-
    get_code(S, C),
    (   C == -1
    ->  (  SoFar == []
        ->  L = end_of_file
        ;   reverse(SoFar, L)
        )
    ;   (  C == 0'\n
        -> reverse(SoFar, L)
        ;  read_line_codes(S, [C|SoFar], L)
        )
    ).
