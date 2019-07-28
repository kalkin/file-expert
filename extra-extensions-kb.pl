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

:- multifile extension/2.
:- multifile filename/2.
:- multifile interpreter/2.
:- multifile heuristic/3.

filename('README.md', 'Markdown').
filename('AUTHORS', 'Text').
filename('DESCRIPTION', 'Text').
filename('LICENSE', 'Text').
filename('LICENSE-APACHE', 'Text').
filename('LICENSE-BSD', 'Text').
filename('LICENSE-MIT', 'Text').
filename('VERSION', 'Text').
filename('version', 'Text').
filename('LS_COLORS', 'Dircolors').

extension('.png', 'PNG').
extension('.zunit', 'Shell').
extension('.woff', 'WOFF').

interpreter('python2.1', 'Python').
interpreter('python2.2', 'Python').
interpreter('python2.3', 'Python').
interpreter('python2.4', 'Python').
interpreter('python2.5', 'Python').
interpreter('python2.6', 'Python').
interpreter('python2.7', 'Python').

heuristic(File, '.spec', 'RPM Spec'):-
    file:match_regex(File, "\\s*Name\\:\\s*\\w+"), !.
