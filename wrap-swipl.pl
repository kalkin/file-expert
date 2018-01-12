fileExtension("", "").

fileExtension(PATH, EXT):-
    file_base_name(PATH, NAME),
    split_string(NAME, ".", "", LIST),
    reverse(LIST, [EXT_STR|_]),
    not(atom_string(NAME, EXT_STR)),
    string_concat(".", EXT_STR, EXT).


fileFirstLine(PATH, FIRST_LINE):-
    exists_file(PATH),
    open(PATH, read, Stream),
    read_line_to_codes(Stream, Codes),
    atom_chars(FIRST_LINE, Codes).

my_concat(BANG, PATH, CMD, MAGIC_LINE):-
    string_concat(BANG, PATH, PREFIX),
    string_concat(PREFIX, CMD, MAGIC_LINE).
