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


:- use_module('lib/file').
:- ['github-extensions-kb', 'extra-extensions-kb'].
:- use_module('src/file_expert').

say(File, unknown_type):-
    write(File), write('\t'), write('Unknown file'), nl.

say(File, multiple_possibilities):-
    write(File), write('\t'), write('Unknown file'), nl.

say(File, Type):-
    write(File), write('\t'), write(Type), nl.

guess([]):-
    write("No files specified"), nl,
    halt(1).

guess([Last]) :- !,
        guess_file(Last, Type),
        say(Last, Type).

guess([H|Rest]) :-
        guess_file(H, Type),
        say(H, Type),
        guess(Rest).
