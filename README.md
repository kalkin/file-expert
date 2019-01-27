# file-expert

An expert system for recognizing file types, similar to GitHub/linguist, but
written in Prolog.

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

## Getting Started

### Build Dependencies

* Autoconf
* Automake
* SWI Prolog
* Python 3
* PyYAML


### Installation

⒈ Checkout this repository `git clone https://github.com/kalkin/file-expert`.

⒉ Run `autoreconf -i && ./configure --prefix=/usr && make` to build it.

⒊ Install with `sudo make install`

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

## License

This project is licensed under the GNU Affero General Public License - see the
[COPYING.md](COPYING.md) file for details.

## AUTHOR

Written by Bahtiar \`kalkin-\` Gadimov.
