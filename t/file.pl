:- begin_tests(parse_extension).
:- use_module('../lib/file').

test("File with no extension"):-
    \+ parse_extension("asda", _).

test("One possible extension"):-
    parse_extension("asda.foo", ".foo").

fixme("No extension for '.foo'"):-
    \+ parse_extension(".foo", _).

fixme("One possible extension for a file starting with a dot"):-
    parse_extension(".asda.foo", ".foo").

test("Filename with version number separated by dots"):-
    parse_extension("linguist/samples/Easybuild/bzip2-1.0.6-GCC-4.9.2.eb", ".eb").

test("Handling of upper case extensions"):-
    parse_extension("linguist/samples/C/2D.C", ".c").

test("Two possible extensions, only last"):-
    parse_extension("asda.foo.bar", ".bar").

test("Two possible extensions, all"):-
    parse_extension("asda.foo.bar", ".foo.bar").

test("Invalid Extensions"):-
    \+ parse_extension("asda.foo.bar", ".asda.foo.bar"),
    \+ parse_extension("asda.foo.bar", "..asda.foo.bar").

run_tests.
:- end_tests(parse_extension).
