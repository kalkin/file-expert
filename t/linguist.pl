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

:- begin_tests(linguist).
:- use_module('../lib/file').

test("Linguist samples", [
    nondet,
    forall(expected_results([File, Type])), 
    condition(\+skipped(File))
    ]):-
    string_to_atom(Type, A),
    guess_file(File, A).

:- end_tests(linguist).
