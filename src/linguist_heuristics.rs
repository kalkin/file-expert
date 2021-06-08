use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref APEX_1: Regex = Regex::new(r#"^\s*(?:private|public|protected|global)\s+(?:(?:with|without) sharing\s+)?class\b"#).unwrap();
    static ref ACTIONSCRIPT_1: Regex = Regex::new(r#"^\s*(?:package(?:\s+[\w.]+)?\s+(?:{|$)|import\s+[\w.*]+\s*;|(?=.*?(?:intrinsic|extends))(intrinsic\s+)?class\s+[\w<>.]+(?:\s+extends\s+[\w<>.]+)?|(?:(?:public|protected|private|static)\s+)*(?:(?:var|const|local)\s+\w+\s*:\s*[\w<>.]+(?:\s*=.*)?\s*;|function\s+\w+\s*\((?:\s*\w+\s*:\s*[\w<>.]+\s*(,\s*\w+\s*:\s*[\w<>.]+\s*)*)?\)))"#).unwrap();
    static ref AGS_SCRIPT_1: Regex = Regex::new(r#"^(\/\/.+|((import|export)\s+)?(function|int|float|char)\s+((room|repeatedly|on|game)_)?([A-Za-z]+[A-Za-z_0-9]+)\s*[;\(])"#).unwrap();
    static ref AL_1: Regex = Regex::new(r#"\b(?i:(CODEUNIT|PAGE|PAGEEXTENSION|PAGECUSTOMIZATION|DOTNET|ENUM|ENUMEXTENSION|VALUE|QUERY|REPORT|TABLE|TABLEEXTENSION|XMLPORT|PROFILE|CONTROLADDIN))\b"#).unwrap();
    static ref ASCIIDOC_1: Regex = Regex::new(r#"^[=-]+(\s|\n)|{{[A-Za-z]"#).unwrap();
    static ref BEEF_1: Regex = Regex::new(r#"\b(class|namespace|void|static)\b"#).unwrap();
    static ref BITBAKE_1: Regex = Regex::new(r#"^\s*(# |include|require)\b"#).unwrap();
    static ref BLITZBASIC_1: Regex = Regex::new(r#"(<^\s*; |End Function)"#).unwrap();
    static ref BRAINFUCK_1: Regex = Regex::new(r#"(?:\+|>|<){4,}"#).unwrap();
    static ref COMMON_LISP_1: Regex = Regex::new(r#"^\s*\((?i:defun|in-package|defpackage) "#).unwrap();
    static ref COMMON_LISP_2: Regex = Regex::new(r#"\(def(un|macro)\s"#).unwrap();
    static ref COMMON_LISP_3: Regex = Regex::new(r#"^\s*\((?i:defun|in-package|defpackage) "#).unwrap();
    static ref COOL_1: Regex = Regex::new(r#"^class"#).unwrap();
    static ref COQ_1: Regex = Regex::new(r#"(?:^|\s)(?:Proof|Qed)\.(?:$|\s)|(?:^|\s)Require[ \t]+(Import|Export)\s"#).unwrap();
    static ref CPP_1: Regex = Regex::new(r#"^\s*#\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>"#).unwrap();
    static ref CPP_2: Regex = Regex::new(r#"^\s*template\s*<"#).unwrap();
    static ref CPP_3: Regex = Regex::new(r#"^[ \t]*(try|constexpr)"#).unwrap();
    static ref CPP_4: Regex = Regex::new(r#"^[ \t]*catch\s*\("#).unwrap();
    static ref CPP_5: Regex = Regex::new(r#"^[ \t]*(class|(using[ \t]+)?namespace)\s+\w+"#).unwrap();
    static ref CPP_6: Regex = Regex::new(r#"^[ \t]*(private|public|protected):$"#).unwrap();
    static ref CPP_7: Regex = Regex::new(r#"std::\w+"#).unwrap();
    static ref CWEB_1: Regex = Regex::new(r#"^@(<|\w+\.)"#).unwrap();
    static ref C_PLUS__PLUS__1: Regex = Regex::new(r#"^\s*#(?:(?:if|ifdef|define|pragma)\s+\w|\s*include\s+<[^>]+>)"#).unwrap();
    static ref C_PLUS__PLUS__2: Regex = Regex::new(r#"^\s*template\s*<"#).unwrap();
    static ref C_SHARP__1: Regex = Regex::new(r#"^\s*(?:namespace|using|\/\/)\s*[\w\.]+\s*"#).unwrap();
    static ref DIRECTX_3D_FILE_1: Regex = Regex::new(r#"^xof 030(2|3)(?:txt|bin|tzip|bzip)\b"#).unwrap();
    static ref DTRACE_1: Regex = Regex::new(r#"^(\w+:\w*:\w*:\w*|BEGIN|END|provider\s+|(tick|profile)-\w+\s+{[^}]*}|#pragma\s+D\s+(option|attributes|depends_on)\s|#pragma\s+ident\s)"#).unwrap();
    static ref D_1: Regex = Regex::new(r#"^module\s+[\w.]*\s*;|import\s+[\w\s,.:]*;|\w+\s+\w+\s*\(.*\)(?:\(.*\))?\s*{[^}]*}|unittest\s*(?:\(.*\))?\s*{[^}]*}"#).unwrap();
    static ref EAGLE_1: Regex = Regex::new(r#"^\s*<!DOCTYPE\s+eagle\b"#).unwrap();
    static ref ECLIPSE_1: Regex = Regex::new(r#"^[^#]+:-"#).unwrap();
    static ref ECL_1: Regex = Regex::new(r#":="#).unwrap();
    static ref EIFFEL_1: Regex = Regex::new(r#"^(note|class|feature|end|inherit)"#).unwrap();
    static ref ERLANG_1: Regex = Regex::new(r#"^\s*(?:%%|main\s*\(.*?\)\s*->)"#).unwrap();
    static ref FAUST_1: Regex = Regex::new(r#"\bprocess\s*[(=]|\b(library|import)\s*\(\s*\"|\bdeclare\s+(name|version|author|copyright|license)\s+\""#).unwrap();
    static ref FILEBENCH_WML_1: Regex = Regex::new(r#"flowop"#).unwrap();
    static ref FILTERSCRIPT_1: Regex = Regex::new(r#"#include|#pragma\s+(rs|version)|__attribute__"#).unwrap();
    static ref FLUX_1: Regex = Regex::new(r#"^\s*(typedef|atomic)\b"#).unwrap();
    static ref FORTH_1: Regex = Regex::new(r#"^: "#).unwrap();
    static ref FORTH_2: Regex = Regex::new(r#"^: "#).unwrap();
    static ref FORTH_3: Regex = Regex::new(r#"^(: |also |new-device|previous )"#).unwrap();
    static ref FORTH_4: Regex = Regex::new(r#"^(: |new-device)"#).unwrap();
    static ref FORTRAN_1: Regex = Regex::new(r#"^(?i:[c*][^abd-z]|      (subroutine|program|end|data)\s|\s*!)"#).unwrap();
    static ref FREGE_1: Regex = Regex::new(r#"^\s*(import|module|package|data|type) "#).unwrap();
    static ref F_SHARP__1: Regex = Regex::new(r#"^\s*(#light|import|let|module|namespace|open|type)"#).unwrap();
    static ref GAP_1: Regex = Regex::new(r#"\s*(Declare|BindGlobal|KeyDependentOperation)"#).unwrap();
    static ref GAP_2: Regex = Regex::new(r#"gap> "#).unwrap();
    static ref GAP_3: Regex = Regex::new(r#"\s+:=\s+"#).unwrap();
    static ref GCC_MACHINE_DESCRIPTION_1: Regex = Regex::new(r#"^(;;|\(define_)"#).unwrap();
    static ref GDSCRIPT_1: Regex = Regex::new(r#"\s*(extends|var|const|enum|func|class|signal|tool|yield|assert|onready)"#).unwrap();
    static ref GENIE_1: Regex = Regex::new(r#"^\[indent=[0-9]+\]"#).unwrap();
    static ref GENIE_2: Regex = Regex::new(r#"^\t*(def|class|construct|init)\b"#).unwrap();
    static ref GENIE_3: Regex = Regex::new(r#"^(init|import)\b"#).unwrap();
    static ref GERBER_IMAGE_1: Regex = Regex::new(r#"^[DGMT][0-9]{2}\*\r?\n"#).unwrap();
    static ref GERBER_IMAGE_2: Regex = Regex::new(r#"^[DGMT][0-9]{2}\*$"#).unwrap();
    static ref GERBER_IMAGE_3: Regex = Regex::new(r#"^[DGMT][0-9]{2}\*\r?\n"#).unwrap();
    static ref GERBER_IMAGE_4: Regex = Regex::new(r#"^[DGMT][0-9]{2}\*\r?\n"#).unwrap();
    static ref GLSL_1: Regex = Regex::new(r#"^\s*(#version|precision|uniform|varying|vec[234])"#).unwrap();
    static ref GLSL_2: Regex = Regex::new(r#"^#version\s+[0-9]+\b"#).unwrap();
    static ref GNUPLOT_1: Regex = Regex::new(r#"^s?plot\b"#).unwrap();
    static ref GNUPLOT_2: Regex = Regex::new(r#"^set\s+(term|terminal|out|output|[xy]tics|[xy]label|[xy]range|style)\b"#).unwrap();
    static ref GOSU_1: Regex = Regex::new(r#"^uses (java|gw)\."#).unwrap();
    static ref GRAPH_MODELING_LANGUAGE_1: Regex = Regex::new(r#"(?i:^\s*(graph|node)\s+\[$)"#).unwrap();
    static ref HACK_1: Regex = Regex::new(r#"<\?hh"#).unwrap();
    static ref HACK_2: Regex = Regex::new(r#"<\?hh"#).unwrap();
    static ref HIVEQL_1: Regex = Regex::new(r#"(?i:SELECT\s+[\w*,]+\s+FROM|(CREATE|ALTER|DROP)\s(DATABASE|SCHEMA|TABLE))"#).unwrap();
    static ref HTML_1: Regex = Regex::new(r#"^\s*</?(?i:html|head|title|body|span|div)\b"#).unwrap();
    static ref IDL_1: Regex = Regex::new(r#"^\s*function[ \w,]+$"#).unwrap();
    static ref INI_1: Regex = Regex::new(r#"last_client="#).unwrap();
    static ref INI_2: Regex = Regex::new(r#"^[;\[]"#).unwrap();
    static ref JASMIN_1: Regex = Regex::new(r#"^\.\w+\b"#).unwrap();
    static ref JAVASCRIPT_1: Regex = Regex::new(r#"(?m:\/\/|(\"|')use strict\1|export\s+default\s|\/\*.*?\*\/)"#).unwrap();
    static ref JAVA_PROPERTIES_1: Regex = Regex::new(r#"^[^#!][^:]*:"#).unwrap();
    static ref JSON_1: Regex = Regex::new(r#"\A\s*[{\[]"#).unwrap();
    static ref JSON_2: Regex = Regex::new(r#"\\"modelName\\"\:\s*\\"GM"#).unwrap();
    static ref KEY_EQUALS_VALUE_1: Regex = Regex::new(r#"^[^#!;][^=]*="#).unwrap();
    static ref LEX_1: Regex = Regex::new(r#"^(%[%{}]xs|<.*>)"#).unwrap();
    static ref LIMBO_1: Regex = Regex::new(r#"^\w+\s*:\s*module\s*{"#).unwrap();
    static ref LINKER_SCRIPT_1: Regex = Regex::new(r#"OUTPUT_ARCH\(|OUTPUT_FORMAT\(|SECTIONS"#).unwrap();
    static ref LOGOS_1: Regex = Regex::new(r#"^%(end|ctor|hook|group)\b"#).unwrap();
    static ref LOOMSCRIPT_1: Regex = Regex::new(r#"^\s*package\s*[\w\.\/\*\s]*\s*{"#).unwrap();
    static ref LTSPICE_SYMBOL_1: Regex = Regex::new(r#"^SymbolType[ \t]"#).unwrap();
    static ref M4SUGAR_1: Regex = Regex::new(r#"AC_DEFUN|AC_PREREQ|AC_INIT"#).unwrap();
    static ref M4SUGAR_2: Regex = Regex::new(r#"^_?m4_"#).unwrap();
    static ref M68K_1: Regex = Regex::new(r#"(?im)\bmoveq(?:\.l)?\s+#(?:\$-?[0-9a-f]{1,3}|%[0-1]{1,8}|-?[0-9]{1,3}),\s*d[0-7]\b"#).unwrap();
    static ref M68K_2: Regex = Regex::new(r#"(?im)^\s*move(?:\.[bwl])?\s+(?:sr|usp),\s*[^\s]+"#).unwrap();
    static ref M68K_3: Regex = Regex::new(r#"(?im)^\s*move\.[bwl]\s+.*\b[ad]\d"#).unwrap();
    static ref M68K_4: Regex = Regex::new(r#"(?im)^\s*movem\.[bwl]\b"#).unwrap();
    static ref M68K_5: Regex = Regex::new(r#"(?im)^\s*move[mp](?:\.[wl])?\b"#).unwrap();
    static ref M68K_6: Regex = Regex::new(r#"(?im)^\s*btst\b"#).unwrap();
    static ref M68K_7: Regex = Regex::new(r#"(?im)^\s*dbra\b"#).unwrap();
    static ref MAKEFILE_1: Regex = Regex::new(r#"([\/\\].*:\s+.*\s\\$|: \\$|^[ %]:|^[\w\s\/\\.]+\w+\.\w+\s*:\s+[\w\s\/\\.]+\w+\.\w+)"#).unwrap();
    static ref MAN_HEADING_1: Regex = Regex::new(r#"^[.'][ \t]*SH +(?:[^\"\s]+|\"[^\"\s]+)"#).unwrap();
    static ref MAN_TITLE_1: Regex = Regex::new(r#"^[.'][ \t]*TH +(?:[^\"\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\s@]+@)"#).unwrap();
    static ref MARKDOWN_1: Regex = Regex::new(r#"(^[-A-Za-z0-9=#!\*\[|>])|<\/"#).unwrap();
    static ref MARKDOWN_2: Regex = Regex::new(r#"\A\z"#).unwrap();
    static ref MATHEMATICA_1: Regex = Regex::new(r#"\(\*"#).unwrap();
    static ref MATHEMATICA_2: Regex = Regex::new(r#"\*\)$"#).unwrap();
    static ref MATLAB_1: Regex = Regex::new(r#"^\s*%"#).unwrap();
    static ref MDOC_DATE_1: Regex = Regex::new(r#"^[.'][ \t]*Dd +(?:[^\"\s]+|\"[^\"]+\")"#).unwrap();
    static ref MDOC_HEADING_1: Regex = Regex::new(r#"^[.'][ \t]*Sh +(?:[^\"\s]|\"[^\"]+\")"#).unwrap();
    static ref MDOC_TITLE_1: Regex = Regex::new(r#"^[.'][ \t]*Dt +(?:[^\"\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\s@]+@)"#).unwrap();
    static ref MERCURY_1: Regex = Regex::new(r#":-\s+\w+"#).unwrap();
    static ref MICROSOFT_DEVELOPER_STUDIO_PROJECT_1: Regex = Regex::new(r#"# Microsoft Developer Studio Generated Build File"#).unwrap();
    static ref MODULA_2_1: Regex = Regex::new(r#"^\s*(?i:MODULE|END) [\w\.]+;"#).unwrap();
    static ref MOOCODE_1: Regex = Regex::new(r#"^\s*@\w+\s+"#).unwrap();
    static ref MUF_1: Regex = Regex::new(r#"^: "#).unwrap();
    static ref M_1: Regex = Regex::new(r#"^\s*;"#).unwrap();
    static ref NASL_1: Regex = Regex::new(r#"^\s*include\s*\(\s*(?:\"|')[\\/\w\-\.:\s]+\.(?:nasl|inc)\s*(?:\"|')\s*\)\s*;"#).unwrap();
    static ref NASL_2: Regex = Regex::new(r#"^\s*(?:global|local)_var\s+(?:\w+(?:\s*=\s*[\w\-\"']+)?\s*)(?:,\s*\w+(?:\s*=\s*[\w\-\"']+)?\s*)*+\s*;"#).unwrap();
    static ref NASL_3: Regex = Regex::new(r#"^\s*namespace\s+\w+\s*{"#).unwrap();
    static ref NASL_4: Regex = Regex::new(r#"^\s*object\s+\w+\s*(?:extends\s+\w+(?:::\w+)?)?\s*{"#).unwrap();
    static ref NASL_5: Regex = Regex::new(r#"^\s*(?:public\s+|private\s+|\s*)function\s+\w+\s*\([\w\s,]*\)\s*{"#).unwrap();
    static ref NEMERLE_1: Regex = Regex::new(r#"^(module|namespace|using)\s"#).unwrap();
    static ref NEWLISP_1: Regex = Regex::new(r#"^\s*\(define "#).unwrap();
    static ref NL_1: Regex = Regex::new(r#"^(b|g)[0-9]+ "#).unwrap();
    static ref OBJECTIVEC_1: Regex = Regex::new(r#"^\s*(@(interface|class|protocol|property|end|synchronised|selector|implementation)\b|#import\s+.+\.h[\">])"#).unwrap();
    static ref OBJECTSCRIPT_1: Regex = Regex::new(r#"^Class\s"#).unwrap();
    static ref OBJECT_DATA_INSTANCE_NOTATION_1: Regex = Regex::new(r#"(?:^|<)\s*[A-Za-z0-9_]+\s*=\s*<"#).unwrap();
    static ref OCAML_1: Regex = Regex::new(r#"(^\s*module)|let rec |match\s+(\S+\s)+with"#).unwrap();
    static ref ODIN_1: Regex = Regex::new(r#"package\s+\w+|\b(?:im|ex)port\s*\"[\w:./]+\"|\w+\s*::\s*(?:proc|struct)\s*\(|^\s*//\s"#).unwrap();
    static ref OPENCL_1: Regex = Regex::new(r#"\/\* |\/\/ |^\}"#).unwrap();
    static ref OPENEDGE_ABL_1: Regex = Regex::new(r#"&ANALYZE-SUSPEND _UIB-CODE-BLOCK _CUSTOM _DEFINITIONS"#).unwrap();
    static ref PASCAL_1: Regex = Regex::new(r#"^\s*end[.;]"#).unwrap();
    static ref PERL5_1: Regex = Regex::new(r#"\buse\s+(?:strict\b|v?5\.)"#).unwrap();
    static ref PERL6_1: Regex = Regex::new(r#"^\s*(?:use\s+v6\b|\bmodule\b|\b(?:my\s+)?class\b)"#).unwrap();
    static ref PHP_1: Regex = Regex::new(r#"^<\?(?:php)?"#).unwrap();
    static ref PHP_2: Regex = Regex::new(r#"<\?[^h]"#).unwrap();
    static ref PICOLISP_1: Regex = Regex::new(r#"^\((de|class|rel|code|data|must)\s"#).unwrap();
    static ref PLPGSQL_1: Regex = Regex::new(r#"(?i:^\\i\b|AS\s+\$\$|LANGUAGE\s+'?plpgsql'?|BEGIN(\s+WORK)?\s*;)"#).unwrap();
    static ref PLSQL_1: Regex = Regex::new(r#"(?i:\$\$PLSQL_|XMLTYPE|systimestamp|\.nextval|CONNECT\s+BY|AUTHID\s+(DEFINER|CURRENT_USER)|constructor\W+function)"#).unwrap();
    static ref POD_6_1: Regex = Regex::new(r#"^[\s&&[^\n]]*=(comment|begin pod|begin para|item\d+)"#).unwrap();
    static ref POV_RAY_SDL_1: Regex = Regex::new(r#"^\s*#(declare|local|macro|while)\s"#).unwrap();
    static ref PROGUARD_1: Regex = Regex::new(r#"^-(include\b.*\.pro$|keep\b|keepclassmembers\b|keepattributes\b)"#).unwrap();
    static ref PROLOG_1: Regex = Regex::new(r#"^[^#]*:-"#).unwrap();
    static ref PUBLIC_KEY_1: Regex = Regex::new(r#"^(----[- ]BEGIN|ssh-(rsa|dss)) "#).unwrap();
    static ref PUPPET_1: Regex = Regex::new(r#"^\s+\w+\s+=>\s"#).unwrap();
    static ref PYTHON_1: Regex = Regex::new(r#"(?m:^(import|from|class|def)\s)"#).unwrap();
    static ref QMAKE_1: Regex = Regex::new(r#"HEADERS"#).unwrap();
    static ref QMAKE_2: Regex = Regex::new(r#"SOURCES"#).unwrap();
    static ref QT_SCRIPT_1: Regex = Regex::new(r#"(\w+\.prototype\.\w+|===|\bvar\b)"#).unwrap();
    static ref Q_1: Regex = Regex::new(r#"((?i:[A-Z.][\w.]*:{)|(^|\n)\\(cd?|d|l|p|ts?) )"#).unwrap();
    static ref Q_SHARP__1: Regex = Regex::new(r#"^((\/{2,3})?\s*(namespace|operation)\b)"#).unwrap();
    static ref RAKU_1: Regex = Regex::new(r#"^\s*(?:use\s+v6\b|\bmodule\b|\bmy\s+class\b)"#).unwrap();
    static ref REASON_1: Regex = Regex::new(r#"^\s*module\s+type\s"#).unwrap();
    static ref REASON_2: Regex = Regex::new(r#"^\s*(?:include|open)\s+\w+\s*;\s*$"#).unwrap();
    static ref REASON_3: Regex = Regex::new(r#"^\s*let\s+(?:module\s\w+\s*=\s*{|\w+:\s+.*=.*;\s*$)"#).unwrap();
    static ref REBOL_1: Regex = Regex::new(r#"(?i:\bRebol\b)"#).unwrap();
    static ref RENDERSCRIPT_1: Regex = Regex::new(r#"#include|#pragma\s+(rs|version)|__attribute__"#).unwrap();
    static ref RESCRIPT_1: Regex = Regex::new(r#"^\s*(let|module|type)\s+\w*\s+=\s+"#).unwrap();
    static ref RESCRIPT_2: Regex = Regex::new(r#"^\s*(?:include|open)\s+\w+\s*$"#).unwrap();
    static ref ROFF_1: Regex = Regex::new(r#"^\.(?:[A-Za-z]{2}(?:\s|$)|\\\")"#).unwrap();
    static ref ROFF_2: Regex = Regex::new(r#"^\.[A-Za-z]{2}(\s|$)"#).unwrap();
    static ref ROFF_3: Regex = Regex::new(r#"^[.'][A-Za-z]{2}(\s|$)"#).unwrap();
    static ref ROFF_4: Regex = Regex::new(r#"^[.']"#).unwrap();
    static ref ROFF_5: Regex = Regex::new(r#"^\.\\\" "#).unwrap();
    static ref RPC_1: Regex = Regex::new(r#"\b(program|version)\s+\w+\s*{|\bunion\s+\w+\s+switch\s*\("#).unwrap();
    static ref RUNOFF_1: Regex = Regex::new(r#"(?i:^\.!|^\f|\f$|^\.end lit(?:eral)?\b|^\.[a-zA-Z].*?;\.[a-zA-Z](?:[; \t])|\^\*[^\s*][^*]*\\\*(?=$|\s)|^\.c;[ \t]*\w+)"#).unwrap();
    static ref RUST_1: Regex = Regex::new(r#"^(use |fn |mod |pub |macro_rules|impl|#!?\[)"#).unwrap();
    static ref R_1: Regex = Regex::new(r#"<-|^\s*#"#).unwrap();
    static ref SCALA_1: Regex = Regex::new(r#"(^\s*import (scala|java)\.|^\s*class\b)"#).unwrap();
    static ref SCHEME_1: Regex = Regex::new(r#"^\s*\((?:define|let)"#).unwrap();
    static ref SMALLTALK_1: Regex = Regex::new(r#"![\w\s]+methodsFor: "#).unwrap();
    static ref SOLIDITY_1: Regex = Regex::new(r#"\bpragma\s+solidity\b|\b(?:abstract\s+)?contract\s+(?!\d)[a-zA-Z0-9$_]+(?:\s+is\s+(?:[a-zA-Z0-9$_][^\{]*?)?)?\s*\{"#).unwrap();
    static ref SOURCEPAWN_1: Regex = Regex::new(r#"^public\s+(?:SharedPlugin(?:\s+|:)__pl_\w+\s*=(?:\s*{)?|(?:void\s+)?__pl_\w+_SetNTVOptional\(\)(?:\s*{)?)"#).unwrap();
    static ref SQLPL_1: Regex = Regex::new(r#"(?i:ALTER\s+MODULE|MODE\s+DB2SQL|\bSYS(CAT|PROC)\.|ASSOCIATE\s+RESULT\s+SET|\bEND!\s*$)"#).unwrap();
    static ref STANDARD_ML_1: Regex = Regex::new(r#"=> |case\s+(\S+\s)+of"#).unwrap();
    static ref SUBRIP_TEXT_1: Regex = Regex::new(r#"^(\d{2}:\d{2}:\d{2},\d{3})\s*(-->)\s*(\d{2}:\d{2}:\d{2},\d{3})$"#).unwrap();
    static ref SUPERCOLLIDER_1: Regex = Regex::new(r#"(?i:\^(this|super)\.|^\s*~\w+\s*=\.)"#).unwrap();
    static ref SWIG_1: Regex = Regex::new(r#"^[ \t]*%[a-z_]+\b|^%[{}]$"#).unwrap();
    static ref TEXT_1: Regex = Regex::new(r#"THE_TITLE"#).unwrap();
    static ref TEX_1: Regex = Regex::new(r#"\\\w+{"#).unwrap();
    static ref TEX_2: Regex = Regex::new(r#"^\\(contentsline|defcounter|beamer|boolfalse)"#).unwrap();
    static ref TSQL_1: Regex = Regex::new(r#"(?i:^\s*GO\b|BEGIN(\s+TRY|\s+CATCH)|OUTPUT\s+INSERTED|DECLARE\s+@|\[dbo\])"#).unwrap();
    static ref TSX_1: Regex = Regex::new(r#"^\s*(import.+(from\s+|require\()['\"]react|\/\/\/\s*<reference\s)"#).unwrap();
    static ref TURING_1: Regex = Regex::new(r#"^\s*%[ \t]+|^\s*var\s+\w+(\s*:\s*\w+)?\s*:=\s*\w+"#).unwrap();
    static ref UNITY3D_ASSET_1: Regex = Regex::new(r#"tag:unity3d.com"#).unwrap();
    static ref UNIX_ASSEMBLY_1: Regex = Regex::new(r#"^\s*\.(?:include\s|globa?l\s|[A-Za-z][_A-Za-z0-9]*:)"#).unwrap();
    static ref VERILOG_1: Regex = Regex::new(r#"^[ \t]*module\s+[^\s()]+\s+\#?\(|^[ \t]*`(?:define|ifdef|ifndef|include|timescale)|^[ \t]*always[ \t]+@|^[ \t]*initial[ \t]+(begin|@)"#).unwrap();
    static ref VIM_HELP_FILE_1: Regex = Regex::new(r#"(?:(?:[ \t]|^)vi(?:m[<=>]?\d+|m)?|[ \t]ex)(?=:(?=[ \t]*set?[ \t][^\n:]+:)|:(?![ \t]* set?[ \t]))(?:(?:[ \t]|[ \t]*:[ \t]*)\w*(?:[ \t]*=(?:[^\\[ \t]]|\\.)*)?)*[[ \t]:](?:filetype|ft|syntax)[ \t]*=help(?=[ \t]|:|$)"#).unwrap();
    static ref VIM_SCRIPT_1: Regex = Regex::new(r#"^UseVimball"#).unwrap();
    static ref V_1: Regex = Regex::new(r#"\$(?:if|else)[ \t]|^[ \t]*fn\s+[^\s()]+\(.*?\).*?\{|^[ \t]*for\s*\{"#).unwrap();
    static ref WORLD_OF_WARCRAFT_ADDON_DATA_1: Regex = Regex::new(r#"^## |@no-lib-strip@"#).unwrap();
    static ref XBASE_1: Regex = Regex::new(r#"^\s*#\s*(?i:if|ifdef|ifndef|define|command|xcommand|translate|xtranslate|include|pragma|undef)\b"#).unwrap();
    static ref XML_1: Regex = Regex::new(r#"^(\s*)(?i:<Project|<Import|<Property|<?xml|xmlns)"#).unwrap();
    static ref XML_2: Regex = Regex::new(r#"(?i:^\s*(\<\?xml|xmlns))"#).unwrap();
    static ref XML_3: Regex = Regex::new(r#"<!ENTITY "#).unwrap();
    static ref XML_4: Regex = Regex::new(r#"^\s*<\?xml\s+version"#).unwrap();
    static ref XML_5: Regex = Regex::new(r#"^\s*<\?xml"#).unwrap();
    static ref XML_6: Regex = Regex::new(r#"<TS\b"#).unwrap();
    static ref XML_7: Regex = Regex::new(r#"(?i:^\s*<\?xml\s+version)"#).unwrap();
    static ref XML_8: Regex = Regex::new(r#"^\s*<\w+\b"#).unwrap();
    static ref XML_PROPERTY_LIST_1: Regex = Regex::new(r#"<!DOCTYPE\s+plist"#).unwrap();
    static ref X_PIXMAP_1: Regex = Regex::new(r#"^\s*\/\* XPM \*\/"#).unwrap();
}
pub fn linguist_heuristic(ext: &str, content: &str) -> Option<&'static str> {
    match ext {
        ".1" | ".2" | ".3" | ".4" | ".5" | ".6" | ".7" | ".8" | ".9" => {
            if match_lines(&MDOC_DATE_1, &content)
                && match_lines(&MDOC_TITLE_1, &content)
                && match_lines(&MDOC_HEADING_1, &content)
            {
                Some("Roff Manpage")
            } else if match_lines(&MAN_TITLE_1, &content) && match_lines(&MAN_HEADING_1, &content) {
                Some("Roff Manpage")
            } else if match_lines(&ROFF_1, &content) {
                Some("Roff")
            } else {
                None
            }
        }
        ".1in" | ".1m" | ".1x" | ".3in" | ".3m" | ".3p" | ".3pm" | ".3qt" | ".3x" | ".man"
        | ".mdoc" => {
            if match_lines(&MDOC_DATE_1, &content)
                && match_lines(&MDOC_TITLE_1, &content)
                && match_lines(&MDOC_HEADING_1, &content)
            {
                Some("Roff Manpage")
            } else if match_lines(&MAN_TITLE_1, &content) && match_lines(&MAN_HEADING_1, &content) {
                Some("Roff Manpage")
            } else {
                Some("Roff")
            }
        }
        ".al" => {
            if match_lines(&AL_1, &content) {
                Some("AL")
            } else {
                Some("Perl")
            }
        }
        ".as" => {
            if match_lines(&ACTIONSCRIPT_1, &content) {
                Some("ActionScript")
            } else {
                Some("AngelScript")
            }
        }
        ".asc" => {
            if match_lines(&PUBLIC_KEY_1, &content) {
                Some("Public Key")
            } else if match_lines(&ASCIIDOC_1, &content) {
                Some("AsciiDoc")
            } else if match_lines(&AGS_SCRIPT_1, &content) {
                Some("AGS Script")
            } else {
                None
            }
        }
        ".asm" => {
            if match_lines(&M68K_1, &content)
                || match_lines(&M68K_2, &content)
                || match_lines(&M68K_3, &content)
                || match_lines(&M68K_4, &content)
                || match_lines(&M68K_5, &content)
                || match_lines(&M68K_6, &content)
                || match_lines(&M68K_7, &content)
            {
                Some("Motorola 68K Assembly")
            } else {
                None
            }
        }
        ".asy" => {
            if match_lines(&LTSPICE_SYMBOL_1, &content) {
                Some("LTspice Symbol")
            } else {
                Some("Asymptote")
            }
        }
        ".bb" => {
            if match_lines(&BLITZBASIC_1, &content) {
                Some("BlitzBasic")
            } else if match_lines(&BITBAKE_1, &content) {
                Some("BitBake")
            } else {
                None
            }
        }
        ".brd" => {
            if match_lines(&EAGLE_1, &content) {
                Some("Eagle")
            } else {
                Some("KiCad Legacy Layout")
            }
        }
        ".builds" => {
            if match_lines(&XML_1, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".cake" => {
            if match_lines(&C_SHARP__1, &content) {
                Some("C#")
            } else {
                None
            }
        }
        ".ch" => {
            if match_lines(&XBASE_1, &content) {
                Some("xBase")
            } else {
                None
            }
        }
        ".cl" => {
            if match_lines(&COMMON_LISP_1, &content) {
                Some("Common Lisp")
            } else if match_lines(&COOL_1, &content) {
                Some("Cool")
            } else if match_lines(&OPENCL_1, &content) {
                Some("OpenCL")
            } else {
                None
            }
        }
        ".cls" => {
            if match_lines(&TEX_1, &content) {
                Some("TeX")
            } else if match_lines(&OBJECTSCRIPT_1, &content) {
                Some("ObjectScript")
            } else if match_lines(&APEX_1, &content) {
                Some("Apex")
            } else {
                None
            }
        }
        ".cmp" => {
            if match_lines(&GERBER_IMAGE_1, &content) {
                Some("Gerber Image")
            } else {
                None
            }
        }
        ".config" => {
            if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".cp" => {
            if match_lines(&CPP_1, &content)
                || match_lines(&CPP_2, &content)
                || match_lines(&CPP_3, &content)
                || match_lines(&CPP_4, &content)
                || match_lines(&CPP_5, &content)
                || match_lines(&CPP_6, &content)
                || match_lines(&CPP_7, &content)
            {
                Some("C++")
            } else {
                None
            }
        }
        ".cs" => {
            if match_lines(&SMALLTALK_1, &content) {
                Some("Smalltalk")
            } else if match_lines(&C_SHARP__1, &content) {
                Some("C#")
            } else {
                None
            }
        }
        ".d" => {
            if match_lines(&DTRACE_1, &content) {
                Some("DTrace")
            } else if match_lines(&MAKEFILE_1, &content) {
                Some("Makefile")
            } else {
                Some("D")
            }
        }
        ".dist" => {
            if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".dll.config" => {
            if match_lines(&XML_8, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".dsp" => {
            if match_lines(&MICROSOFT_DEVELOPER_STUDIO_PROJECT_1, &content) {
                Some("Microsoft Developer Studio Project")
            } else if match_lines(&FAUST_1, &content) {
                Some("Faust")
            } else {
                None
            }
        }
        ".e" => {
            if match_lines(&EIFFEL_1, &content) {
                Some("Eiffel")
            } else {
                Some("E")
            }
        }
        ".ecl" => {
            if match_lines(&ECLIPSE_1, &content) {
                Some("ECLiPSe")
            } else if match_lines(&ECL_1, &content) {
                Some("ECL")
            } else {
                None
            }
        }
        ".es" => {
            if match_lines(&ERLANG_1, &content) {
                Some("Erlang")
            } else if match_lines(&JAVASCRIPT_1, &content) {
                Some("JavaScript")
            } else {
                None
            }
        }
        ".f" => {
            if match_lines(&FORTH_1, &content) {
                Some("Forth")
            } else if match_lines(&FILEBENCH_WML_1, &content) {
                Some("Filebench WML")
            } else if match_lines(&FORTRAN_1, &content) {
                Some("Fortran")
            } else {
                None
            }
        }
        ".for" => {
            if match_lines(&FORTH_2, &content) {
                Some("Forth")
            } else if match_lines(&FORTRAN_1, &content) {
                Some("Fortran")
            } else {
                None
            }
        }
        ".fr" => {
            if match_lines(&FORTH_3, &content) {
                Some("Forth")
            } else if match_lines(&FREGE_1, &content) {
                Some("Frege")
            } else {
                Some("Text")
            }
        }
        ".fs" => {
            if match_lines(&FORTH_4, &content) {
                Some("Forth")
            } else if match_lines(&F_SHARP__1, &content) {
                Some("F#")
            } else if match_lines(&GLSL_1, &content) {
                Some("GLSL")
            } else if match_lines(&FILTERSCRIPT_1, &content) {
                Some("Filterscript")
            } else {
                None
            }
        }
        ".fx" => {
            if match_lines(&FLUX_1, &content) {
                Some("FLUX")
            } else {
                Some("HLSL")
            }
        }
        ".g" => {
            if match_lines(&GAP_3, &content) {
                Some("GAP")
            } else {
                Some("G-code")
            }
        }
        ".gd" => {
            if match_lines(&GAP_1, &content) {
                Some("GAP")
            } else if match_lines(&GDSCRIPT_1, &content) {
                Some("GDScript")
            } else {
                None
            }
        }
        ".gml" => {
            if match_lines(&XML_2, &content) {
                Some("XML")
            } else if match_lines(&GRAPH_MODELING_LANGUAGE_1, &content) {
                Some("Graph Modeling Language")
            } else if match_lines(&GERBER_IMAGE_2, &content) {
                Some("Gerber Image")
            } else {
                Some("Game Maker Language")
            }
        }
        ".gs" => {
            if match_lines(&GLSL_2, &content) {
                Some("GLSL")
            } else if match_lines(&GOSU_1, &content) {
                Some("Gosu")
            } else if match_lines(&GENIE_1, &content)
                || match_lines(&GENIE_2, &content)
                || match_lines(&GENIE_3, &content)
            {
                Some("Genie")
            } else {
                None
            }
        }
        ".gst" => {
            if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                Some("Gosu")
            }
        }
        ".h" => {
            if match_lines(&OBJECTIVEC_1, &content) {
                Some("Objective-C")
            } else if match_lines(&CPP_1, &content)
                || match_lines(&CPP_2, &content)
                || match_lines(&CPP_3, &content)
                || match_lines(&CPP_4, &content)
                || match_lines(&CPP_5, &content)
                || match_lines(&CPP_6, &content)
                || match_lines(&CPP_7, &content)
            {
                Some("C++")
            } else {
                Some("C")
            }
        }
        ".hh" => {
            if match_lines(&HACK_1, &content) {
                Some("Hack")
            } else if match_lines(&CPP_1, &content)
                || match_lines(&CPP_2, &content)
                || match_lines(&CPP_3, &content)
                || match_lines(&CPP_4, &content)
                || match_lines(&CPP_5, &content)
                || match_lines(&CPP_6, &content)
                || match_lines(&CPP_7, &content)
            {
                Some("C++")
            } else {
                None
            }
        }
        ".html.hl" => {
            if match_lines(&HTML_1, &content) {
                Some("HTML")
            } else {
                None
            }
        }
        ".i" => {
            if match_lines(&M68K_1, &content)
                || match_lines(&M68K_2, &content)
                || match_lines(&M68K_3, &content)
                || match_lines(&M68K_4, &content)
                || match_lines(&M68K_5, &content)
                || match_lines(&M68K_6, &content)
                || match_lines(&M68K_7, &content)
            {
                Some("Motorola 68K Assembly")
            } else if match_lines(&SWIG_1, &content) {
                Some("SWIG")
            } else {
                None
            }
        }
        ".ice" => {
            if match_lines(&JSON_1, &content) {
                Some("JSON")
            } else {
                Some("Slice")
            }
        }
        ".inc" => {
            if match_lines(&M68K_1, &content)
                || match_lines(&M68K_2, &content)
                || match_lines(&M68K_3, &content)
                || match_lines(&M68K_4, &content)
                || match_lines(&M68K_5, &content)
                || match_lines(&M68K_6, &content)
                || match_lines(&M68K_7, &content)
            {
                Some("Motorola 68K Assembly")
            } else if match_lines(&PHP_1, &content) {
                Some("PHP")
            } else if match_lines(&SOURCEPAWN_1, &content) {
                Some("SourcePawn")
            } else if match_lines(&NASL_1, &content)
                || match_lines(&NASL_2, &content)
                || match_lines(&NASL_3, &content)
                || match_lines(&NASL_4, &content)
                || match_lines(&NASL_5, &content)
            {
                Some("NASL")
            } else if match_lines(&POV_RAY_SDL_1, &content) {
                Some("POV-Ray SDL")
            } else if match_lines(&CPP_1, &content)
                || match_lines(&CPP_2, &content)
                || match_lines(&CPP_3, &content)
                || match_lines(&CPP_4, &content)
                || match_lines(&CPP_5, &content)
                || match_lines(&CPP_6, &content)
                || match_lines(&CPP_7, &content)
            {
                Some("C++")
            } else if match_lines(&HTML_1, &content) {
                Some("HTML")
            } else {
                Some("C++")
            }
        }
        ".j" => {
            if match_lines(&JASMIN_1, &content) {
                Some("Jasmin")
            } else {
                Some("Objective-J")
            }
        }
        ".l" => {
            if match_lines(&COMMON_LISP_2, &content) {
                Some("Common Lisp")
            } else if match_lines(&LEX_1, &content) {
                Some("Lex")
            } else if match_lines(&ROFF_2, &content) {
                Some("Roff")
            } else if match_lines(&PICOLISP_1, &content) {
                Some("PicoLisp")
            } else {
                None
            }
        }
        ".ls" => {
            if match_lines(&LOOMSCRIPT_1, &content) {
                Some("LoomScript")
            } else {
                Some("LiveScript")
            }
        }
        ".lsp" | ".lisp" => {
            if match_lines(&COMMON_LISP_3, &content) {
                Some("Common Lisp")
            } else if match_lines(&NEWLISP_1, &content) {
                Some("NewLisp")
            } else {
                None
            }
        }
        ".m" => {
            if match_lines(&OBJECTIVEC_1, &content) {
                Some("Objective-C")
            } else if match_lines(&MERCURY_1, &content) {
                Some("Mercury")
            } else if match_lines(&MUF_1, &content) {
                Some("MUF")
            } else if match_lines(&M_1, &content) {
                Some("M")
            } else if match_lines(&MATHEMATICA_1, &content) && match_lines(&MATHEMATICA_2, &content)
            {
                Some("Mathematica")
            } else if match_lines(&MATLAB_1, &content) {
                Some("MATLAB")
            } else if match_lines(&LIMBO_1, &content) {
                Some("Limbo")
            } else {
                None
            }
        }
        ".m4" => {
            if match_lines(&M4SUGAR_1, &content) || match_lines(&M4SUGAR_2, &content) {
                Some("M4Sugar")
            } else {
                Some("M4")
            }
        }
        ".mask" => {
            if match_lines(&UNITY3D_ASSET_1, &content) {
                Some("Unity3D Asset")
            } else {
                None
            }
        }
        ".md" => {
            if match_lines(&MARKDOWN_1, &content) || match_lines(&MARKDOWN_2, &content) {
                Some("Markdown")
            } else if match_lines(&GCC_MACHINE_DESCRIPTION_1, &content) {
                Some("GCC Machine Description")
            } else {
                Some("Markdown")
            }
        }
        ".ml" => {
            if match_lines(&OCAML_1, &content) {
                Some("OCaml")
            } else if match_lines(&STANDARD_ML_1, &content) {
                Some("Standard ML")
            } else {
                None
            }
        }
        ".mm" => {
            if match_lines(&XML_8, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".mod" => {
            if match_lines(&XML_3, &content) {
                Some("XML")
            } else if match_lines(&MODULA_2_1, &content) {
                Some("Modula-2")
            } else {
                Some("Linux Kernel Module")
            }
        }
        ".moo" => {
            if match_lines(&MOOCODE_1, &content) {
                Some("Moocode")
            } else if match_lines(&MERCURY_1, &content) {
                Some("Mercury")
            } else {
                None
            }
        }
        ".ms" => {
            if match_lines(&ROFF_3, &content) {
                Some("Roff")
            } else if match_lines(&UNIX_ASSEMBLY_1, &content) {
                Some("Unix Assembly")
            } else {
                Some("MAXScript")
            }
        }
        ".n" => {
            if match_lines(&ROFF_4, &content) {
                Some("Roff")
            } else if match_lines(&NEMERLE_1, &content) {
                Some("Nemerle")
            } else {
                None
            }
        }
        ".ncl" => {
            if match_lines(&XML_4, &content) {
                Some("XML")
            } else if match_lines(&GERBER_IMAGE_3, &content) {
                Some("Gerber Image")
            } else if match_lines(&TEXT_1, &content) {
                Some("Text")
            } else {
                None
            }
        }
        ".nl" => {
            if match_lines(&NL_1, &content) {
                Some("NL")
            } else {
                Some("NewLisp")
            }
        }
        ".odin" => {
            if match_lines(&OBJECT_DATA_INSTANCE_NOTATION_1, &content) {
                Some("Object Data Instance Notation")
            } else if match_lines(&ODIN_1, &content) {
                Some("Odin")
            } else {
                None
            }
        }
        ".p" => {
            if match_lines(&GNUPLOT_1, &content) || match_lines(&GNUPLOT_2, &content) {
                Some("Gnuplot")
            } else {
                Some("OpenEdge ABL")
            }
        }
        ".php" => {
            if match_lines(&HACK_2, &content) {
                Some("Hack")
            } else if match_lines(&PHP_2, &content) {
                Some("PHP")
            } else {
                None
            }
        }
        ".pl" => {
            if match_lines(&PROLOG_1, &content) {
                Some("Prolog")
            } else if match_lines(&PERL5_1, &content) {
                Some("Perl")
            } else if match_lines(&PERL6_1, &content) {
                Some("Raku")
            } else {
                None
            }
        }
        ".plist" => {
            if match_lines(&XML_PROPERTY_LIST_1, &content) {
                Some("XML Property List")
            } else {
                Some("OpenStep Property List")
            }
        }
        ".pluginspec" => {
            if match_lines(&XML_8, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".pm" => {
            if match_lines(&PERL5_1, &content) {
                Some("Perl")
            } else if match_lines(&PERL6_1, &content) {
                Some("Raku")
            } else if match_lines(&X_PIXMAP_1, &content) {
                Some("X PixMap")
            } else {
                None
            }
        }
        ".pod" => {
            if match_lines(&POD_6_1, &content) {
                Some("Pod 6")
            } else {
                Some("Pod")
            }
        }
        ".pp" => {
            if match_lines(&PASCAL_1, &content) {
                Some("Pascal")
            } else if match_lines(&PUPPET_1, &content) {
                Some("Puppet")
            } else {
                None
            }
        }
        ".pro" => {
            if match_lines(&PROGUARD_1, &content) {
                Some("Proguard")
            } else if match_lines(&PROLOG_1, &content) {
                Some("Prolog")
            } else if match_lines(&INI_1, &content) {
                Some("INI")
            } else if match_lines(&QMAKE_1, &content) && match_lines(&QMAKE_2, &content) {
                Some("QMake")
            } else if match_lines(&IDL_1, &content) {
                Some("IDL")
            } else {
                None
            }
        }
        ".properties" => {
            if match_lines(&KEY_EQUALS_VALUE_1, &content) && match_lines(&INI_2, &content) {
                Some("INI")
            } else if match_lines(&KEY_EQUALS_VALUE_1, &content)
                && match_lines(&JAVA_PROPERTIES_1, &content)
            {
                Some("Java Properties")
            } else if match_lines(&KEY_EQUALS_VALUE_1, &content) {
                Some("INI")
            } else if match_lines(&JAVA_PROPERTIES_1, &content) {
                Some("Java properties")
            } else {
                None
            }
        }
        ".q" => {
            if match_lines(&Q_1, &content) {
                Some("q")
            } else if match_lines(&HIVEQL_1, &content) {
                Some("HiveQL")
            } else {
                None
            }
        }
        ".qs" => {
            if match_lines(&Q_SHARP__1, &content) {
                Some("Q#")
            } else if match_lines(&QT_SCRIPT_1, &content) {
                Some("Qt Script")
            } else {
                None
            }
        }
        ".r" => {
            if match_lines(&REBOL_1, &content) {
                Some("Rebol")
            } else if match_lines(&R_1, &content) {
                Some("R")
            } else {
                None
            }
        }
        ".re" => {
            if match_lines(&REASON_1, &content)
                || match_lines(&REASON_2, &content)
                || match_lines(&REASON_3, &content)
            {
                Some("Reason")
            } else if match_lines(&C_PLUS__PLUS__1, &content)
                || match_lines(&C_PLUS__PLUS__2, &content)
            {
                Some("C++")
            } else {
                None
            }
        }
        ".res" => {
            if match_lines(&RESCRIPT_1, &content) || match_lines(&RESCRIPT_2, &content) {
                Some("ReScript")
            } else if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".rno" => {
            if match_lines(&RUNOFF_1, &content) {
                Some("RUNOFF")
            } else if match_lines(&ROFF_5, &content) {
                Some("Roff")
            } else {
                None
            }
        }
        ".rpy" => {
            if match_lines(&PYTHON_1, &content) {
                Some("Python")
            } else {
                Some("Ren'Py")
            }
        }
        ".rs" => {
            if match_lines(&RUST_1, &content) {
                Some("Rust")
            } else if match_lines(&RENDERSCRIPT_1, &content) {
                Some("RenderScript")
            } else if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".s" => {
            if match_lines(&M68K_1, &content)
                || match_lines(&M68K_2, &content)
                || match_lines(&M68K_3, &content)
                || match_lines(&M68K_4, &content)
                || match_lines(&M68K_5, &content)
                || match_lines(&M68K_6, &content)
                || match_lines(&M68K_7, &content)
            {
                Some("Motorola 68K Assembly")
            } else {
                None
            }
        }
        ".sc" => {
            if match_lines(&SUPERCOLLIDER_1, &content) {
                Some("SuperCollider")
            } else if match_lines(&SCALA_1, &content) {
                Some("Scala")
            } else {
                None
            }
        }
        ".sch" => {
            if match_lines(&EAGLE_1, &content) {
                Some("Eagle")
            } else if match_lines(&XML_5, &content) {
                Some("XML")
            } else if match_lines(&SCHEME_1, &content) {
                Some("Scheme")
            } else {
                Some("KiCad Schematic")
            }
        }
        ".sls" => {
            if match_lines(&SCHEME_1, &content) {
                Some("Scheme")
            } else {
                Some("SaltStack")
            }
        }
        ".sol" => {
            if match_lines(&SOLIDITY_1, &content) {
                Some("Solidity")
            } else if match_lines(&GERBER_IMAGE_4, &content) {
                Some("Gerber Image")
            } else {
                None
            }
        }
        ".sql" => {
            if match_lines(&PLPGSQL_1, &content) {
                Some("PLpgSQL")
            } else if match_lines(&SQLPL_1, &content) {
                Some("SQLPL")
            } else if match_lines(&PLSQL_1, &content) {
                Some("PLSQL")
            } else if match_lines(&TSQL_1, &content) {
                Some("TSQL")
            } else {
                Some("SQL")
            }
        }
        ".srt" => {
            if match_lines(&SUBRIP_TEXT_1, &content) {
                Some("SubRip Text")
            } else {
                None
            }
        }
        ".st" => {
            if match_lines(&HTML_1, &content) {
                Some("HTML")
            } else if match_lines(&C_PLUS__PLUS__1, &content)
                || match_lines(&C_PLUS__PLUS__2, &content)
            {
                Some("C++")
            } else {
                Some("Smalltalk")
            }
        }
        ".t" => {
            if match_lines(&PERL5_1, &content) {
                Some("Perl")
            } else if match_lines(&RAKU_1, &content) {
                Some("Raku")
            } else if match_lines(&TURING_1, &content) {
                Some("Turing")
            } else {
                None
            }
        }
        ".toc" => {
            if match_lines(&WORLD_OF_WARCRAFT_ADDON_DATA_1, &content) {
                Some("World of Warcraft Addon Data")
            } else if match_lines(&TEX_2, &content) {
                Some("TeX")
            } else {
                None
            }
        }
        ".ts" => {
            if match_lines(&XML_6, &content) {
                Some("XML")
            } else {
                Some("TypeScript")
            }
        }
        ".tst" => {
            if match_lines(&GAP_2, &content) {
                Some("GAP")
            } else {
                Some("Scilab")
            }
        }
        ".tsx" => {
            if match_lines(&TSX_1, &content) {
                Some("TSX")
            } else if match_lines(&XML_7, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".txt" => {
            if match_lines(&VIM_HELP_FILE_1, &content) {
                Some("Vim Help File")
            } else {
                Some("Text")
            }
        }
        ".v" => {
            if match_lines(&COQ_1, &content) {
                Some("Coq")
            } else if match_lines(&VERILOG_1, &content) {
                Some("Verilog")
            } else if match_lines(&V_1, &content) {
                Some("V")
            } else {
                None
            }
        }
        ".vba" => {
            if match_lines(&VIM_SCRIPT_1, &content) {
                Some("Vim script")
            } else {
                Some("VBA")
            }
        }
        ".w" => {
            if match_lines(&OPENEDGE_ABL_1, &content) {
                Some("OpenEdge ABL")
            } else if match_lines(&CWEB_1, &content) {
                Some("CWeb")
            } else {
                None
            }
        }
        ".workflow" => {
            if match_lines(&XML_5, &content) {
                Some("XML")
            } else {
                None
            }
        }
        ".x" => {
            if match_lines(&DIRECTX_3D_FILE_1, &content) {
                Some("DirectX 3D File")
            } else if match_lines(&RPC_1, &content) {
                Some("RPC")
            } else if match_lines(&LOGOS_1, &content) {
                Some("Logos")
            } else if match_lines(&LINKER_SCRIPT_1, &content) {
                Some("Linker Script")
            } else {
                None
            }
        }
        ".yy" => {
            if match_lines(&JSON_2, &content) {
                Some("JSON")
            } else {
                Some("Yacc")
            }
        }
        ".bf" => {
            if match_lines(&BRAINFUCK_1, &content) {
                Some("Brainfuck")
            } else if match_lines(&BEEF_1, &content) {
                Some("Beef")
            } else {
                Some("HyPhy")
            }
        }
        ".b" => {
            if match_lines(&BRAINFUCK_1, &content) {
                Some("Brainfuck")
            } else {
                Some("Limbo")
            }
        }
        _ => None,
    }
}
fn match_lines(regex: &Regex, text: &str) -> bool {
    let lines: Vec<&str> = text.lines().collect::<Vec<_>>();
    for line in lines {
        if regex.is_match(line).unwrap() {
            return true;
        }
    }
    return false;
}
