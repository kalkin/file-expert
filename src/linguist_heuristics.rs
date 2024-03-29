//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref APACHECONF_1: Regex = Regex::new(r#"^\s*<(?:VirtualHost|Directory)\b"#).expect("Valid RegEx");
    static ref APEX_1: Regex = Regex::new(r#"^\s*(?:private|public|protected|global)\s+(?:(?:with|without) sharing\s+)?class\b"#).expect("Valid RegEx");
    static ref ACTIONSCRIPT_1: Regex = Regex::new(r#"^\s*(?:package(?:\s+[\w.]+)?\s+(?:{|$)|import\s+[\w.*]+\s*;|(?=.*?(?:intrinsic|extends))(intrinsic\s+)?class\s+[\w<>.]+(?:\s+extends\s+[\w<>.]+)?|(?:(?:public|protected|private|static)\s+)*(?:(?:var|const|local)\s+\w+\s*:\s*[\w<>.]+(?:\s*=.*)?\s*;|function\s+\w+\s*\((?:\s*\w+\s*:\s*[\w<>.]+\s*(,\s*\w+\s*:\s*[\w<>.]+\s*)*)?\)))"#).expect("Valid RegEx");
    static ref AGS_SCRIPT_1: Regex = Regex::new(r#"^(\/\/.+|((import|export)\s+)?(function|int|float|char)\s+((room|repeatedly|on|game)_)?([A-Za-z]+[A-Za-z_0-9]+)\s*[;\(])"#).expect("Valid RegEx");
    static ref AL_1: Regex = Regex::new(r#"\b(?i:(CODEUNIT|PAGE|PAGEEXTENSION|PAGECUSTOMIZATION|DOTNET|ENUM|ENUMEXTENSION|VALUE|QUERY|REPORT|TABLE|TABLEEXTENSION|XMLPORT|PROFILE|CONTROLADDIN))\b"#).expect("Valid RegEx");
    static ref ASCIIDOC_1: Regex = Regex::new(r#"^[=-]+(\s|\n)|{{[A-Za-z]"#).expect("Valid RegEx");
    static ref BASIC: Regex = Regex::new(r#"^\s*\d+"#).expect("Valid RegEx");
    static ref BEEF_1: Regex = Regex::new(r#"^\s*(class|namespace|void|static)\b"#).expect("Valid RegEx");
    static ref BITBAKE_1: Regex = Regex::new(r#"^\s*(# |include|require)\b"#).expect("Valid RegEx");
    static ref BLITZBASIC_1: Regex = Regex::new(r#"(<^\s*; |End Function)"#).expect("Valid RegEx");
    static ref BRAINFUCK_1: Regex = Regex::new(r#"(?:\+|>|<){4,}"#).expect("Valid RegEx");
    static ref COMMON_LISP_1: Regex = Regex::new(r#"^\s*\((?i:defun|in-package|defpackage) "#).expect("Valid RegEx");
    static ref COMMON_LISP_2: Regex = Regex::new(r#"\(def(un|macro)\s"#).expect("Valid RegEx");
    static ref COMMON_LISP_3: Regex = Regex::new(r#"^\s*\((?i:defun|in-package|defpackage) "#).expect("Valid RegEx");
    static ref COOL_1: Regex = Regex::new(r#"^class"#).expect("Valid RegEx");
    static ref COQ_1: Regex = Regex::new(r#"(?:^|\s)(?:Proof|Qed)\.(?:$|\s)|(?:^|\s)Require[ \t]+(Import|Export)\s"#).expect("Valid RegEx");
    static ref CUE_SHEET_1: Regex = Regex::new(r#"^\s*TRACK\s\d+\s.*$"#).expect("Valid RegEx");
    static ref CPP_1: Regex = Regex::new(r#"^\s*#\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>"#).expect("Valid RegEx");
    static ref CPP_2: Regex = Regex::new(r#"^\s*template\s*<"#).expect("Valid RegEx");
    static ref CPP_3: Regex = Regex::new(r#"^[ \t]*(try|constexpr)"#).expect("Valid RegEx");
    static ref CPP_4: Regex = Regex::new(r#"^[ \t]*catch\s*\("#).expect("Valid RegEx");
    static ref CPP_5: Regex = Regex::new(r#"^[ \t]*(class|(using[ \t]+)?namespace)\s+\w+"#).expect("Valid RegEx");
    static ref CPP_6: Regex = Regex::new(r#"^[ \t]*(private|public|protected):$"#).expect("Valid RegEx");
    static ref CPP_7: Regex = Regex::new(r#"std::\w+"#).expect("Valid RegEx");
    static ref CWEB_1: Regex = Regex::new(r#"^@(<|\w+\.)"#).expect("Valid RegEx");
    static ref C_PLUS__PLUS__1: Regex = Regex::new(r#"^\s*#(?:(?:if|ifdef|define|pragma)\s+\w|\s*include\s+<[^>]+>)"#).expect("Valid RegEx");
    static ref C_PLUS__PLUS__2: Regex = Regex::new(r#"^\s*template\s*<"#).expect("Valid RegEx");
    static ref C_SHARP__1: Regex = Regex::new(r#"^\s*(?:namespace|using|\/\/)\s*[\w\.]+\s*"#).expect("Valid RegEx");
    static ref DIRECTX_3D_FILE_1: Regex = Regex::new(r#"^xof 030(2|3)(?:txt|bin|tzip|bzip)\b"#).expect("Valid RegEx");
    static ref DTRACE_1: Regex = Regex::new(r#"^(\w+:\w*:\w*:\w*|BEGIN|END|provider\s+|(tick|profile)-\w+\s+{[^}]*}|#pragma\s+D\s+(option|attributes|depends_on)\s|#pragma\s+ident\s)"#).expect("Valid RegEx");
    static ref D_1: Regex = Regex::new(r#"^module\s+[\w.]*\s*;|import\s+[\w\s,.:]*;|\w+\s+\w+\s*\(.*\)(?:\(.*\))?\s*{[^}]*}|unittest\s*(?:\(.*\))?\s*{[^}]*}"#).expect("Valid RegEx");
    static ref EAGLE_1: Regex = Regex::new(r#"^\s*<!DOCTYPE\s+eagle\b"#).expect("Valid RegEx");
    static ref ECLIPSE_1: Regex = Regex::new(r#"^[^#]+:-"#).expect("Valid RegEx");
    static ref ECL_1: Regex = Regex::new(r#":="#).expect("Valid RegEx");
    static ref EIFFEL_1: Regex = Regex::new(r#"^(note|class|feature|end|inherit)"#).expect("Valid RegEx");
    static ref ERLANG_1: Regex = Regex::new(r#"^\s*(?:%%|main\s*\(.*?\)\s*->)"#).expect("Valid RegEx");
    static ref FAUST_1: Regex = Regex::new(r#"\bprocess\s*[(=]|\b(library|import)\s*\(\s*\"|\bdeclare\s+(name|version|author|copyright|license)\s+\""#).expect("Valid RegEx");
    static ref FILEBENCH_WML_1: Regex = Regex::new(r#"flowop"#).expect("Valid RegEx");
    static ref FILTERSCRIPT_1: Regex = Regex::new(r#"#include|#pragma\s+(rs|version)|__attribute__"#).expect("Valid RegEx");
    static ref FLUX_1: Regex = Regex::new(r#"^\s*(typedef|atomic)\b"#).expect("Valid RegEx");
    static ref FLUENT: Regex = Regex::new(r#"^-?[a-zA-Z][a-zA-Z0-9_-]* *=|\{\$-?[a-zA-Z][-\w]*(?:\.[a-zA-Z][-\w]*)?\}"#).expect("Valid RegEx");
    static ref FREE_BASIC: Regex = Regex::new(r#"^[ \t]*#(?:define|endif|endmacro|ifn?def|if|include|lang|macro)\s"#).expect("Valid RegEx");
    static ref FREE_MARKER: Regex = Regex::new(r#"^(?:<|[a-zA-Z-][a-zA-Z0-9_-]+[ \t]+\w)|\${\w+[^\n]*?}|^[ \t]*(?:<#--.*?-->|<#([a-z]+)(?=\s|>)[^>]*>.*?</#\1>|\[#--.*?--\]|\[#([a-z]+)(?=\s|\])[^\]]*\].*?\[#\2\])"#).expect("Valid RegEx");
    static ref FORTH_1: Regex = Regex::new(r#"^: "#).expect("Valid RegEx");
    static ref FORTH_2: Regex = Regex::new(r#"^: "#).expect("Valid RegEx");
    static ref FORTH_3: Regex = Regex::new(r#"^(: |also |new-device|previous )"#).expect("Valid RegEx");
    static ref FORTH_4: Regex = Regex::new(r#"^(: |new-device)"#).expect("Valid RegEx");
    static ref FORTRAN_1: Regex = Regex::new(r#"^(?i:[c*][^abd-z]|      (subroutine|program|end|data)\s|\s*!)"#).expect("Valid RegEx");
    static ref FREGE_1: Regex = Regex::new(r#"^\s*(import|module|package|data|type) "#).expect("Valid RegEx");
    static ref F_SHARP__1: Regex = Regex::new(r#"^\s*(#light|import|let|module|namespace|open|type)"#).expect("Valid RegEx");
    static ref GAP_1: Regex = Regex::new(r#"\s*(Declare|BindGlobal|KeyDependentOperation)"#).expect("Valid RegEx");
    static ref GAP_2: Regex = Regex::new(r#"gap> "#).expect("Valid RegEx");
    static ref GAP_3: Regex = Regex::new(r#"\s+:=\s+"#).expect("Valid RegEx");
    static ref GCC_MACHINE_DESCRIPTION_1: Regex = Regex::new(r#"^(;;|\(define_)"#).expect("Valid RegEx");
    static ref GDSCRIPT_1: Regex = Regex::new(r#"\s*(extends|var|const|enum|func|class|signal|tool|yield|assert|onready)"#).expect("Valid RegEx");
    static ref GENIE_1: Regex = Regex::new(r#"^\[indent=[0-9]+\]"#).expect("Valid RegEx");
    static ref GENIE_2: Regex = Regex::new(r#"^\t*(def|class|construct|init)\b"#).expect("Valid RegEx");
    static ref GENIE_3: Regex = Regex::new(r#"^(init|import)\b"#).expect("Valid RegEx");
    static ref GERBER_IMAGE_1: Regex = Regex::new(r#"^[DGMT][0-9]{2}\*$"#).expect("Valid RegEx");
    static ref GLSL_1: Regex = Regex::new(r#"^\s*(#version|precision|uniform|varying|vec[234])"#).expect("Valid RegEx");
    static ref GLSL_2: Regex = Regex::new(r#"^#version\s+[0-9]+\b"#).expect("Valid RegEx");
    static ref GNUPLOT_1: Regex = Regex::new(r#"^s?plot\b"#).expect("Valid RegEx");
    static ref GNUPLOT_2: Regex = Regex::new(r#"^set\s+(term|terminal|out|output|[xy]tics|[xy]label|[xy]range|style)\b"#).expect("Valid RegEx");
    static ref GOSU_1: Regex = Regex::new(r#"^uses (java|gw)\."#).expect("Valid RegEx");
    static ref GRAPH_MODELING_LANGUAGE_1: Regex = Regex::new(r#"(?i:^\s*(graph|node)\b\s*\[?$)"#).expect("Valid RegEx");
    static ref HACK_1: Regex = Regex::new(r#"<\?hh"#).expect("Valid RegEx");
    static ref HACK_2: Regex = Regex::new(r#"<\?hh"#).expect("Valid RegEx");
    static ref HAPROXY_1: Regex = Regex::new(r#"^\s*(?:frontend|backend|listen)\s+(?:\w|\d)+"#).expect("Valid RegEx");
    static ref HIVEQL_1: Regex = Regex::new(r#"(?i:SELECT\s+[\w*,]+\s+FROM|(CREATE|ALTER|DROP)\s(DATABASE|SCHEMA|TABLE))"#).expect("Valid RegEx");
    static ref HTML_1: Regex = Regex::new(r#"^\s*</?(?i:html|head|title|body|span|div)\b"#).expect("Valid RegEx");
    static ref IDL_1: Regex = Regex::new(r#"^\s*function[ \w,]+$"#).expect("Valid RegEx");
    static ref INI_1: Regex = Regex::new(r#"last_client="#).expect("Valid RegEx");
    static ref INI_2: Regex = Regex::new(r#"^[;\[]"#).expect("Valid RegEx");
    static ref JASMIN_1: Regex = Regex::new(r#"^\.\w+\b"#).expect("Valid RegEx");
    static ref JAVASCRIPT_1: Regex = Regex::new(r#"(?m:\/\/|(\"|')use strict\1|export\s+default\s|\/\*.*?\*\/)"#).expect("Valid RegEx");
    static ref JAVA_PROPERTIES_1: Regex = Regex::new(r#"^[^#!][^:]*:"#).expect("Valid RegEx");
    static ref JSON_1: Regex = Regex::new(r#"\A\s*[{\[]"#).expect("Valid RegEx");
    static ref JSON_2: Regex = Regex::new(r#"\"modelName\"\:\s*\"GM"#).expect("Valid RegEx");
    static ref JSONIQ: Regex = Regex::new(r#"(^\s*(import\s+module|variable\s+\$\w+))"#).expect("Valid RegEx");
    static ref KEY_EQUALS_VALUE_1: Regex = Regex::new(r#"^[^#!;][^=]*="#).expect("Valid RegEx");
    static ref KUSTO_1: Regex = Regex::new(r#"(^\|\s*(where|extend|project|limit|summarize))|(^\.\w+)"#).expect("Valid RegEx");
    static ref LEX_1: Regex = Regex::new(r#"^(%[%{}]xs|<.*>)"#).expect("Valid RegEx");
    static ref LIMBO_1: Regex = Regex::new(r#"^\w+\s*:\s*module\s*{?$"#).expect("Valid RegEx");
    static ref LINKER_SCRIPT_1: Regex = Regex::new(r#"OUTPUT_ARCH\(|OUTPUT_FORMAT\(|SECTIONS"#).expect("Valid RegEx");
    static ref LINUX_KERNEL_MODULE_1: Regex = Regex::new(r#"^.+\.ko"#).expect("Valid RegEx");
    static ref LOGOS_1: Regex = Regex::new(r#"^%(end|ctor|hook|group)\b"#).expect("Valid RegEx");
    static ref LOOMSCRIPT_1: Regex = Regex::new(r#"^\s*package\s*[\w\.\/\*\s]*\s*{?$"#).expect("Valid RegEx");
    static ref LTSPICE_SYMBOL_1: Regex = Regex::new(r#"^SymbolType[ \t]"#).expect("Valid RegEx");
    static ref M4SUGAR_1: Regex = Regex::new(r#"AC_DEFUN|AC_PREREQ|AC_INIT"#).expect("Valid RegEx");
    static ref M4SUGAR_2: Regex = Regex::new(r#"^_?m4_"#).expect("Valid RegEx");
    static ref M68K_1: Regex = Regex::new(r#"(?im)\bmoveq(?:\.l)?\s+#(?:\$-?[0-9a-f]{1,3}|%[0-1]{1,8}|-?[0-9]{1,3}),\s*d[0-7]\b"#).expect("Valid RegEx");
    static ref M68K_2: Regex = Regex::new(r#"(?im)^\s*move(?:\.[bwl])?\s+(?:sr|usp),\s*[^\s]+"#).expect("Valid RegEx");
    static ref M68K_3: Regex = Regex::new(r#"(?im)^\s*move\.[bwl]\s+.*\b[ad]\d"#).expect("Valid RegEx");
    static ref M68K_4: Regex = Regex::new(r#"(?im)^\s*movem\.[bwl]\b"#).expect("Valid RegEx");
    static ref M68K_5: Regex = Regex::new(r#"(?im)^\s*move[mp](?:\.[wl])?\b"#).expect("Valid RegEx");
    static ref M68K_6: Regex = Regex::new(r#"(?im)^\s*btst\b"#).expect("Valid RegEx");
    static ref M68K_7: Regex = Regex::new(r#"(?im)^\s*dbra\b"#).expect("Valid RegEx");
    static ref MAKEFILE_1: Regex = Regex::new(r#"([\/\\].*:\s+.*\s\\$|: \\$|^[ %]:|^[\w\s\/\\.]+\w+\.\w+\s*:\s+[\w\s\/\\.]+\w+\.\w+)"#).expect("Valid RegEx");
    static ref MAN_HEADING_1: Regex = Regex::new(r#"^[.'][ \t]*SH +(?:[^\"\s]+|\"[^\"\s]+)"#).expect("Valid RegEx");
    static ref MAN_TITLE_1: Regex = Regex::new(r#"^[.'][ \t]*TH +(?:[^\"\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\s@]+@)"#).expect("Valid RegEx");
    static ref MARKDOWN_1: Regex = Regex::new(r#"(^[-A-Za-z0-9=#!\*\[|>])|<\/"#).expect("Valid RegEx");
    static ref MARKDOWN_2: Regex = Regex::new(r#"\A\z"#).expect("Valid RegEx");
    static ref MATHEMATICA_1: Regex = Regex::new(r#"\(\*"#).expect("Valid RegEx");
    static ref MATHEMATICA_2: Regex = Regex::new(r#"\*\)$"#).expect("Valid RegEx");
    static ref MATLAB_1: Regex = Regex::new(r#"^\s*%"#).expect("Valid RegEx");
    static ref MATLAB_2: Regex = Regex::new(r#"^\s*end\b"#).expect("Valid RegEx");
    static ref MDOC_DATE_1: Regex = Regex::new(r#"^[.'][ \t]*Dd +(?:[^\"\s]+|\"[^\"]+\")"#).expect("Valid RegEx");
    static ref MDOC_HEADING_1: Regex = Regex::new(r#"^[.'][ \t]*Sh +(?:[^\"\s]|\"[^\"]+\")"#).expect("Valid RegEx");
    static ref MDOC_TITLE_1: Regex = Regex::new(r#"^[.'][ \t]*Dt +(?:[^\"\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\s@]+@)"#).expect("Valid RegEx");
    static ref MERCURY_1: Regex = Regex::new(r#":-\s+\w+"#).expect("Valid RegEx");
    static ref MICROSOFT_DEVELOPER_STUDIO_PROJECT_1: Regex = Regex::new(r#"# Microsoft Developer Studio Generated Build File"#).expect("Valid RegEx");
    static ref MODULA_2_1: Regex = Regex::new(r#"^\s*(?i:MODULE|END) [\w\.]+;"#).expect("Valid RegEx");
    static ref MOOCODE_1: Regex = Regex::new(r#"^\s*@\w+\s+"#).expect("Valid RegEx");
    static ref MOTOKO: Regex = Regex::new(r#"\b(func|shared)\b"#).expect("Valid RegEx");
    static ref MUF_1: Regex = Regex::new(r#"^: "#).expect("Valid RegEx");
    static ref M_1: Regex = Regex::new(r#"^\s*;"#).expect("Valid RegEx");
    static ref NASL_1: Regex = Regex::new(r#"^\s*include\s*\(\s*(?:\"|')[\\/\w\-\.:\s]+\.(?:nasl|inc)\s*(?:\"|')\s*\)\s*;"#).expect("Valid RegEx");
    static ref NASL_2: Regex = Regex::new(r#"^\s*(?:global|local)_var\s+(?:\w+(?:\s*=\s*[\w\-\"']+)?\s*)(?:,\s*\w+(?:\s*=\s*[\w\-\"']+)?\s*)*+\s*;"#).expect("Valid RegEx");
    static ref NASL_3: Regex = Regex::new(r#"^\s*namespace\s+\w+\s*{"#).expect("Valid RegEx");
    static ref NASL_4: Regex = Regex::new(r#"^\s*object\s+\w+\s*(?:extends\s+\w+(?:::\w+)?)?\s*{"#).expect("Valid RegEx");
    static ref NASL_5: Regex = Regex::new(r#"^\s*(?:public\s+|private\s+|\s*)function\s+\w+\s*\([\w\s,]*\)\s*{"#).expect("Valid RegEx");
    static ref NEMERLE_1: Regex = Regex::new(r#"^(module|namespace|using)\s"#).expect("Valid RegEx");
    static ref NEWLISP_1: Regex = Regex::new(r#"^\s*\(define "#).expect("Valid RegEx");
    static ref NL_1: Regex = Regex::new(r#"^(b|g)[0-9]+ "#).expect("Valid RegEx");
    static ref OBJECTIVEC_1: Regex = Regex::new(r#"^\s*(@(interface|class|protocol|property|end|synchronised|selector|implementation)\b|#import\s+.+\.h[\">])"#).expect("Valid RegEx");
    static ref OBJECTIVEC_2: Regex = Regex::new(r#"^\s*#\s*(?:include|import)\s+<(?:CoreVideo|Foundation|Cocoa|CoreFoundation)/.*\.h>\s*$"#).expect("Valid RegEx");
    static ref OBJECTSCRIPT_1: Regex = Regex::new(r#"^Class\s"#).expect("Valid RegEx");
    static ref OBJECT_DATA_INSTANCE_NOTATION_1: Regex = Regex::new(r#"(?:^|<)\s*[A-Za-z0-9_]+\s*=\s*<"#).expect("Valid RegEx");
    static ref OCAML_1: Regex = Regex::new(r#"(^\s*module)|let rec |match\s+(\S+\s)+with"#).expect("Valid RegEx");
    static ref ODIN_1: Regex = Regex::new(r#"package\s+\w+|\b(?:im|ex)port\s*\"[\w:./]+\"|\w+\s*::\s*(?:proc|struct)\s*\(|^\s*//\s"#).expect("Valid RegEx");
    static ref OPENCL_1: Regex = Regex::new(r#"\/\* |\/\/ |^\}"#).expect("Valid RegEx");
    static ref OPENEDGE_ABL_1: Regex = Regex::new(r#"&ANALYZE-SUSPEND _UIB-CODE-BLOCK _CUSTOM _DEFINITIONS"#).expect("Valid RegEx");
    static ref PASCAL_1: Regex = Regex::new(r#"^\s*end[.;]\s*$"#).expect("Valid RegEx");
    static ref PASCAL_2: Regex = Regex::new(r#"(?i:^\s*{\$(?:mode|ifdef|undef|define)[ ]+[a-z0-9_]+})"#).expect("Valid RegEx");
    static ref PAWN_1: Regex = Regex::new(r#"^\s*#include\s+<\w+>"#).expect("Valid RegEx");
    static ref PAWN_2: Regex = Regex::new(r#"^\s*public\s+(?:\w|\d)+\("#).expect("Valid RegEx");
    static ref PAWN_3: Regex = Regex::new(r#"^\s*stock\s+(?:\w|\d)+:"#).expect("Valid RegEx");
    static ref PERL5_1: Regex = Regex::new(r#"\buse\s+(?:strict\b|sigtrap\b|v?5\.)"#).expect("Valid RegEx");
    static ref PERL6_1: Regex = Regex::new(r#"^\s*(?:use\s+v6\b|\bmodule\b|\b(?:my\s+)?class\b)"#).expect("Valid RegEx");
    static ref PHP_1: Regex = Regex::new(r#"^<\?(?:php)?"#).expect("Valid RegEx");
    static ref PHP_2: Regex = Regex::new(r#"<\?[^h]"#).expect("Valid RegEx");
    static ref PICOLISP_1: Regex = Regex::new(r#"^\((de|class|rel|code|data|must)\s"#).expect("Valid RegEx");
    static ref PLPGSQL_1: Regex = Regex::new(r#"(?i:^\\i\b|AS\s+\$\$|LANGUAGE\s+'?plpgsql'?|BEGIN(\s+WORK)?\s*;)"#).expect("Valid RegEx");
    static ref PLSQL_1: Regex = Regex::new(r#"(?i:\$\$PLSQL_|XMLTYPE|systimestamp|\.nextval|CONNECT\s+BY|AUTHID\s+(DEFINER|CURRENT_USER)|constructor\W+function)"#).expect("Valid RegEx");
    static ref POD_6_1: Regex = Regex::new(r#"^[\s&&[^\n]]*=(comment|begin pod|begin para|item\d+)"#).expect("Valid RegEx");
    static ref POV_RAY_SDL_1: Regex = Regex::new(r#"^\s*#(declare|local|macro|while)\s"#).expect("Valid RegEx");
    static ref PROGUARD_1: Regex = Regex::new(r#"^-(include\b.*\.pro$|keep\b|keepclassmembers\b|keepattributes\b)"#).expect("Valid RegEx");
    static ref PROLOG_1: Regex = Regex::new(r#"^[^#]*:-"#).expect("Valid RegEx");
    static ref PUBLIC_KEY_1: Regex = Regex::new(r#"^(----[- ]BEGIN|ssh-(rsa|dss)) "#).expect("Valid RegEx");
    static ref PUPPET_1: Regex = Regex::new(r#"^\s+\w+\s+=>\s"#).expect("Valid RegEx");
    static ref PYTHON_1: Regex = Regex::new(r#"(?m:^(import|from|class|def)\s)"#).expect("Valid RegEx");
    static ref PYTHON_SPEC_1: Regex = Regex::new(r#"=\s*Analysis\("#).expect("Valid RegEx");
    static ref PYTHON_SPEC_2: Regex = Regex::new(r#"=\s*PYZ\("#).expect("Valid RegEx");
    static ref PYTHON_SPEC_3: Regex = Regex::new(r#"=\s*EXE\("#).expect("Valid RegEx");
    static ref PYTHON_SPEC_4: Regex = Regex::new(r#"=\s*COLLECT\("#).expect("Valid RegEx");
    static ref QMAKE_1: Regex = Regex::new(r#"HEADERS"#).expect("Valid RegEx");
    static ref QMAKE_2: Regex = Regex::new(r#"SOURCES"#).expect("Valid RegEx");
    static ref QT_SCRIPT_1: Regex = Regex::new(r#"(\w+\.prototype\.\w+|===|\bvar\b)"#).expect("Valid RegEx");
    static ref Q_1: Regex = Regex::new(r#"((?i:[A-Z.][\w.]*:{)|(^|\n)\\(cd?|d|l|p|ts?) )"#).expect("Valid RegEx");
    static ref Q_SHARP__1: Regex = Regex::new(r#"^((\/{2,3})?\s*(namespace|operation)\b)"#).expect("Valid RegEx");
    static ref RAKU_1: Regex = Regex::new(r#"^\s*(?:use\s+v6\b|\bmodule\b|\bmy\s+class\b)"#).expect("Valid RegEx");
    static ref REASON_1: Regex = Regex::new(r#"^\s*module\s+type\s"#).expect("Valid RegEx");
    static ref REASON_2: Regex = Regex::new(r#"^\s*(?:include|open)\s+\w+\s*;\s*$"#).expect("Valid RegEx");
    static ref REASON_3: Regex = Regex::new(r#"^\s*let\s+(?:module\s\w+\s*=\s*{|\w+:\s+.*=.*;\s*$)"#).expect("Valid RegEx");
    static ref REBOL_1: Regex = Regex::new(r#"(?i:\bRebol\b)"#).expect("Valid RegEx");
    static ref RENDERSCRIPT_1: Regex = Regex::new(r#"#include|#pragma\s+(rs|version)|__attribute__"#).expect("Valid RegEx");
    static ref RESCRIPT_1: Regex = Regex::new(r#"^\s*(let|module|type)\s+\w*\s+=\s+"#).expect("Valid RegEx");
    static ref RESCRIPT_2: Regex = Regex::new(r#"^\s*(?:include|open)\s+\w+\s*$"#).expect("Valid RegEx");
    static ref ROFF_1: Regex = Regex::new(r#"^\.(?:[A-Za-z]{2}(?:\s|$)|\\\")"#).expect("Valid RegEx");
    static ref ROFF_2: Regex = Regex::new(r#"^\.[A-Za-z]{2}(\s|$)"#).expect("Valid RegEx");
    static ref ROFF_3: Regex = Regex::new(r#"^[.'][A-Za-z]{2}(\s|$)"#).expect("Valid RegEx");
    static ref ROFF_4: Regex = Regex::new(r#"^[.']"#).expect("Valid RegEx");
    static ref ROFF_5: Regex = Regex::new(r#"^\.\\\" "#).expect("Valid RegEx");
    static ref RPC_1: Regex = Regex::new(r#"\b(program|version)\s+\w+\s*{|\bunion\s+\w+\s+switch\s*\("#).expect("Valid RegEx");
    static ref RPM_SPEC_1: Regex = Regex::new(r#"\s*Name\:\s*\w+"#).expect("Valid RegEx");
    static ref RUNOFF_1: Regex = Regex::new(r#"(?i:^\.!|^\f|\f$|^\.end lit(?:eral)?\b|^\.[a-zA-Z].*?;\.[a-zA-Z](?:[; \t])|\^\*[^\s*][^*]*\\\*(?=$|\s)|^\.c;[ \t]*\w+)"#).expect("Valid RegEx");
    static ref RUST_1: Regex = Regex::new(r#"^(use |fn |mod |pub |macro_rules|impl|#!?\[)"#).expect("Valid RegEx");
    static ref R_1: Regex = Regex::new(r#"<-|^\s*#"#).expect("Valid RegEx");
    static ref SCALA_1: Regex = Regex::new(r#"(^\s*import (scala|java)\.|^\s*class\b)"#).expect("Valid RegEx");
    static ref SCHEME_1: Regex = Regex::new(r#"^\s*\((?:define|let)"#).expect("Valid RegEx");
    static ref SLICE_1: Regex = Regex::new(r#"^\s*(module|struct|interface|sequence|enum)\s+\w+"#).expect("Valid RegEx");
    static ref SMALLTALK_2: Regex = Regex::new(r#"\A\s*[\[{(^\"'\w#]|[a-zA-Z_]\w*\s*:=\s*[a-zA-Z_]\w*|class\s*>>\s*[a-zA-Z_]\w*|^[a-zA-Z_]\w*\s+[a-zA-Z_]\w*:|^Class\s*{|if(?:True|False):\s*\["#).expect("Valid RegEx");
    static ref SMALLTALK_1: Regex = Regex::new(r#"![\w\s]+methodsFor: "#).expect("Valid RegEx");
    static ref SOLIDITY_1: Regex = Regex::new(r#"\bpragma\s+solidity\b|\b(?:abstract\s+)?contract\s+(?!\d)[a-zA-Z0-9$_]+(?:\s+is\s+(?:[a-zA-Z0-9$_][^\{]*?)?)?\s*\{"#).expect("Valid RegEx");
    static ref SOURCEPAWN_1: Regex = Regex::new(r#"^public\s+(?:SharedPlugin(?:\s+|:)__pl_\w+\s*=(?:\s*{)?|(?:void\s+)?__pl_\w+_SetNTVOptional\(\)(?:\s*{)?)"#).expect("Valid RegEx");
    static ref SOURCEPAWN_2: Regex = Regex::new(r#"^methodmap\s+\w+\s+<\s+\w+"#).expect("Valid RegEx");
    static ref SOURCEPAWN_3: Regex = Regex::new(r#"^\s*MarkNativeAsOptional\s*\("#).expect("Valid RegEx");
    static ref SQLPL_1: Regex = Regex::new(r#"(?i:ALTER\s+MODULE|MODE\s+DB2SQL|\bSYS(CAT|PROC)\.|ASSOCIATE\s+RESULT\s+SET|\bEND!\s*$)"#).expect("Valid RegEx");
    static ref STANDARD_ML_1: Regex = Regex::new(r#"=> |case\s+(\S+\s)+of"#).expect("Valid RegEx");
    static ref STRING_TEMPLATE: Regex = Regex::new(r#"\$\w+[($]|(.)!\s*.+?\s*!\1|<!\s*.+?\s*!>|\[!\s*.+?\s*!\]|\{!\s*.+?\s*!\}"#).expect("Valid RegEx");
    static ref SUBRIP_TEXT_1: Regex = Regex::new(r#"^(\d{2}:\d{2}:\d{2},\d{3})\s*(-->)\s*(\d{2}:\d{2}:\d{2},\d{3})$"#).expect("Valid RegEx");
    static ref SUPERCOLLIDER_1: Regex = Regex::new(r#"(?i:\^(this|super)\.|^\s*~\w+\s*=\.)"#).expect("Valid RegEx");
    static ref SWIG_1: Regex = Regex::new(r#"^[ \t]*%[a-z_]+\b|^%[{}]$"#).expect("Valid RegEx");
    static ref TEXT_1: Regex = Regex::new(r#"THE_TITLE"#).expect("Valid RegEx");
    static ref TEX_1: Regex = Regex::new(r#"\\\w+{"#).expect("Valid RegEx");
    static ref TEX_2: Regex = Regex::new(r#"^\\(contentsline|defcounter|beamer|boolfalse)"#).expect("Valid RegEx");
    static ref TSQL_1: Regex = Regex::new(r#"(?i:^\s*GO\b|BEGIN(\s+TRY|\s+CATCH)|OUTPUT\s+INSERTED|DECLARE\s+@|\[dbo\])"#).expect("Valid RegEx");
    static ref TSX_1: Regex = Regex::new(r#"^\s*(import.+(from\s+|require\()['\"]react|\/\/\/\s*<reference\s)"#).expect("Valid RegEx");
    static ref TURING_1: Regex = Regex::new(r#"^\s*%[ \t]+|^\s*var\s+\w+(\s*:\s*\w+)?\s*:=\s*\w+"#).expect("Valid RegEx");
    static ref UNITY3D_ASSET_1: Regex = Regex::new(r#"tag:unity3d.com"#).expect("Valid RegEx");
    static ref UNIX_ASSEMBLY_1: Regex = Regex::new(r#"^\s*\.(?:include\s|globa?l\s|[A-Za-z][_A-Za-z0-9]*:)"#).expect("Valid RegEx");
    static ref VERILOG_1: Regex = Regex::new(r#"^[ \t]*module\s+[^\s()]+\s+\#?\(|^[ \t]*`(?:define|ifdef|ifndef|include|timescale)|^[ \t]*always[ \t]+@|^[ \t]*initial[ \t]+(begin|@)"#).expect("Valid RegEx");
    static ref VBA_1: Regex = Regex::new(r#"^\s*End\s+(?:If|Sub|Function|Select|Enum|Property)"#).expect("Valid RegEx");
    static ref VIM_HELP_FILE_1: Regex = Regex::new(r#"(?:(?:^|[ \t])(?:vi|Vi(?=m))(?:m[<=>]?[0-9]+|m)?|[ \t]ex)(?=:(?=[ \t]*set?[ \t][^\r\n:]+:)|:(?![ \t]*set?[ \t]))(?:(?:[ \t]*:[ \t]*|[ \t])\w*(?:[ \t]*=(?:[^\\\s]|\\.)*)?)*[ \t:](?:filetype|ft|syntax)[ \t]*=(help)(?=$|\s|:)"#).expect("Valid RegEx");
    static ref VIM_SCRIPT_1: Regex = Regex::new(r#"^UseVimball"#).expect("Valid RegEx");
    static ref V_1: Regex = Regex::new(r#"\$(?:if|else)[ \t]|^[ \t]*fn\s+[^\s()]+\(.*?\).*?\{|^[ \t]*for\s*\{"#).expect("Valid RegEx");
    static ref WORLD_OF_WARCRAFT_ADDON_DATA_1: Regex = Regex::new(r#"^## |@no-lib-strip@"#).expect("Valid RegEx");
    static ref XBASE_1: Regex = Regex::new(r#"^\s*#\s*(?i:if|ifdef|ifndef|define|command|xcommand|translate|xtranslate|include|pragma|undef)\b"#).expect("Valid RegEx");
    static ref XML_1: Regex = Regex::new(r#"^(\s*)(?i:<Project|<Import|<Property|<?xml|xmlns)"#).expect("Valid RegEx");
    static ref XML_2: Regex = Regex::new(r#"(?i:^\s*(<\?xml|xmlns))"#).expect("Valid RegEx");
    static ref XML_3: Regex = Regex::new(r#"<!ENTITY "#).expect("Valid RegEx");
    static ref XML_4: Regex = Regex::new(r#"^\s*<\?xml\s+version"#).expect("Valid RegEx");
    static ref XML_5: Regex = Regex::new(r#"^\s*<\?xml"#).expect("Valid RegEx");
    static ref XML_6: Regex = Regex::new(r#"<TS\b"#).expect("Valid RegEx");
    static ref XML_7: Regex = Regex::new(r#"(?i:^\s*<\?xml\s+version)"#).expect("Valid RegEx");
    static ref XML_8: Regex = Regex::new(r#"^\s*<\w+\b"#).expect("Valid RegEx");
    static ref XML_PROPERTY_LIST_1: Regex = Regex::new(r#"<!DOCTYPE\s+plist"#).expect("Valid RegEx");
    static ref X_PIXMAP_1: Regex = Regex::new(r#"^\s*\/\* XPM \*\/"#).expect("Valid RegEx");
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::if_same_then_else)]
#[allow(clippy::if_then_some_else_none)]
pub fn linguist_heuristic(ext: &str, content: &[String]) -> Option<&'static str> {
    match ext {
        ".1" | ".2" | ".3" | ".4" | ".5" | ".6" | ".7" | ".8" | ".9" => {
            if match_lines(&MDOC_DATE_1, content)
                && match_lines(&MDOC_TITLE_1, content)
                && match_lines(&MDOC_HEADING_1, content)
            {
                Some("Roff Manpage")
            } else if match_lines(&MAN_TITLE_1, content) && match_lines(&MAN_HEADING_1, content) {
                Some("Roff Manpage")
            } else if match_lines(&ROFF_1, content) {
                Some("Roff")
            } else {
                None
            }
        }
        ".1in" | ".1m" | ".1x" | ".3in" | ".3m" | ".3p" | ".3pm" | ".3qt" | ".3x" | ".man"
        | ".mdoc" => {
            if match_lines(&MDOC_DATE_1, content)
                && match_lines(&MDOC_TITLE_1, content)
                && match_lines(&MDOC_HEADING_1, content)
            {
                Some("Roff Manpage")
            } else if match_lines(&MAN_TITLE_1, content) && match_lines(&MAN_HEADING_1, content) {
                Some("Roff Manpage")
            } else {
                Some("Roff")
            }
        }
        ".al" => {
            if match_lines(&AL_1, content) {
                Some("AL")
            } else {
                Some("Perl")
            }
        }
        ".as" => {
            if match_lines(&ACTIONSCRIPT_1, content) {
                Some("ActionScript")
            } else {
                Some("AngelScript")
            }
        }
        ".asc" => {
            if match_lines(&PUBLIC_KEY_1, content) {
                Some("Public Key")
            } else if match_lines(&ASCIIDOC_1, content) {
                Some("AsciiDoc")
            } else if match_lines(&AGS_SCRIPT_1, content) {
                Some("AGS Script")
            } else {
                None
            }
        }
        ".asm" => {
            if match_lines(&M68K_1, content)
                || match_lines(&M68K_2, content)
                || match_lines(&M68K_3, content)
                || match_lines(&M68K_4, content)
                || match_lines(&M68K_5, content)
                || match_lines(&M68K_6, content)
                || match_lines(&M68K_7, content)
            {
                Some("Motorola 68K Assembly")
            } else {
                Some("Assembly")
            }
        }
        ".asy" => {
            if match_lines(&LTSPICE_SYMBOL_1, content) {
                Some("LTspice Symbol")
            } else {
                Some("Asymptote")
            }
        }
        ".bas" => {
            if match_lines(&FREE_BASIC, content) {
                Some("FreeBasic")
            } else if match_lines(&BASIC, content) {
                Some("BASIC")
            } else {
                Some("VBA")
            }
        }
        ".bb" => {
            if match_lines(&BLITZBASIC_1, content) {
                Some("BlitzBasic")
            } else if match_lines(&BITBAKE_1, content) {
                Some("BitBake")
            } else {
                None
            }
        }
        ".brd" => {
            if match_lines(&EAGLE_1, content) {
                Some("Eagle")
            } else {
                Some("KiCad Legacy Layout")
            }
        }
        ".builds" => {
            if match_lines(&XML_1, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".cake" => {
            if match_lines(&C_SHARP__1, content) {
                Some("C#")
            } else {
                Some("CoffeeScript")
            }
        }
        ".cfg" => {
            if match_lines(&HAPROXY_1, content) {
                Some("HAProxy")
            } else {
                Some("INI")
            }
        }
        ".ch" => {
            if match_lines(&XBASE_1, content) {
                Some("xBase")
            } else {
                Some("Charity")
            }
        }
        ".cl" => {
            if match_lines(&COMMON_LISP_1, content) {
                Some("Common Lisp")
            } else if match_lines(&COOL_1, content) {
                Some("Cool")
            } else if match_lines(&OPENCL_1, content) {
                Some("OpenCL")
            } else {
                None
            }
        }
        ".cls" => {
            if match_lines(&TEX_1, content) {
                Some("TeX")
            } else if match_lines(&OBJECTSCRIPT_1, content) {
                Some("ObjectScript")
            } else if match_lines(&APEX_1, content) {
                Some("Apex")
            } else if match_lines(&VBA_1, content) {
                Some("VBA")
            } else {
                Some("OpenEdge ABL")
            }
        }
        ".cmp" => {
            if match_lines(&GERBER_IMAGE_1, content) {
                Some("Gerber Image")
            } else {
                None
            }
        }
        ".cp" => {
            if match_lines(&CPP_1, content)
                || match_lines(&CPP_2, content)
                || match_lines(&CPP_3, content)
                || match_lines(&CPP_4, content)
                || match_lines(&CPP_5, content)
                || match_lines(&CPP_6, content)
                || match_lines(&CPP_7, content)
            {
                Some("C++")
            } else {
                Some("Component Pascal")
            }
        }
        ".cs" => {
            if match_lines(&SMALLTALK_1, content) {
                Some("Smalltalk")
            } else if match_lines(&C_SHARP__1, content) {
                Some("C#")
            } else {
                None
            }
        }
        ".csl" => {
            if match_lines(&XML_2, content) {
                Some("XML")
            } else if match_lines(&KUSTO_1, content) {
                Some("Kusto")
            } else {
                None
            }
        }
        ".cue" => {
            if match_lines(&CUE_SHEET_1, content) {
                Some("Cue Sheet")
            } else {
                Some("CUE")
            }
        }
        ".d" => {
            if match_lines(&DTRACE_1, content) {
                Some("DTrace")
            } else if match_lines(&MAKEFILE_1, content) {
                Some("Makefile")
            } else {
                Some("D")
            }
        }
        ".ddl" => {
            if match_lines(&PLSQL_1, content) {
                Some("PLSQL")
            } else {
                Some("SQL")
            }
        }
        ".xml.dist" => {
            if match_lines(&XML_5, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".dll.config" => {
            if match_lines(&XML_8, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".dsp" => {
            if match_lines(&MICROSOFT_DEVELOPER_STUDIO_PROJECT_1, content) {
                Some("Microsoft Developer Studio Project")
            } else if match_lines(&FAUST_1, content) {
                Some("Faust")
            } else {
                None
            }
        }
        ".e" => {
            if match_lines(&EIFFEL_1, content) {
                Some("Eiffel")
            } else {
                Some("E")
            }
        }
        ".ecl" => {
            if match_lines(&ECLIPSE_1, content) {
                Some("ECLiPSe")
            } else if match_lines(&ECL_1, content) {
                Some("ECL")
            } else {
                None
            }
        }
        ".es" => {
            if match_lines(&ERLANG_1, content) {
                Some("Erlang")
            } else if match_lines(&JAVASCRIPT_1, content) {
                Some("JavaScript")
            } else {
                None
            }
        }
        ".f" => {
            if match_lines(&FORTH_1, content) {
                Some("Forth")
            } else if match_lines(&FILEBENCH_WML_1, content) {
                Some("Filebench WML")
            } else if match_lines(&FORTRAN_1, content) {
                Some("Fortran")
            } else {
                None
            }
        }
        ".fcgi" => {
            if match_lines(&PHP_1, content) {
                Some("PHP")
            } else if match_lines(&PYTHON_1, content) {
                Some("Python")
            } else {
                None
            }
        }
        ".for" => {
            if match_lines(&FORTH_2, content) {
                Some("Forth")
            } else if match_lines(&FORTRAN_1, content) {
                Some("Fortran")
            } else {
                Some("Formatted")
            }
        }
        ".fr" => {
            if match_lines(&FORTH_3, content) {
                Some("Forth")
            } else if match_lines(&FREGE_1, content) {
                Some("Frege")
            } else {
                Some("Text")
            }
        }
        ".frag" => {
            if match_lines(&GLSL_1, content) {
                Some("GLSL")
            } else {
                Some("JavaScript")
            }
        }
        ".fs" => {
            if match_lines(&FORTH_4, content) {
                Some("Forth")
            } else if match_lines(&F_SHARP__1, content) {
                Some("F#")
            } else if match_lines(&GLSL_1, content) {
                Some("GLSL")
            } else if match_lines(&FILTERSCRIPT_1, content) {
                Some("Filterscript")
            } else {
                None
            }
        }
        ".ftl" => {
            if match_lines(&FREE_MARKER, content) {
                Some("FreeMarker")
            } else if match_lines(&FLUENT, content) {
                Some("Fluent")
            } else {
                None
            }
        }
        ".fx" => {
            if match_lines(&FLUX_1, content) {
                Some("FLUX")
            } else {
                Some("HLSL")
            }
        }
        ".g" => {
            if match_lines(&GAP_3, content) {
                Some("GAP")
            } else {
                Some("G-code")
            }
        }
        ".gd" => {
            if match_lines(&GAP_1, content) {
                Some("GAP")
            } else if match_lines(&GDSCRIPT_1, content) {
                Some("GDScript")
            } else {
                None
            }
        }
        ".gml" => {
            if match_lines(&XML_2, content) {
                Some("XML")
            } else if match_lines(&GRAPH_MODELING_LANGUAGE_1, content) {
                Some("Graph Modeling Language")
            } else if match_lines(&GERBER_IMAGE_1, content) {
                Some("Gerber Image")
            } else {
                Some("Game Maker Language")
            }
        }
        ".gs" => {
            if match_lines(&GLSL_2, content) {
                Some("GLSL")
            } else if match_lines(&GOSU_1, content) {
                Some("Gosu")
            } else if match_lines(&GENIE_1, content)
                || match_lines(&GENIE_2, content)
                || match_lines(&GENIE_3, content)
            {
                Some("Genie")
            } else {
                Some("JavaScript")
            }
        }
        ".gst" => {
            if match_lines(&XML_5, content) {
                Some("XML")
            } else {
                Some("Gosu")
            }
        }
        ".h" => {
            if match_lines(&OBJECTIVEC_1, content) {
                Some("Objective-C")
            } else if match_lines(&CPP_1, content)
                || match_lines(&CPP_2, content)
                || match_lines(&CPP_3, content)
                || match_lines(&CPP_4, content)
                || match_lines(&CPP_5, content)
                || match_lines(&CPP_6, content)
                || match_lines(&CPP_7, content)
            {
                Some("C++")
            } else {
                Some("C")
            }
        }
        ".hh" => {
            if match_lines(&HACK_1, content) {
                Some("Hack")
            } else if match_lines(&CPP_1, content)
                || match_lines(&CPP_2, content)
                || match_lines(&CPP_3, content)
                || match_lines(&CPP_4, content)
                || match_lines(&CPP_5, content)
                || match_lines(&CPP_6, content)
                || match_lines(&CPP_7, content)
            {
                Some("C++")
            } else {
                None
            }
        }
        ".html.hl" => {
            if match_lines(&HTML_1, content) {
                Some("HTML")
            } else {
                None
            }
        }
        ".i" => {
            if match_lines(&M68K_1, content)
                || match_lines(&M68K_2, content)
                || match_lines(&M68K_3, content)
                || match_lines(&M68K_4, content)
                || match_lines(&M68K_5, content)
                || match_lines(&M68K_6, content)
                || match_lines(&M68K_7, content)
            {
                Some("Motorola 68K Assembly")
            } else if match_lines(&SWIG_1, content) {
                Some("SWIG")
            } else {
                Some("Assembly")
            }
        }
        ".ice" => {
            if match_lines(&SLICE_1, content) {
                Some("Slice")
            } else if match_lines(&JSON_1, content) {
                Some("JSON")
            } else {
                None
            }
        }
        ".inc" => {
            if match_lines(&M68K_1, content)
                || match_lines(&M68K_2, content)
                || match_lines(&M68K_3, content)
                || match_lines(&M68K_4, content)
                || match_lines(&M68K_5, content)
                || match_lines(&M68K_6, content)
                || match_lines(&M68K_7, content)
            {
                Some("Motorola 68K Assembly")
            } else if match_lines(&PHP_1, content) {
                Some("PHP")
            } else if match_lines(&SOURCEPAWN_1, content)
                || match_lines(&SOURCEPAWN_2, content)
                || match_lines(&SOURCEPAWN_3, content)
            {
                Some("SourcePawn")
            } else if match_lines(&NASL_1, content)
                || match_lines(&NASL_2, content)
                || match_lines(&NASL_3, content)
                || match_lines(&NASL_4, content)
                || match_lines(&NASL_5, content)
            {
                Some("NASL")
            } else if match_lines(&POV_RAY_SDL_1, content) {
                Some("POV-Ray SDL")
            } else if match_lines(&CPP_1, content)
                || match_lines(&CPP_2, content)
                || match_lines(&CPP_3, content)
                || match_lines(&CPP_4, content)
                || match_lines(&CPP_5, content)
                || match_lines(&CPP_6, content)
                || match_lines(&CPP_7, content)
            {
                Some("C++")
            } else if match_lines(&HTML_1, content) {
                Some("HTML")
            } else if match_lines(&PASCAL_1, content) || match_lines(&PASCAL_2, content) {
                Some("Pascal")
            } else if match_lines(&PAWN_1, content)
                || match_lines(&PAWN_2, content)
                || match_lines(&PAWN_3, content)
            {
                Some("Pawn")
            } else {
                Some("Assembly")
            }
        }
        ".j" => {
            if match_lines(&JASMIN_1, content) {
                Some("Jasmin")
            } else {
                Some("Objective-J")
            }
        }
        ".jq" => {
            if match_lines(&JSONIQ, content) {
                Some("JSONiq")
            } else {
                Some("jq")
            }
        }
        ".l" => {
            if match_lines(&COMMON_LISP_2, content) {
                Some("Common Lisp")
            } else if match_lines(&LEX_1, content) {
                Some("Lex")
            } else if match_lines(&ROFF_2, content) {
                Some("Roff")
            } else if match_lines(&PICOLISP_1, content) {
                Some("PicoLisp")
            } else {
                None
            }
        }
        ".ls" => {
            if match_lines(&LOOMSCRIPT_1, content) {
                Some("LoomScript")
            } else {
                Some("LiveScript")
            }
        }
        ".lsp" | ".lisp" => {
            if match_lines(&COMMON_LISP_3, content) {
                Some("Common Lisp")
            } else if match_lines(&NEWLISP_1, content) {
                Some("NewLisp")
            } else {
                None
            }
        }
        ".m" => {
            if match_lines(&OBJECTIVEC_1, content) || match_lines(&OBJECTIVEC_2, content) {
                Some("Objective-C")
            } else if match_lines(&MERCURY_1, content) {
                Some("Mercury")
            } else if match_lines(&MUF_1, content) {
                Some("MUF")
            } else if match_lines(&M_1, content) {
                Some("M")
            } else if match_lines(&MATHEMATICA_1, content) && match_lines(&MATHEMATICA_2, content) {
                Some("Mathematica")
            } else if match_lines(&MATLAB_1, content) || match_lines(&MATLAB_2, content) {
                Some("MATLAB")
            } else if match_lines(&LIMBO_1, content) {
                Some("Limbo")
            } else {
                Some("M")
            }
        }
        ".m4" => {
            if match_lines(&M4SUGAR_1, content) || match_lines(&M4SUGAR_2, content) {
                Some("M4Sugar")
            } else {
                Some("M4")
            }
        }
        ".mask" => {
            if match_lines(&UNITY3D_ASSET_1, content) {
                Some("Unity3D Asset")
            } else {
                Some("Mask")
            }
        }
        ".md" => {
            if match_lines(&GCC_MACHINE_DESCRIPTION_1, content) {
                Some("GCC Machine Description")
            } else {
                Some("Markdown")
            }
        }
        ".ml" => {
            if match_lines(&OCAML_1, content) {
                Some("OCaml")
            } else if match_lines(&STANDARD_ML_1, content) {
                Some("Standard ML")
            } else {
                Some("OCaml")
            }
        }
        ".mm" => {
            if match_lines(&XML_8, content) {
                Some("XML")
            } else {
                Some("Objective-C++")
            }
        }
        ".mod" => {
            if match_lines(&XML_3, content) {
                Some("XML")
            } else if match_lines(&MODULA_2_1, content) {
                Some("Modula-2")
            } else if match_lines(&LINUX_KERNEL_MODULE_1, content) {
                Some("Linux Kernel Module")
            } else {
                Some("AMPL")
            }
        }
        ".mo" => {
            if match_lines(&MOTOKO, content) {
                Some("Motoko")
            } else {
                Some("Modelica")
            }
        }
        ".moo" => {
            if match_lines(&MOOCODE_1, content) {
                Some("Moocode")
            } else if match_lines(&MERCURY_1, content) {
                Some("Mercury")
            } else {
                None
            }
        }
        ".ms" => {
            if match_lines(&ROFF_3, content) {
                Some("Roff")
            } else if match_lines(&UNIX_ASSEMBLY_1, content) {
                Some("Unix Assembly")
            } else {
                Some("MAXScript")
            }
        }
        ".n" => {
            if match_lines(&ROFF_4, content) {
                Some("Roff")
            } else if match_lines(&NEMERLE_1, content) {
                Some("Nemerle")
            } else {
                None
            }
        }
        ".ncl" => {
            if match_lines(&XML_4, content) {
                Some("XML")
            } else if match_lines(&GERBER_IMAGE_1, content) {
                Some("Gerber Image")
            } else if match_lines(&TEXT_1, content) {
                Some("Text")
            } else {
                Some("NCL")
            }
        }
        ".nb" => {
            if match_lines(&MATHEMATICA_1, content) || match_lines(&MATHEMATICA_2, content) {
                Some("Mathematica")
            } else {
                Some("Text")
            }
        }
        ".nl" => {
            if match_lines(&NL_1, content) {
                Some("NL")
            } else {
                Some("NewLisp")
            }
        }
        ".odin" => {
            if match_lines(&OBJECT_DATA_INSTANCE_NOTATION_1, content) {
                Some("Object Data Instance Notation")
            } else if match_lines(&ODIN_1, content) {
                Some("Odin")
            } else {
                None
            }
        }
        ".p" => {
            if match_lines(&GNUPLOT_1, content) || match_lines(&GNUPLOT_2, content) {
                Some("Gnuplot")
            } else {
                Some("OpenEdge ABL")
            }
        }
        ".php" => {
            if match_lines(&HACK_2, content) {
                Some("Hack")
            } else if match_lines(&PHP_2, content) {
                Some("PHP")
            } else {
                None
            }
        }
        ".pl" => {
            if match_lines(&PROLOG_1, content) {
                Some("Prolog")
            } else if match_lines(&PERL5_1, content) {
                Some("Perl")
            } else if match_lines(&PERL6_1, content) {
                Some("Raku")
            } else {
                None
            }
        }
        ".plist" => {
            if match_lines(&XML_PROPERTY_LIST_1, content) {
                Some("XML Property List")
            } else {
                Some("OpenStep Property List")
            }
        }
        ".pluginspec" => {
            if match_lines(&XML_8, content) {
                Some("XML")
            } else {
                Some("Ruby")
            }
        }
        ".pm" => {
            if match_lines(&PERL5_1, content) {
                Some("Perl")
            } else if match_lines(&PERL6_1, content) {
                Some("Raku")
            } else if match_lines(&X_PIXMAP_1, content) {
                Some("X PixMap")
            } else {
                None
            }
        }
        ".pod" => {
            if match_lines(&POD_6_1, content) {
                Some("Pod 6")
            } else {
                Some("Pod")
            }
        }
        ".pp" => {
            if match_lines(&PASCAL_1, content) || match_lines(&PASCAL_2, content) {
                Some("Pascal")
            } else {
                Some("Puppet")
            }
        }
        ".prc" => {
            if match_lines(&PLSQL_1, content) {
                Some("PLSQL")
            } else {
                Some("SQL")
            }
        }
        ".pro" => {
            if match_lines(&PROGUARD_1, content) {
                Some("Proguard")
            } else if match_lines(&PROLOG_1, content) {
                Some("Prolog")
            } else if match_lines(&INI_1, content) {
                Some("INI")
            } else if match_lines(&QMAKE_1, content) && match_lines(&QMAKE_2, content) {
                Some("QMake")
            } else if match_lines(&IDL_1, content) {
                Some("IDL")
            } else {
                None
            }
        }
        ".properties" => {
            if match_lines(&KEY_EQUALS_VALUE_1, content) && match_lines(&INI_2, content) {
                Some("INI")
            } else if match_lines(&KEY_EQUALS_VALUE_1, content)
                && match_lines(&JAVA_PROPERTIES_1, content)
            {
                Some("Java Properties")
            } else if match_lines(&KEY_EQUALS_VALUE_1, content) {
                Some("INI")
            } else if match_lines(&JAVA_PROPERTIES_1, content) {
                Some("Java Properties")
            } else {
                None
            }
        }
        ".q" => {
            if match_lines(&Q_1, content) {
                Some("q")
            } else if match_lines(&HIVEQL_1, content) {
                Some("HiveQL")
            } else {
                None
            }
        }
        ".qs" => {
            if match_lines(&Q_SHARP__1, content) {
                Some("Q#")
            } else if match_lines(&QT_SCRIPT_1, content) {
                Some("Qt Script")
            } else {
                None
            }
        }
        ".r" => {
            if match_lines(&REBOL_1, content) {
                Some("Rebol")
            } else if match_lines(&R_1, content) {
                Some("R")
            } else {
                None
            }
        }
        ".re" => {
            if match_lines(&REASON_1, content)
                || match_lines(&REASON_2, content)
                || match_lines(&REASON_3, content)
            {
                Some("Reason")
            } else if match_lines(&C_PLUS__PLUS__1, content)
                || match_lines(&C_PLUS__PLUS__2, content)
            {
                Some("C++")
            } else {
                None
            }
        }
        ".res" => {
            if match_lines(&RESCRIPT_1, content) || match_lines(&RESCRIPT_2, content) {
                Some("ReScript")
            } else if match_lines(&XML_5, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".rno" => {
            if match_lines(&RUNOFF_1, content) {
                Some("RUNOFF")
            } else if match_lines(&ROFF_5, content) {
                Some("Roff")
            } else {
                None
            }
        }
        ".rpy" => {
            if match_lines(&PYTHON_1, content) {
                Some("Python")
            } else {
                Some("Ren'Py")
            }
        }
        ".rs" => {
            if match_lines(&RUST_1, content) {
                Some("Rust")
            } else if match_lines(&RENDERSCRIPT_1, content) {
                Some("RenderScript")
            } else if match_lines(&XML_5, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".s" => {
            if match_lines(&M68K_1, content)
                || match_lines(&M68K_2, content)
                || match_lines(&M68K_3, content)
                || match_lines(&M68K_4, content)
                || match_lines(&M68K_5, content)
                || match_lines(&M68K_6, content)
                || match_lines(&M68K_7, content)
            {
                Some("Motorola 68K Assembly")
            } else {
                Some("Unix Assembly")
            }
        }
        ".sc" => {
            if match_lines(&SUPERCOLLIDER_1, content) {
                Some("SuperCollider")
            } else if match_lines(&SCALA_1, content) {
                Some("Scala")
            } else {
                None
            }
        }
        ".scd" => {
            if match_lines(&SUPERCOLLIDER_1, content) {
                Some("SuperCollider")
            } else {
                Some("Markdown")
            }
        }
        ".sch" => {
            if match_lines(&EAGLE_1, content) {
                Some("Eagle")
            } else if match_lines(&XML_5, content) {
                Some("XML")
            } else if match_lines(&SCHEME_1, content) {
                Some("Scheme")
            } else {
                Some("KiCad Schematic")
            }
        }
        ".shader" => {
            if match_lines(&GLSL_1, content) {
                Some("GLSL")
            } else {
                Some("ShaderLab")
            }
        }
        ".sls" => {
            if match_lines(&SCHEME_1, content) {
                Some("Scheme")
            } else {
                Some("SaltStack")
            }
        }
        ".spec" => {
            if match_lines(&RPM_SPEC_1, content) {
                Some("RPM Spec")
            } else if match_lines(&PYTHON_SPEC_1, content)
                && match_lines(&PYTHON_SPEC_2, content)
                && match_lines(&PYTHON_SPEC_3, content)
                && match_lines(&PYTHON_SPEC_4, content)
            {
                Some("Python")
            } else {
                Some("Ruby")
            }
        }
        ".sol" => {
            if match_lines(&SOLIDITY_1, content) {
                Some("Solidity")
            } else if match_lines(&GERBER_IMAGE_1, content) {
                Some("Gerber Image")
            } else {
                None
            }
        }
        ".sql" => {
            if match_lines(&PLPGSQL_1, content) {
                Some("PLpgSQL")
            } else if match_lines(&SQLPL_1, content) {
                Some("SQLPL")
            } else if match_lines(&PLSQL_1, content) {
                Some("PLSQL")
            } else if match_lines(&TSQL_1, content) {
                Some("TSQL")
            } else {
                Some("SQL")
            }
        }
        ".srt" => {
            if match_lines(&SUBRIP_TEXT_1, content) {
                Some("SubRip Text")
            } else {
                Some("SRecode Template")
            }
        }
        ".st" => {
            if match_lines(&STRING_TEMPLATE, content) {
                Some("StringTemplate")
            } else if match_lines(&SMALLTALK_2, content) {
                Some("Smalltalk")
            } else {
                None
            }
        }
        ".t" => {
            if match_lines(&PERL5_1, content) {
                Some("Perl")
            } else if match_lines(&RAKU_1, content) {
                Some("Raku")
            } else if match_lines(&TURING_1, content) {
                Some("Turing")
            } else {
                Some("Terra")
            }
        }
        ".toc" => {
            if match_lines(&WORLD_OF_WARCRAFT_ADDON_DATA_1, content) {
                Some("World of Warcraft Addon Data")
            } else if match_lines(&TEX_2, content) {
                Some("TeX")
            } else {
                None
            }
        }
        ".ts" => {
            if match_lines(&XML_6, content) {
                Some("XML")
            } else {
                Some("TypeScript")
            }
        }
        ".tst" => {
            if match_lines(&GAP_2, content) {
                Some("GAP")
            } else {
                Some("Scilab")
            }
        }
        ".tsx" => {
            if match_lines(&TSX_1, content) {
                Some("TSX")
            } else if match_lines(&XML_7, content) {
                Some("XML")
            } else {
                None
            }
        }
        ".txt" => {
            if match_lines(&VIM_HELP_FILE_1, content) {
                Some("Vim Help File")
            } else {
                Some("Text")
            }
        }
        ".v" => {
            if match_lines(&COQ_1, content) {
                Some("Coq")
            } else if match_lines(&VERILOG_1, content) {
                Some("Verilog")
            } else if match_lines(&V_1, content) {
                Some("V")
            } else {
                None
            }
        }
        ".vba" => {
            if match_lines(&VIM_SCRIPT_1, content) {
                Some("Vim script")
            } else {
                Some("VBA")
            }
        }
        ".vhost" => {
            if match_lines(&APACHECONF_1, content) {
                Some("ApacheConf")
            } else {
                Some("Nginx")
            }
        }
        ".w" => {
            if match_lines(&OPENEDGE_ABL_1, content) {
                Some("OpenEdge ABL")
            } else if match_lines(&CWEB_1, content) {
                Some("CWeb")
            } else {
                None
            }
        }
        ".workflow" => {
            if match_lines(&XML_5, content) {
                Some("XML")
            } else {
                Some("HCL")
            }
        }
        ".x" => {
            if match_lines(&DIRECTX_3D_FILE_1, content) {
                Some("DirectX 3D File")
            } else if match_lines(&RPC_1, content) {
                Some("RPC")
            } else if match_lines(&LOGOS_1, content) {
                Some("Logos")
            } else if match_lines(&LINKER_SCRIPT_1, content) {
                Some("Linker Script")
            } else {
                None
            }
        }
        ".yy" => {
            if match_lines(&JSON_2, content) {
                Some("JSON")
            } else {
                Some("Yacc")
            }
        }
        ".bf" => {
            if match_lines(&BRAINFUCK_1, content) {
                Some("Brainfuck")
            } else if match_lines(&BEEF_1, content) {
                Some("Beef")
            } else {
                Some("HyPhy")
            }
        }
        ".b" => {
            if match_lines(&BRAINFUCK_1, content) {
                Some("Brainfuck")
            } else {
                Some("Limbo")
            }
        }
        _ => None,
    }
}
fn match_lines(regex: &Regex, lines: &[String]) -> bool {
    for line in lines {
        if regex.is_match(line).expect("Valid RegEx") {
            return true;
        }
    }
    false
}
