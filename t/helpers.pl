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

:- module(helpers, [expected_results/1, sample_dir/1]).

sample_dir(Path):-
    absolute_file_name('linguist/samples', Path).

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

