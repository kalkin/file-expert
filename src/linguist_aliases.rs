use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ALIASES: HashMap<String, String> = [
        ("ags".to_string(), "AGS Script".to_string()),
        ("ags script".to_string(), "AGS Script".to_string()),
        ("aspx".to_string(), "ASP.NET".to_string()),
        ("aspx-vb".to_string(), "ASP.NET".to_string()),
        ("asp.net".to_string(), "ASP.NET".to_string()),
        ("ats2".to_string(), "ATS".to_string()),
        ("ats".to_string(), "ATS".to_string()),
        ("actionscript 3".to_string(), "ActionScript".to_string()),
        ("actionscript3".to_string(), "ActionScript".to_string()),
        ("as3".to_string(), "ActionScript".to_string()),
        ("actionscript".to_string(), "ActionScript".to_string()),
        ("ada95".to_string(), "Ada".to_string()),
        ("ada2005".to_string(), "Ada".to_string()),
        ("ada".to_string(), "Ada".to_string()),
        ("acfm".to_string(), "Adobe Font Metrics".to_string()),
        (
            "adobe composite font metrics".to_string(),
            "Adobe Font Metrics".to_string()
        ),
        (
            "adobe multiple font metrics".to_string(),
            "Adobe Font Metrics".to_string()
        ),
        ("amfm".to_string(), "Adobe Font Metrics".to_string()),
        (
            "adobe font metrics".to_string(),
            "Adobe Font Metrics".to_string()
        ),
        ("abuild".to_string(), "Alpine Abuild".to_string()),
        ("apkbuild".to_string(), "Alpine Abuild".to_string()),
        ("alpine abuild".to_string(), "Alpine Abuild".to_string()),
        ("altium".to_string(), "Altium Designer".to_string()),
        ("altium designer".to_string(), "Altium Designer".to_string()),
        ("aconf".to_string(), "ApacheConf".to_string()),
        ("apache".to_string(), "ApacheConf".to_string()),
        ("apacheconf".to_string(), "ApacheConf".to_string()),
        ("osascript".to_string(), "AppleScript".to_string()),
        ("applescript".to_string(), "AppleScript".to_string()),
        ("asm".to_string(), "Assembly".to_string()),
        ("nasm".to_string(), "Assembly".to_string()),
        ("assembly".to_string(), "Assembly".to_string()),
        ("ahk".to_string(), "AutoHotkey".to_string()),
        ("autohotkey".to_string(), "AutoHotkey".to_string()),
        ("au3".to_string(), "AutoIt".to_string()),
        ("AutoIt3".to_string(), "AutoIt".to_string()),
        ("AutoItScript".to_string(), "AutoIt".to_string()),
        ("autoit".to_string(), "AutoIt".to_string()),
        ("bat".to_string(), "Batchfile".to_string()),
        ("batch".to_string(), "Batchfile".to_string()),
        ("dosbatch".to_string(), "Batchfile".to_string()),
        ("winbatch".to_string(), "Batchfile".to_string()),
        ("batchfile".to_string(), "Batchfile".to_string()),
        ("b3d".to_string(), "BlitzBasic".to_string()),
        ("blitz3d".to_string(), "BlitzBasic".to_string()),
        ("blitzplus".to_string(), "BlitzBasic".to_string()),
        ("bplus".to_string(), "BlitzBasic".to_string()),
        ("blitzbasic".to_string(), "BlitzBasic".to_string()),
        ("bmax".to_string(), "BlitzMax".to_string()),
        ("blitzmax".to_string(), "BlitzMax".to_string()),
        ("csharp".to_string(), "C#".to_string()),
        ("cake".to_string(), "C#".to_string()),
        ("cakescript".to_string(), "C#".to_string()),
        ("c#".to_string(), "C#".to_string()),
        ("cpp".to_string(), "C++".to_string()),
        ("c++".to_string(), "C++".to_string()),
        ("c2hs".to_string(), "C2hs Haskell".to_string()),
        ("c2hs haskell".to_string(), "C2hs Haskell".to_string()),
        ("Cabal".to_string(), "Cabal Config".to_string()),
        ("cabal config".to_string(), "Cabal Config".to_string()),
        ("Carto".to_string(), "CartoCSS".to_string()),
        ("cartocss".to_string(), "CartoCSS".to_string()),
        ("chpl".to_string(), "Chapel".to_string()),
        ("chapel".to_string(), "Chapel".to_string()),
        ("asp".to_string(), "Classic ASP".to_string()),
        ("classic asp".to_string(), "Classic ASP".to_string()),
        ("soy".to_string(), "Closure Templates".to_string()),
        (
            "closure templates".to_string(),
            "Closure Templates".to_string()
        ),
        ("CoNLL".to_string(), "CoNLL-U".to_string()),
        ("CoNLL-X".to_string(), "CoNLL-U".to_string()),
        ("conll-u".to_string(), "CoNLL-U".to_string()),
        ("ql".to_string(), "CodeQL".to_string()),
        ("codeql".to_string(), "CodeQL".to_string()),
        ("coffee".to_string(), "CoffeeScript".to_string()),
        ("coffee-script".to_string(), "CoffeeScript".to_string()),
        ("coffeescript".to_string(), "CoffeeScript".to_string()),
        ("cfm".to_string(), "ColdFusion".to_string()),
        ("cfml".to_string(), "ColdFusion".to_string()),
        ("coldfusion html".to_string(), "ColdFusion".to_string()),
        ("coldfusion".to_string(), "ColdFusion".to_string()),
        ("cfc".to_string(), "ColdFusion CFC".to_string()),
        ("coldfusion cfc".to_string(), "ColdFusion CFC".to_string()),
        ("lisp".to_string(), "Common Lisp".to_string()),
        ("common lisp".to_string(), "Common Lisp".to_string()),
        ("cwl".to_string(), "Common Workflow Language".to_string()),
        (
            "common workflow language".to_string(),
            "Common Workflow Language".to_string()
        ),
        ("c++-objdump".to_string(), "Cpp-ObjDump".to_string()),
        ("cpp-objdump".to_string(), "Cpp-ObjDump".to_string()),
        ("csound-orc".to_string(), "Csound".to_string()),
        ("csound".to_string(), "Csound".to_string()),
        ("csound-csd".to_string(), "Csound Document".to_string()),
        ("csound document".to_string(), "Csound Document".to_string()),
        ("csound-sco".to_string(), "Csound Score".to_string()),
        ("csound score".to_string(), "Csound Score".to_string()),
        ("pyrex".to_string(), "Cython".to_string()),
        ("cython".to_string(), "Cython".to_string()),
        ("dcl".to_string(), "DIGITAL Command Language".to_string()),
        (
            "digital command language".to_string(),
            "DIGITAL Command Language".to_string()
        ),
        ("byond".to_string(), "DM".to_string()),
        ("dm".to_string(), "DM".to_string()),
        ("dtrace-script".to_string(), "DTrace".to_string()),
        ("dtrace".to_string(), "DTrace".to_string()),
        ("dpatch".to_string(), "Darcs Patch".to_string()),
        ("darcs patch".to_string(), "Darcs Patch".to_string()),
        ("udiff".to_string(), "Diff".to_string()),
        ("diff".to_string(), "Diff".to_string()),
        ("editor-config".to_string(), "EditorConfig".to_string()),
        ("editorconfig".to_string(), "EditorConfig".to_string()),
        ("elisp".to_string(), "Emacs Lisp".to_string()),
        ("emacs".to_string(), "Emacs Lisp".to_string()),
        ("emacs lisp".to_string(), "Emacs Lisp".to_string()),
        ("fsharp".to_string(), "F#".to_string()),
        ("f#".to_string(), "F#".to_string()),
        ("fstar".to_string(), "F*".to_string()),
        ("f*".to_string(), "F*".to_string()),
        ("FIGfont".to_string(), "FIGlet Font".to_string()),
        ("figlet font".to_string(), "FIGlet Font".to_string()),
        ("ftl".to_string(), "FreeMarker".to_string()),
        ("freemarker".to_string(), "FreeMarker".to_string()),
        ("xml+genshi".to_string(), "Genshi".to_string()),
        ("xml+kid".to_string(), "Genshi".to_string()),
        ("genshi".to_string(), "Genshi".to_string()),
        ("rs-274x".to_string(), "Gerber Image".to_string()),
        ("gerber image".to_string(), "Gerber Image".to_string()),
        ("pot".to_string(), "Gettext Catalog".to_string()),
        ("gettext catalog".to_string(), "Gettext Catalog".to_string()),
        ("cucumber".to_string(), "Gherkin".to_string()),
        ("gherkin".to_string(), "Gherkin".to_string()),
        ("gitattributes".to_string(), "Git Attributes".to_string()),
        ("git attributes".to_string(), "Git Attributes".to_string()),
        ("gitconfig".to_string(), "Git Config".to_string()),
        ("gitmodules".to_string(), "Git Config".to_string()),
        ("git config".to_string(), "Git Config".to_string()),
        ("golang".to_string(), "Go".to_string()),
        ("go".to_string(), "Go".to_string()),
        ("gf".to_string(), "Grammatical Framework".to_string()),
        (
            "grammatical framework".to_string(),
            "Grammatical Framework".to_string()
        ),
        ("gsp".to_string(), "Groovy Server Pages".to_string()),
        (
            "java server page".to_string(),
            "Groovy Server Pages".to_string()
        ),
        (
            "groovy server pages".to_string(),
            "Groovy Server Pages".to_string()
        ),
        (
            "HashiCorp Configuration Language".to_string(),
            "HCL".to_string()
        ),
        ("terraform".to_string(), "HCL".to_string()),
        ("hcl".to_string(), "HCL".to_string()),
        ("xhtml".to_string(), "HTML".to_string()),
        ("html".to_string(), "HTML".to_string()),
        ("django".to_string(), "HTML+Django".to_string()),
        ("html+django/jinja".to_string(), "HTML+Django".to_string()),
        ("html+jinja".to_string(), "HTML+Django".to_string()),
        ("htmldjango".to_string(), "HTML+Django".to_string()),
        ("html+django".to_string(), "HTML+Django".to_string()),
        ("ecr".to_string(), "HTML+ECR".to_string()),
        ("html+ecr".to_string(), "HTML+ECR".to_string()),
        ("eex".to_string(), "HTML+EEX".to_string()),
        ("leex".to_string(), "HTML+EEX".to_string()),
        ("html+eex".to_string(), "HTML+EEX".to_string()),
        ("erb".to_string(), "HTML+ERB".to_string()),
        ("rhtml".to_string(), "HTML+ERB".to_string()),
        ("html+ruby".to_string(), "HTML+ERB".to_string()),
        ("html+erb".to_string(), "HTML+ERB".to_string()),
        ("razor".to_string(), "HTML+Razor".to_string()),
        ("html+razor".to_string(), "HTML+Razor".to_string()),
        ("hbs".to_string(), "Handlebars".to_string()),
        ("htmlbars".to_string(), "Handlebars".to_string()),
        ("handlebars".to_string(), "Handlebars".to_string()),
        ("hylang".to_string(), "Hy".to_string()),
        ("hy".to_string(), "Hy".to_string()),
        ("igor".to_string(), "IGOR Pro".to_string()),
        ("igorpro".to_string(), "IGOR Pro".to_string()),
        ("igor pro".to_string(), "IGOR Pro".to_string()),
        ("dosini".to_string(), "INI".to_string()),
        ("ini".to_string(), "INI".to_string()),
        ("irc".to_string(), "IRC log".to_string()),
        ("irc logs".to_string(), "IRC log".to_string()),
        ("irc log".to_string(), "IRC log".to_string()),
        ("ignore".to_string(), "Ignore List".to_string()),
        ("gitignore".to_string(), "Ignore List".to_string()),
        ("git-ignore".to_string(), "Ignore List".to_string()),
        ("ignore list".to_string(), "Ignore List".to_string()),
        ("ijm".to_string(), "ImageJ Macro".to_string()),
        ("imagej macro".to_string(), "ImageJ Macro".to_string()),
        ("i7".to_string(), "Inform 7".to_string()),
        ("inform7".to_string(), "Inform 7".to_string()),
        ("inform 7".to_string(), "Inform 7".to_string()),
        ("jsonc".to_string(), "JSON with Comments".to_string()),
        (
            "json with comments".to_string(),
            "JSON with Comments".to_string()
        ),
        ("jsp".to_string(), "Java Server Pages".to_string()),
        (
            "java server pages".to_string(),
            "Java Server Pages".to_string()
        ),
        ("js".to_string(), "JavaScript".to_string()),
        ("node".to_string(), "JavaScript".to_string()),
        ("javascript".to_string(), "JavaScript".to_string()),
        (
            "IPython Notebook".to_string(),
            "Jupyter Notebook".to_string()
        ),
        (
            "jupyter notebook".to_string(),
            "Jupyter Notebook".to_string()
        ),
        ("ksy".to_string(), "Kaitai Struct".to_string()),
        ("kaitai struct".to_string(), "Kaitai Struct".to_string()),
        ("pcbnew".to_string(), "KiCad Layout".to_string()),
        ("kicad layout".to_string(), "KiCad Layout".to_string()),
        (
            "eeschema schematic".to_string(),
            "KiCad Schematic".to_string()
        ),
        ("kicad schematic".to_string(), "KiCad Schematic".to_string()),
        ("lassoscript".to_string(), "Lasso".to_string()),
        ("lasso".to_string(), "Lasso".to_string()),
        ("flex".to_string(), "Lex".to_string()),
        ("lex".to_string(), "Lex".to_string()),
        ("litcoffee".to_string(), "Literate CoffeeScript".to_string()),
        (
            "literate coffeescript".to_string(),
            "Literate CoffeeScript".to_string()
        ),
        ("lhaskell".to_string(), "Literate Haskell".to_string()),
        ("lhs".to_string(), "Literate Haskell".to_string()),
        (
            "literate haskell".to_string(),
            "Literate Haskell".to_string()
        ),
        ("live-script".to_string(), "LiveScript".to_string()),
        ("ls".to_string(), "LiveScript".to_string()),
        ("livescript".to_string(), "LiveScript".to_string()),
        ("mumps".to_string(), "M".to_string()),
        ("m".to_string(), "M".to_string()),
        ("autoconf".to_string(), "M4Sugar".to_string()),
        ("m4sugar".to_string(), "M4Sugar".to_string()),
        ("octave".to_string(), "MATLAB".to_string()),
        ("matlab".to_string(), "MATLAB".to_string()),
        ("m2".to_string(), "Macaulay2".to_string()),
        ("macaulay2".to_string(), "Macaulay2".to_string()),
        ("bsdmake".to_string(), "Makefile".to_string()),
        ("make".to_string(), "Makefile".to_string()),
        ("mf".to_string(), "Makefile".to_string()),
        ("makefile".to_string(), "Makefile".to_string()),
        ("pandoc".to_string(), "Markdown".to_string()),
        ("markdown".to_string(), "Markdown".to_string()),
        ("markojs".to_string(), "Marko".to_string()),
        ("marko".to_string(), "Marko".to_string()),
        ("mma".to_string(), "Mathematica".to_string()),
        ("mathematica".to_string(), "Mathematica".to_string()),
        ("max/msp".to_string(), "Max".to_string()),
        ("maxmsp".to_string(), "Max".to_string()),
        ("max".to_string(), "Max".to_string()),
        ("m68k".to_string(), "Motorola 68K Assembly".to_string()),
        (
            "motorola 68k assembly".to_string(),
            "Motorola 68K Assembly".to_string()
        ),
        ("amusewiki".to_string(), "Muse".to_string()),
        ("emacs muse".to_string(), "Muse".to_string()),
        ("muse".to_string(), "Muse".to_string()),
        ("nette object notation".to_string(), "NEON".to_string()),
        ("ne-on".to_string(), "NEON".to_string()),
        ("neon".to_string(), "NEON".to_string()),
        ("npmrc".to_string(), "NPM Config".to_string()),
        ("npm config".to_string(), "NPM Config".to_string()),
        ("nginx configuration file".to_string(), "Nginx".to_string()),
        ("nginx".to_string(), "Nginx".to_string()),
        ("nixos".to_string(), "Nix".to_string()),
        ("nix".to_string(), "Nix".to_string()),
        ("nush".to_string(), "Nu".to_string()),
        ("nu".to_string(), "Nu".to_string()),
        ("njk".to_string(), "Nunjucks".to_string()),
        ("nunjucks".to_string(), "Nunjucks".to_string()),
        ("obj-c".to_string(), "Objective-C".to_string()),
        ("objc".to_string(), "Objective-C".to_string()),
        ("objectivec".to_string(), "Objective-C".to_string()),
        ("objective-c".to_string(), "Objective-C".to_string()),
        ("obj-c++".to_string(), "Objective-C++".to_string()),
        ("objc++".to_string(), "Objective-C++".to_string()),
        ("objectivec++".to_string(), "Objective-C++".to_string()),
        ("objective-c++".to_string(), "Objective-C++".to_string()),
        ("obj-j".to_string(), "Objective-J".to_string()),
        ("objectivej".to_string(), "Objective-J".to_string()),
        ("objj".to_string(), "Objective-J".to_string()),
        ("objective-j".to_string(), "Objective-J".to_string()),
        ("odinlang".to_string(), "Odin".to_string()),
        ("odin-lang".to_string(), "Odin".to_string()),
        ("odin".to_string(), "Odin".to_string()),
        ("progress".to_string(), "OpenEdge ABL".to_string()),
        ("openedge".to_string(), "OpenEdge ABL".to_string()),
        ("abl".to_string(), "OpenEdge ABL".to_string()),
        ("openedge abl".to_string(), "OpenEdge ABL".to_string()),
        ("openrc".to_string(), "OpenRC runscript".to_string()),
        (
            "openrc runscript".to_string(),
            "OpenRC runscript".to_string()
        ),
        ("AFDKO".to_string(), "OpenType Feature File".to_string()),
        (
            "opentype feature file".to_string(),
            "OpenType Feature File".to_string()
        ),
        ("inc".to_string(), "PHP".to_string()),
        ("php".to_string(), "PHP".to_string()),
        ("pov-ray".to_string(), "POV-Ray SDL".to_string()),
        ("povray".to_string(), "POV-Ray SDL".to_string()),
        ("pov-ray sdl".to_string(), "POV-Ray SDL".to_string()),
        ("pasm".to_string(), "Parrot Assembly".to_string()),
        ("parrot assembly".to_string(), "Parrot Assembly".to_string()),
        (
            "pir".to_string(),
            "Parrot Internal Representation".to_string()
        ),
        (
            "parrot internal representation".to_string(),
            "Parrot Internal Representation".to_string()
        ),
        ("delphi".to_string(), "Pascal".to_string()),
        ("objectpascal".to_string(), "Pascal".to_string()),
        ("pascal".to_string(), "Pascal".to_string()),
        ("cperl".to_string(), "Perl".to_string()),
        ("perl".to_string(), "Perl".to_string()),
        ("postscr".to_string(), "PostScript".to_string()),
        ("postscript".to_string(), "PostScript".to_string()),
        ("posh".to_string(), "PowerShell".to_string()),
        ("pwsh".to_string(), "PowerShell".to_string()),
        ("powershell".to_string(), "PowerShell".to_string()),
        ("protobuf".to_string(), "Protocol Buffer".to_string()),
        (
            "Protocol Buffers".to_string(),
            "Protocol Buffer".to_string()
        ),
        ("protocol buffer".to_string(), "Protocol Buffer".to_string()),
        ("python3".to_string(), "Python".to_string()),
        ("rusthon".to_string(), "Python".to_string()),
        ("python".to_string(), "Python".to_string()),
        ("pycon".to_string(), "Python console".to_string()),
        ("python console".to_string(), "Python console".to_string()),
        ("qsharp".to_string(), "Q#".to_string()),
        ("q#".to_string(), "Q#".to_string()),
        ("R".to_string(), "R".to_string()),
        ("Rscript".to_string(), "R".to_string()),
        ("splus".to_string(), "R".to_string()),
        ("r".to_string(), "R".to_string()),
        ("arexx".to_string(), "REXX".to_string()),
        ("rexx".to_string(), "REXX".to_string()),
        ("rpcgen".to_string(), "RPC".to_string()),
        ("oncrpc".to_string(), "RPC".to_string()),
        ("xdr".to_string(), "RPC".to_string()),
        ("rpc".to_string(), "RPC".to_string()),
        ("specfile".to_string(), "RPM Spec".to_string()),
        ("rpm spec".to_string(), "RPM Spec".to_string()),
        ("ragel-rb".to_string(), "Ragel".to_string()),
        ("ragel-ruby".to_string(), "Ragel".to_string()),
        ("ragel".to_string(), "Ragel".to_string()),
        ("perl6".to_string(), "Raku".to_string()),
        ("perl-6".to_string(), "Raku".to_string()),
        ("raku".to_string(), "Raku".to_string()),
        ("raw".to_string(), "Raw token data".to_string()),
        ("raw token data".to_string(), "Raw token data".to_string()),
        ("inputrc".to_string(), "Readline Config".to_string()),
        ("readline".to_string(), "Readline Config".to_string()),
        ("readline config".to_string(), "Readline Config".to_string()),
        ("red/system".to_string(), "Red".to_string()),
        ("red".to_string(), "Red".to_string()),
        ("regexp".to_string(), "Regular Expression".to_string()),
        ("regex".to_string(), "Regular Expression".to_string()),
        (
            "regular expression".to_string(),
            "Regular Expression".to_string()
        ),
        ("renpy".to_string(), "Ren'Py".to_string()),
        ("ren'py".to_string(), "Ren'Py".to_string()),
        ("groff".to_string(), "Roff".to_string()),
        ("man".to_string(), "Roff".to_string()),
        ("manpage".to_string(), "Roff".to_string()),
        ("man page".to_string(), "Roff".to_string()),
        ("man-page".to_string(), "Roff".to_string()),
        ("mdoc".to_string(), "Roff".to_string()),
        ("nroff".to_string(), "Roff".to_string()),
        ("troff".to_string(), "Roff".to_string()),
        ("roff".to_string(), "Roff".to_string()),
        ("jruby".to_string(), "Ruby".to_string()),
        ("macruby".to_string(), "Ruby".to_string()),
        ("rake".to_string(), "Ruby".to_string()),
        ("rb".to_string(), "Ruby".to_string()),
        ("rbx".to_string(), "Ruby".to_string()),
        ("ruby".to_string(), "Ruby".to_string()),
        ("saltstate".to_string(), "SaltStack".to_string()),
        ("salt".to_string(), "SaltStack".to_string()),
        ("saltstack".to_string(), "SaltStack".to_string()),
        ("sh".to_string(), "Shell".to_string()),
        ("shell-script".to_string(), "Shell".to_string()),
        ("bash".to_string(), "Shell".to_string()),
        ("zsh".to_string(), "Shell".to_string()),
        ("shell".to_string(), "Shell".to_string()),
        ("bash session".to_string(), "ShellSession".to_string()),
        ("console".to_string(), "ShellSession".to_string()),
        ("shellsession".to_string(), "ShellSession".to_string()),
        ("coccinelle".to_string(), "SmPL".to_string()),
        ("smpl".to_string(), "SmPL".to_string()),
        ("squeak".to_string(), "Smalltalk".to_string()),
        ("smalltalk".to_string(), "Smalltalk".to_string()),
        ("sourcemod".to_string(), "SourcePawn".to_string()),
        ("sourcepawn".to_string(), "SourcePawn".to_string()),
        ("sml".to_string(), "Standard ML".to_string()),
        ("standard ml".to_string(), "Standard ML".to_string()),
        ("bazel".to_string(), "Starlark".to_string()),
        ("bzl".to_string(), "Starlark".to_string()),
        ("starlark".to_string(), "Starlark".to_string()),
        ("latex".to_string(), "TeX".to_string()),
        ("tex".to_string(), "TeX".to_string()),
        ("fundamental".to_string(), "Text".to_string()),
        ("plain text".to_string(), "Text".to_string()),
        ("text".to_string(), "Text".to_string()),
        ("tl".to_string(), "Type Language".to_string()),
        ("type language".to_string(), "Type Language".to_string()),
        ("ts".to_string(), "TypeScript".to_string()),
        ("typescript".to_string(), "TypeScript".to_string()),
        ("Ur/Web".to_string(), "UrWeb".to_string()),
        ("Ur".to_string(), "UrWeb".to_string()),
        ("urweb".to_string(), "UrWeb".to_string()),
        ("vlang".to_string(), "V".to_string()),
        ("v".to_string(), "V".to_string()),
        ("vb6".to_string(), "VBA".to_string()),
        ("visual basic 6".to_string(), "VBA".to_string()),
        (
            "visual basic for applications".to_string(),
            "VBA".to_string()
        ),
        ("vba".to_string(), "VBA".to_string()),
        ("vimhelp".to_string(), "Vim Help File".to_string()),
        ("vim help file".to_string(), "Vim Help File".to_string()),
        ("SnipMate".to_string(), "Vim Snippet".to_string()),
        ("UltiSnip".to_string(), "Vim Snippet".to_string()),
        ("UltiSnips".to_string(), "Vim Snippet".to_string()),
        ("NeoSnippet".to_string(), "Vim Snippet".to_string()),
        ("vim snippet".to_string(), "Vim Snippet".to_string()),
        ("vim".to_string(), "Vim script".to_string()),
        ("viml".to_string(), "Vim script".to_string()),
        ("nvim".to_string(), "Vim script".to_string()),
        ("vim script".to_string(), "Vim script".to_string()),
        ("visual basic".to_string(), "Visual Basic .NET".to_string()),
        ("vbnet".to_string(), "Visual Basic .NET".to_string()),
        ("vb .net".to_string(), "Visual Basic .NET".to_string()),
        ("vb.net".to_string(), "Visual Basic .NET".to_string()),
        (
            "visual basic .net".to_string(),
            "Visual Basic .NET".to_string()
        ),
        ("wast".to_string(), "WebAssembly".to_string()),
        ("wasm".to_string(), "WebAssembly".to_string()),
        ("webassembly".to_string(), "WebAssembly".to_string()),
        ("wgetrc".to_string(), "Wget Config".to_string()),
        ("wget config".to_string(), "Wget Config".to_string()),
        ("xbm".to_string(), "X BitMap".to_string()),
        ("x bitmap".to_string(), "X BitMap".to_string()),
        ("xpm".to_string(), "X PixMap".to_string()),
        ("x pixmap".to_string(), "X PixMap".to_string()),
        ("xten".to_string(), "X10".to_string()),
        ("x10".to_string(), "X10".to_string()),
        ("rss".to_string(), "XML".to_string()),
        ("xsd".to_string(), "XML".to_string()),
        ("wsdl".to_string(), "XML".to_string()),
        ("xml".to_string(), "XML".to_string()),
        ("xsl".to_string(), "XSLT".to_string()),
        ("xslt".to_string(), "XSLT".to_string()),
        ("yml".to_string(), "YAML".to_string()),
        ("yaml".to_string(), "YAML".to_string()),
        ("snippet".to_string(), "YASnippet".to_string()),
        ("yas".to_string(), "YASnippet".to_string()),
        ("yasnippet".to_string(), "YASnippet".to_string()),
        ("bro".to_string(), "Zeek".to_string()),
        ("zeek".to_string(), "Zeek".to_string()),
        ("curlrc".to_string(), "cURL Config".to_string()),
        ("curl config".to_string(), "cURL Config".to_string()),
        ("rst".to_string(), "reStructuredText".to_string()),
        (
            "restructuredtext".to_string(),
            "reStructuredText".to_string()
        ),
        ("advpl".to_string(), "xBase".to_string()),
        ("clipper".to_string(), "xBase".to_string()),
        ("foxpro".to_string(), "xBase".to_string()),
        ("xbase".to_string(), "xBase".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
}