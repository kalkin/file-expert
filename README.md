# file-expert

An expert system for recognizing source file, similar to
[GitHub/linguist](https://github.com/github/linguist), but written in Rust.

## Usage

```shell
file-expert FILE…
```

The program can also read the files from `STDIN` like this:

```shell
git ls-files|file-expert
```

### Output Format

Each output line contains the filename and the file type separated by the tab
character.

#### Output Example

```
.gitignore      Ignore List
.gitlint.yaml   YAML
.gitmodules     Git Config
COPYING.md      Markdown
Makefile.am     Makefile
README.md       Markdown
configure.ac    M4Sugar
extra-extensions-kb.pl  Prolog
file-expert.pl  Prolog
heuristics.yml  YAML
languages.yml   YAML
m4/BKG_DIRECTORY_VARIABLES.m4   M4Sugar
m4/BKG_ENABLE_XDG.m4    M4Sugar
m4/BKG_PACKAGE_INFO.m4  M4Sugar
run-tests       Shell
transform.py    Python
```

## Installation

### Build Dependencies

* cargo

### Via Cargo

`carg install file-expert`

#### From Source

⒈ Checkout this repository `git clone https://github.com/kalkin/file-expert`.

⒉ Run `cargo install --path .` to build and install it.

## Current State

In production.

## Versioning

This project uses [SemVer](http://semver.org/) for versioning. For the versions
available, see the [tags on this repository](https://github.com/kalkin/file-expert/tags).

## History

Started around November 2017. Replaced with a Nim implementation in December
2018, because at this point the GitHub/linguist heuristics were a huge mess of
`if … else` Ruby spaghetti code.

This project was archived until January 2019. My Nim implementation was broken
by the compiler update. While fixing it I noticed that the GitHub/linguist
heuristics were refactored to a YAML file. This opened up the possibility to
easily autogenerate the heuristics rules. So I started working on the Prolog
implementation again.

During the COVID-19 Summer in June 2021 I got proficient enough to rewrite this
project in Rust. The main benefit is the sheer performance gain.

## License

This project is licensed under the GNU Affero General Public License - see the
[COPYING.md](COPYING.md) file for details.

## AUTHOR

Written by Bahtiar \`kalkin-\` Gadimov.
