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

:- multifile filename/2.
:- multifile interpreter/2.

filename('Markdown', 'README.md').
filename('Text', 'AUTHORS').
filename('Text', 'DESCRIPTION').
filename('Text', 'LICENSE').
filename('Text', 'LICENSE-APACHE').
filename('Text', 'LICENSE-BSD').
filename('Text', 'LICENSE-MIT').
filename('Text', 'VERSION').
filename('Text', 'version').
filename('Dircolors', 'LS_COLORS').

file_extension('.png', 'PNG').
file_extension('.zunit', 'Shell').
file_extension('WOFF', '.woff').


interpreter('Python', 'python2.1').
interpreter('Python', 'python2.2').
interpreter('Python', 'python2.3').
interpreter('Python', 'python2.4').
interpreter('Python', 'python2.5').
interpreter('Python', 'python2.6').
interpreter('Python', 'python2.7').
