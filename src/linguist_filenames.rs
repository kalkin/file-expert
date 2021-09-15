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
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref FILENAMES: HashMap<String, String> = [
        ("APKBUILD".to_string(), "Alpine Abuild".to_string()),
        ("ant.xml".to_string(), "Ant Build System".to_string()),
        ("build.xml".to_string(), "Ant Build System".to_string()),
        (".htaccess".to_string(), "ApacheConf".to_string()),
        ("apache2.conf".to_string(), "ApacheConf".to_string()),
        ("httpd.conf".to_string(), "ApacheConf".to_string()),
        (".browserslistrc".to_string(), "Browserslist".to_string()),
        ("browserslist".to_string(), "Browserslist".to_string()),
        ("CMakeLists.txt".to_string(), "CMake".to_string()),
        ("CODEOWNERS".to_string(), "CODEOWNERS".to_string()),
        ("cabal.config".to_string(), "Cabal Config".to_string()),
        ("cabal.project".to_string(), "Cabal Config".to_string()),
        ("riemann.config".to_string(), "Clojure".to_string()),
        (
            "firestore.rules".to_string(),
            "Cloud Firestore Security Rules".to_string()
        ),
        ("Cakefile".to_string(), "CoffeeScript".to_string()),
        ("Dockerfile".to_string(), "Dockerfile".to_string()),
        (".editorconfig".to_string(), "EditorConfig".to_string()),
        ("mix.lock".to_string(), "Elixir".to_string()),
        (".abbrev_defs".to_string(), "Emacs Lisp".to_string()),
        (".emacs".to_string(), "Emacs Lisp".to_string()),
        (".emacs.desktop".to_string(), "Emacs Lisp".to_string()),
        (".gnus".to_string(), "Emacs Lisp".to_string()),
        (".spacemacs".to_string(), "Emacs Lisp".to_string()),
        (".viper".to_string(), "Emacs Lisp".to_string()),
        ("Cask".to_string(), "Emacs Lisp".to_string()),
        ("Project.ede".to_string(), "Emacs Lisp".to_string()),
        ("_emacs".to_string(), "Emacs Lisp".to_string()),
        ("abbrev_defs".to_string(), "Emacs Lisp".to_string()),
        ("Emakefile".to_string(), "Erlang".to_string()),
        ("rebar.config".to_string(), "Erlang".to_string()),
        ("rebar.config.lock".to_string(), "Erlang".to_string()),
        ("rebar.lock".to_string(), "Erlang".to_string()),
        (".factor-boot-rc".to_string(), "Factor".to_string()),
        (".factor-rc".to_string(), "Factor".to_string()),
        ("Fakefile".to_string(), "Fancy".to_string()),
        (".gn".to_string(), "GN".to_string()),
        ("Gemfile.lock".to_string(), "Gemfile.lock".to_string()),
        (".gitattributes".to_string(), "Git Attributes".to_string()),
        (".gitconfig".to_string(), "Git Config".to_string()),
        (".gitmodules".to_string(), "Git Config".to_string()),
        ("Jenkinsfile".to_string(), "Groovy".to_string()),
        ("haproxy.cfg".to_string(), "HAProxy".to_string()),
        ("buildozer.spec".to_string(), "INI".to_string()),
        (".atomignore".to_string(), "Ignore List".to_string()),
        (".babelignore".to_string(), "Ignore List".to_string()),
        (".bzrignore".to_string(), "Ignore List".to_string()),
        (".coffeelintignore".to_string(), "Ignore List".to_string()),
        (".cvsignore".to_string(), "Ignore List".to_string()),
        (".dockerignore".to_string(), "Ignore List".to_string()),
        (".eleventyignore".to_string(), "Ignore List".to_string()),
        (".eslintignore".to_string(), "Ignore List".to_string()),
        (".gitignore".to_string(), "Ignore List".to_string()),
        (".nodemonignore".to_string(), "Ignore List".to_string()),
        (".npmignore".to_string(), "Ignore List".to_string()),
        (".prettierignore".to_string(), "Ignore List".to_string()),
        (".stylelintignore".to_string(), "Ignore List".to_string()),
        (".vscodeignore".to_string(), "Ignore List".to_string()),
        ("gitignore-global".to_string(), "Ignore List".to_string()),
        ("gitignore_global".to_string(), "Ignore List".to_string()),
        ("ROOT".to_string(), "Isabelle ROOT".to_string()),
        (".arcconfig".to_string(), "JSON".to_string()),
        (".htmlhintrc".to_string(), "JSON".to_string()),
        (".tern-config".to_string(), "JSON".to_string()),
        (".tern-project".to_string(), "JSON".to_string()),
        (".watchmanconfig".to_string(), "JSON".to_string()),
        ("Pipfile.lock".to_string(), "JSON".to_string()),
        ("composer.lock".to_string(), "JSON".to_string()),
        ("mcmod.info".to_string(), "JSON".to_string()),
        (".babelrc".to_string(), "JSON with Comments".to_string()),
        (
            ".eslintrc.json".to_string(),
            "JSON with Comments".to_string()
        ),
        (".jscsrc".to_string(), "JSON with Comments".to_string()),
        (".jshintrc".to_string(), "JSON with Comments".to_string()),
        (".jslintrc".to_string(), "JSON with Comments".to_string()),
        (
            "devcontainer.json".to_string(),
            "JSON with Comments".to_string()
        ),
        (
            "jsconfig.json".to_string(),
            "JSON with Comments".to_string()
        ),
        (
            "language-configuration.json".to_string(),
            "JSON with Comments".to_string()
        ),
        (
            "tsconfig.json".to_string(),
            "JSON with Comments".to_string()
        ),
        ("tslint.json".to_string(), "JSON with Comments".to_string()),
        ("Jakefile".to_string(), "JavaScript".to_string()),
        ("Notebook".to_string(), "Jupyter Notebook".to_string()),
        ("fp-lib-table".to_string(), "KiCad Layout".to_string()),
        ("Lexer.x".to_string(), "Lex".to_string()),
        ("lexer.x".to_string(), "Lex".to_string()),
        ("ld.script".to_string(), "Linker Script".to_string()),
        ("Slakefile".to_string(), "LiveScript".to_string()),
        (".luacheckrc".to_string(), "Lua".to_string()),
        ("configure.ac".to_string(), "M4Sugar".to_string()),
        ("BSDmakefile".to_string(), "Makefile".to_string()),
        ("GNUmakefile".to_string(), "Makefile".to_string()),
        ("Kbuild".to_string(), "Makefile".to_string()),
        ("Makefile".to_string(), "Makefile".to_string()),
        ("Makefile.am".to_string(), "Makefile".to_string()),
        ("Makefile.boot".to_string(), "Makefile".to_string()),
        ("Makefile.builder".to_string(), "Makefile".to_string()),
        ("Makefile.frag".to_string(), "Makefile".to_string()),
        ("Makefile.in".to_string(), "Makefile".to_string()),
        ("Makefile.inc".to_string(), "Makefile".to_string()),
        ("Makefile.wat".to_string(), "Makefile".to_string()),
        ("makefile".to_string(), "Makefile".to_string()),
        ("makefile.sco".to_string(), "Makefile".to_string()),
        ("mkfile".to_string(), "Makefile".to_string()),
        ("contents.lr".to_string(), "Markdown".to_string()),
        ("pom.xml".to_string(), "Maven POM".to_string()),
        ("meson.build".to_string(), "Meson".to_string()),
        ("meson_options.txt".to_string(), "Meson".to_string()),
        (
            "descrip.mmk".to_string(),
            "Module Management System".to_string()
        ),
        (
            "descrip.mms".to_string(),
            "Module Management System".to_string()
        ),
        (".npmrc".to_string(), "NPM Config".to_string()),
        ("nextflow.config".to_string(), "Nextflow".to_string()),
        ("nginx.conf".to_string(), "Nginx".to_string()),
        ("nim.cfg".to_string(), "Nim".to_string()),
        ("Nukefile".to_string(), "Nu".to_string()),
        (".php".to_string(), "PHP".to_string()),
        (".php_cs".to_string(), "PHP".to_string()),
        (".php_cs.dist".to_string(), "PHP".to_string()),
        ("Phakefile".to_string(), "PHP".to_string()),
        ("Makefile.PL".to_string(), "Perl".to_string()),
        ("Rexfile".to_string(), "Perl".to_string()),
        ("ack".to_string(), "Perl".to_string()),
        ("cpanfile".to_string(), "Perl".to_string()),
        ("Modulefile".to_string(), "Puppet".to_string()),
        (".gclient".to_string(), "Python".to_string()),
        ("DEPS".to_string(), "Python".to_string()),
        ("SConscript".to_string(), "Python".to_string()),
        ("SConstruct".to_string(), "Python".to_string()),
        ("Snakefile".to_string(), "Python".to_string()),
        ("wscript".to_string(), "Python".to_string()),
        ("__init__.py".to_string(), "Python".to_string()),
        ("installscript.qs".to_string(), "Qt Script".to_string()),
        (
            "toolchain_installscript.qs".to_string(),
            "Qt Script".to_string()
        ),
        ("m3makefile".to_string(), "Quake".to_string()),
        ("m3overrides".to_string(), "Quake".to_string()),
        (".Rprofile".to_string(), "R".to_string()),
        ("expr-dist".to_string(), "R".to_string()),
        (".inputrc".to_string(), "Readline Config".to_string()),
        ("inputrc".to_string(), "Readline Config".to_string()),
        (
            "language-subtag-registry.txt".to_string(),
            "Record Jar".to_string()
        ),
        ("_redirects".to_string(), "Redirect Rules".to_string()),
        ("eqnrc".to_string(), "Roff".to_string()),
        ("mmn".to_string(), "Roff".to_string()),
        ("mmt".to_string(), "Roff".to_string()),
        ("troffrc".to_string(), "Roff".to_string()),
        ("troffrc-end".to_string(), "Roff".to_string()),
        (".irbrc".to_string(), "Ruby".to_string()),
        (".pryrc".to_string(), "Ruby".to_string()),
        (".simplecov".to_string(), "Ruby".to_string()),
        ("Appraisals".to_string(), "Ruby".to_string()),
        ("Berksfile".to_string(), "Ruby".to_string()),
        ("Brewfile".to_string(), "Ruby".to_string()),
        ("Buildfile".to_string(), "Ruby".to_string()),
        ("Capfile".to_string(), "Ruby".to_string()),
        ("Dangerfile".to_string(), "Ruby".to_string()),
        ("Deliverfile".to_string(), "Ruby".to_string()),
        ("Fastfile".to_string(), "Ruby".to_string()),
        ("Gemfile".to_string(), "Ruby".to_string()),
        ("Guardfile".to_string(), "Ruby".to_string()),
        ("Jarfile".to_string(), "Ruby".to_string()),
        ("Mavenfile".to_string(), "Ruby".to_string()),
        ("Podfile".to_string(), "Ruby".to_string()),
        ("Puppetfile".to_string(), "Ruby".to_string()),
        ("Rakefile".to_string(), "Ruby".to_string()),
        ("Snapfile".to_string(), "Ruby".to_string()),
        ("Thorfile".to_string(), "Ruby".to_string()),
        ("Vagrantfile".to_string(), "Ruby".to_string()),
        ("buildfile".to_string(), "Ruby".to_string()),
        ("file_contexts".to_string(), "SELinux Policy".to_string()),
        ("genfs_contexts".to_string(), "SELinux Policy".to_string()),
        ("initial_sids".to_string(), "SELinux Policy".to_string()),
        ("port_contexts".to_string(), "SELinux Policy".to_string()),
        ("security_classes".to_string(), "SELinux Policy".to_string()),
        ("ssh-config".to_string(), "SSH Config".to_string()),
        ("ssh_config".to_string(), "SSH Config".to_string()),
        ("sshconfig".to_string(), "SSH Config".to_string()),
        ("sshconfig.snip".to_string(), "SSH Config".to_string()),
        ("sshd-config".to_string(), "SSH Config".to_string()),
        ("sshd_config".to_string(), "SSH Config".to_string()),
        (".bash_aliases".to_string(), "Shell".to_string()),
        (".bash_history".to_string(), "Shell".to_string()),
        (".bash_logout".to_string(), "Shell".to_string()),
        (".bash_profile".to_string(), "Shell".to_string()),
        (".bashrc".to_string(), "Shell".to_string()),
        (".cshrc".to_string(), "Shell".to_string()),
        (".env".to_string(), "Shell".to_string()),
        (".env.example".to_string(), "Shell".to_string()),
        (".flaskenv".to_string(), "Shell".to_string()),
        (".login".to_string(), "Shell".to_string()),
        (".profile".to_string(), "Shell".to_string()),
        (".zlogin".to_string(), "Shell".to_string()),
        (".zlogout".to_string(), "Shell".to_string()),
        (".zprofile".to_string(), "Shell".to_string()),
        (".zshenv".to_string(), "Shell".to_string()),
        (".zshrc".to_string(), "Shell".to_string()),
        ("9fs".to_string(), "Shell".to_string()),
        ("PKGBUILD".to_string(), "Shell".to_string()),
        ("bash_aliases".to_string(), "Shell".to_string()),
        ("bash_logout".to_string(), "Shell".to_string()),
        ("bash_profile".to_string(), "Shell".to_string()),
        ("bashrc".to_string(), "Shell".to_string()),
        ("cshrc".to_string(), "Shell".to_string()),
        ("gradlew".to_string(), "Shell".to_string()),
        ("login".to_string(), "Shell".to_string()),
        ("man".to_string(), "Shell".to_string()),
        ("profile".to_string(), "Shell".to_string()),
        ("zlogin".to_string(), "Shell".to_string()),
        ("zlogout".to_string(), "Shell".to_string()),
        ("zprofile".to_string(), "Shell".to_string()),
        ("zshenv".to_string(), "Shell".to_string()),
        ("zshrc".to_string(), "Shell".to_string()),
        ("Singularity".to_string(), "Singularity".to_string()),
        ("Android.bp".to_string(), "Soong".to_string()),
        ("BUCK".to_string(), "Starlark".to_string()),
        ("BUILD".to_string(), "Starlark".to_string()),
        ("BUILD.bazel".to_string(), "Starlark".to_string()),
        ("Tiltfile".to_string(), "Starlark".to_string()),
        ("WORKSPACE".to_string(), "Starlark".to_string()),
        ("Cargo.lock".to_string(), "TOML".to_string()),
        ("Gopkg.lock".to_string(), "TOML".to_string()),
        ("Pipfile".to_string(), "TOML".to_string()),
        ("poetry.lock".to_string(), "TOML".to_string()),
        ("alire.lock".to_string(), "TOML".to_string()),
        ("owh".to_string(), "Tcl".to_string()),
        ("starfield".to_string(), "Tcl".to_string()),
        ("AUTHORS".to_string(), "Text".to_string()),
        ("CHANGELOG".to_string(), "Text".to_string()),
        ("COPYING".to_string(), "Text".to_string()),
        ("COPYING.LESSER".to_string(), "Text".to_string()),
        ("COPYING.regex".to_string(), "Text".to_string()),
        ("COPYRIGHT.regex".to_string(), "Text".to_string()),
        ("FONTLOG".to_string(), "Text".to_string()),
        ("HACKING".to_string(), "Text".to_string()),
        ("INSTALL".to_string(), "Text".to_string()),
        ("INSTALL.mysql".to_string(), "Text".to_string()),
        ("LICENSE".to_string(), "Text".to_string()),
        ("LICENSE-GPL2".to_string(), "Text".to_string()),
        ("LICENSE.mysql".to_string(), "Text".to_string()),
        ("NEWS".to_string(), "Text".to_string()),
        ("README".to_string(), "Text".to_string()),
        ("README.1ST".to_string(), "Text".to_string()),
        ("README.me".to_string(), "Text".to_string()),
        ("README.mysql".to_string(), "Text".to_string()),
        ("README.nss".to_string(), "Text".to_string()),
        ("changelog".to_string(), "Text".to_string()),
        ("click.me".to_string(), "Text".to_string()),
        ("delete.me".to_string(), "Text".to_string()),
        ("go.mod".to_string(), "Text".to_string()),
        ("go.sum".to_string(), "Text".to_string()),
        ("keep.me".to_string(), "Text".to_string()),
        ("package.mask".to_string(), "Text".to_string()),
        ("package.use.mask".to_string(), "Text".to_string()),
        ("package.use.stable.mask".to_string(), "Text".to_string()),
        ("read.me".to_string(), "Text".to_string()),
        ("readme.1st".to_string(), "Text".to_string()),
        ("readme".to_string(), "Text".to_string()),
        ("test.me".to_string(), "Text".to_string()),
        ("use.mask".to_string(), "Text".to_string()),
        ("use.stable.mask".to_string(), "Text".to_string()),
        ("version".to_string(), "Text".to_string()),
        (".exrc".to_string(), "Vim script".to_string()),
        (".gvimrc".to_string(), "Vim script".to_string()),
        (".nvimrc".to_string(), "Vim script".to_string()),
        (".vimrc".to_string(), "Vim script".to_string()),
        ("_vimrc".to_string(), "Vim script".to_string()),
        ("gvimrc".to_string(), "Vim script".to_string()),
        ("nvimrc".to_string(), "Vim script".to_string()),
        ("vimrc".to_string(), "Vim script".to_string()),
        (".wgetrc".to_string(), "Wget Config".to_string()),
        (
            "encodings.dir".to_string(),
            "X Font Directory Index".to_string()
        ),
        (
            "fonts.alias".to_string(),
            "X Font Directory Index".to_string()
        ),
        (
            "fonts.dir".to_string(),
            "X Font Directory Index".to_string()
        ),
        (
            "fonts.scale".to_string(),
            "X Font Directory Index".to_string()
        ),
        (".XCompose".to_string(), "XCompose".to_string()),
        ("XCompose".to_string(), "XCompose".to_string()),
        ("xcompose".to_string(), "XCompose".to_string()),
        (".classpath".to_string(), "XML".to_string()),
        (".cproject".to_string(), "XML".to_string()),
        (".project".to_string(), "XML".to_string()),
        ("App.config".to_string(), "XML".to_string()),
        ("NuGet.config".to_string(), "XML".to_string()),
        ("Settings.StyleCop".to_string(), "XML".to_string()),
        ("Web.Debug.config".to_string(), "XML".to_string()),
        ("Web.Release.config".to_string(), "XML".to_string()),
        ("Web.config".to_string(), "XML".to_string()),
        ("packages.config".to_string(), "XML".to_string()),
        (".clang-format".to_string(), "YAML".to_string()),
        (".clang-tidy".to_string(), "YAML".to_string()),
        (".gemrc".to_string(), "YAML".to_string()),
        ("glide.lock".to_string(), "YAML".to_string()),
        ("yarn.lock".to_string(), "YAML".to_string()),
        (".curlrc".to_string(), "cURL Config".to_string()),
        ("_curlrc".to_string(), "cURL Config".to_string()),
        (".dir_colors".to_string(), "dircolors".to_string()),
        (".dircolors".to_string(), "dircolors".to_string()),
        ("DIR_COLORS".to_string(), "dircolors".to_string()),
        ("_dir_colors".to_string(), "dircolors".to_string()),
        ("_dircolors".to_string(), "dircolors".to_string()),
        ("dir_colors".to_string(), "dircolors".to_string()),
        (".nanorc".to_string(), "nanorc".to_string()),
        ("nanorc".to_string(), "nanorc".to_string()),
        ("robots.txt".to_string(), "robots.txt".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
}
