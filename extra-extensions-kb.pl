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

file_extension('.brd', 'KiCad Legacy Layout').
file_extension('.cfg', 'INI').
file_extension('.frag', 'GLSL').
file_extension('.gif', 'GIF').
file_extension('.h', 'C').
file_extension('.m4', 'M4').
file_extension('.ml', 'OCaml').
file_extension('.mod', 'AMPL').
file_extension('.ms', 'Unix Assembly').
file_extension('.md', 'Markdown').
file_extension('.nb', 'Text').
file_extension('.pl', 'Perl').
file_extension('.pm', 'Perl').
file_extension('.png', 'PNG').
file_extension('.shader', 'GLSL').
file_extension('.st', 'Smalltalk').
file_extension('.tiff', 'tiff').
file_extension('.tif', 'tiff').
file_extension('.v', 'Coq').
file_extension('.zunit', 'Shell').
file_extension('WOFF', '.woff').

file_extension('.cfg', 'HAProxy', '^(global|default|frontend|backend)\\s*\n\\s+').
file_extension('.cp', 'C++', "^[ \\t]*(class|(using[ \\t]+)?namespace)\\s+\\w+").
file_extension('.cp', 'C++', "^[ \\t]*(private|public|protected):$").
file_extension('.cp', 'C++', "^[ \\t]*catch\\s*\\(").
file_extension('.cp', 'C++', "^[ \\t]*try").
file_extension('.cp', 'C++', "^\\s*#\\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>").
file_extension('.cp', 'C++', "^\\s*template\\s*<").
file_extension('.cp', 'C++', "std::\\w+").
file_extension('.fcgi', 'PHP', "<\\?php").
file_extension('.inc', 'C++', "^[ \\t]*(class|(using[ \\t]+)?namespace)\\s+\\w+").
file_extension('.inc', 'C++', "^[ \\t]*(private|public|protected):$").
file_extension('.inc', 'C++', "^[ \\t]*catch\\s*\\(").
file_extension('.inc', 'C++', "^[ \\t]*try").
file_extension('.inc', 'C++', "^\\s*#\\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>").
file_extension('.inc', 'C++', "^\\s*template\\s*<").
file_extension('.inc', 'C++', "std::\\w+").
file_extension('.inc', 'Pascal', "^\\s*end[.;]").
file_extension('.inc', 'SourcePawn', "#define\\s+").
file_extension('.js.frag', 'JavaScript', "(?m:\\/\\/|(\"|')use strict\\1|export\\s+default\\s|\\/\\*.*?\\*\\/)").
file_extension('.m4', 'M4Sugar', "(AC_DEFUN|AC_MSG|AC_IF|AC_INIT|AC_PREREQ|AC_PROG|AC_LIBINIT)").
file_extension('.mm', 'XML', "</map>").
file_extension('.mod', 'Linux Kernel Module', "^[a-zA-Z0-9/_-]+\\.(ko|o)$").
file_extension('.nb', 'Mathematica', "\\*\\)$").

% Order is important
file_extension('.re', 'Reason', "\\blet\\b").
file_extension('.re', 'C++', "^[ \\t]*(class|(using[ \\t]+)?namespace)\\s+\\w+").
file_extension('.re', 'C++', "^[ \\t]*(private|public|protected):$").
file_extension('.re', 'C++', "^[ \\t]*catch\\s*\\(").
file_extension('.re', 'C++', "^[ \\t]*try").
file_extension('.re', 'C++', "^\\s*#\\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>").
file_extension('.re', 'C++', "^\\s*template\\s*<").
file_extension('.re', 'C++', "std::\\w+").

file_extension('.bf', 'HyPhy', 'fprint').
file_extension('.bf', 'HyPhy', '/\\*').
file_extension('.b', 'Limbo', "^\\s*include\\s+\".*\"\\s*;").
file_extension('.md', 'GCC Machine Description', "^\\s*\\(define ").
file_extension('.b', 'Limbo', "^\\w+\\s*:\\s*module\\s*{").
file_extension('.brd', 'Eagle', '!DOCTYPE eagle').
file_extension('.pluginspec', 'XML', "</\\w+>").
file_extension('.sch', 'Eagle', '!DOCTYPE eagle').
file_extension('.sch', 'KiCad Schematic', "\\bWire\\s+\\w+Line\\n").
file_extension('.sch', 'KiCad Schematic', "^\\w [0-9 ]+$").
file_extension('.sch', 'KiCad Schematic', "^\\s*LIBS:").
file_extension('.sch', 'XML', "<\\?xml").
file_extension('.shader', 'ShaderLab', "^\\s*Shader\s+\".*\"$").
file_extension('.spec', 'RPM Spec', '^\\s*Name:\\s*.*$').
file_extension('.st', 'HTML', '</\\w+>').
file_extension('.t', 'Perl 6', '^\\s*use\\s+Test\\s*;').
file_extension('.v', 'Verilog', '^\\s*endmodule\\s*$').
file_extension('.vhost', 'ApacheConf', "</(VirtualHost|Directory)").
file_extension('.vhost', 'Nginx', "\\blocation\\b").
file_extension('.workflow', 'XML', '<\\?xml').

interpreter('Python', 'python2.1').
interpreter('Python', 'python2.2').
interpreter('Python', 'python2.3').
interpreter('Python', 'python2.4').
interpreter('Python', 'python2.5').
interpreter('Python', 'python2.6').
interpreter('Python', 'python2.7').
