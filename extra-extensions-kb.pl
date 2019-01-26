:- multifile file_extension/3.
:- multifile file_extension/2.
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

file_extension('WOFF', '.woff').
file_extension('.zunit', 'Shell').
file_extension('.cfg', 'HAProxy', '^(global|default|frontend|backend)\\s*\n\\s+').
file_extension('.inc', 'Pascal', "^\\s*end[.;]").
file_extension('.fcgi', 'PHP', "<\\?php").
file_extension('.mod', 'Linux Kernel Module', "^[a-zA-Z0-9/_-]+\\.(ko|o)$").
file_extension('.re', 'Reason', '\\blet\\b').
file_extension('.pluginspec', 'XML', '<\\\\\\w+>').
file_extension('.shader', 'ShaderLab', '^\\s*Shader\s+".*"$').
file_extension('.st', 'HTML', '<\\\\\\w+>').
file_extension('.t', 'Perl 6', '^\\s*use\\s+Test\\s*;').
file_extension('.v', 'Verilog', '^\\s*endmodule\\s*$').
