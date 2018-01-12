:- ["github-extensions-kb"].

reverse([], []).
reverse([X], [X]).
reverse([X|Xs], R):- reverse(Xs, T), append(T, [X], R).

fileType(PATH, LIST_OF_TYPES):-
    file_base_name(PATH, NAME),
    split_string(NAME, ".", "", LIST),
    reverse(LIST, [EXT_STR|_]),
    string_concat(".", EXT_STR, EXT),
    setof(TYPE, fileExtension(TYPE, EXT), LIST_OF_TYPES).
