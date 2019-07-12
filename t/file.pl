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

:- begin_tests(parse_extension).
:- use_module('../lib/file').

test("File with no extension"):-
    \+ parse_extension("asda", _).

test("No extension for '.foo'"):-
    \+ parse_extension(".foo", _).

test("One possible extension"):-
    parse_extension("asda.foo", ".foo").

test("One possible extension for a file starting with a dot"):-
    parse_extension(".asda.foo", X),
    assertion(X = ".foo").

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
