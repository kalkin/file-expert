:- begin_tests(linguist).
:- use_module('../lib/file').
:- use_module('../src/file_expert').
:- use_module('helpers').

test("Linguist samples", [nondet,forall(helpers:expected_results([File, Type]))]):-
    string_to_atom(Type, A),
    guess_file(File, A).

:- end_tests(linguist).
