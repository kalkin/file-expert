# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2021-09-18

### Added

- Support Jest Snapshots (github/linguist#5567)
- Support for Go Module and Go Checksum files (github/linguist#5504)
- JAR Manifest file (github/linguist#5505)
- `.markdownlintignore` to Ignore List filenames
- `.tcl.in` file extension (github/linguist#5517)
- Support `.makefile` extension (github/linguist#5526)
- three JSON filenames (github/linguist#5496)
- Astro language (github/linguist#5462)
- Support for Kakoune Script (github/linguist#5058)
- Extend Pascal heuristic (github/linguist#5143)
- Shebang parser handle env arguments & variables
- ".jav" to list of Java extensions (github/linguist#5397)
- Fennel (#5441)
- `.javascript` file extension (github/linguist#5414)
- PEG.js language (github/linguist#5376)
- support for Fluent, disambiguate from FreeMarker (github/linguist#5341)
- Valve Data Format Language (github/linguist#5330)
- @microsoft/api-extractor.json file to JSONC (github/linguist#5374)
- imgbotconfig to JSON filenames (github/linguist#5375)
- support for TextMate property files (github/linguist#5364)
- Android.bp file language for Soong (github/linguist#5361)
- SELinux Kernel Policy Language and Common Intermediate Language (github/linguist#5332)
- support for CUE language & Cue Sheet (github/linguist#5312)
- Android Interface Definition Language (github/linguist#5325)
- rs alias for rust (github/linguist#5321)
- Register `.hta` as an HTML extension (github/linguist#5533)

### Changed

- Remove duplicate README.1st (github/linguist#5504)
- Capitalize Vim Script (github/linguist#5532)
- Improve SourcePawn heuristics (github/linguist#5479)
- Separate Cython from Python (github/linguist#5462)
- Rename language entry for e-mail files (github/linguist#5437)
- Add colors for Awk and regular expression source (github/linguist#5392)
- Rename MediaWiki to Wikitext (github/linguist#5295)

### Fixed

- Handle VimBall files containing vim help modeline

## [v1.0.0-alpha.0] - 2021-09-06

### Changed

- Rewrite in Rust. The command line interface did not change. The results
  improved due to some bugs in the Prolog version. The performance increased.

## [0.13.1] - 2021-02-23

## [0.13.0] - 2021-02-20

### Changed

- Generate optimized builds

### Fixed

- Improve directory handling

## [0.12.0] - 2021-02-18

### Added

- Recognize binaries
- M4 vs M4Sugar disambiguation
- missing Gerber extensions and heuristics
- `.eleventyignore` to Ignore List category
- new Raku file extensions
- support for Boogie
- classify scdoc under Markdown
- support for record-jar files
- Beef language
- ImageJ Macro Language
- support for ABAP CDS

### Changed

- Split Nunjucks into its own language

### Fixed

- misclassified `.rs` XML files

## [0.11.1] - 2021-02-14

### Added

- Support for more languages through updated linguist heuristics

### Fixed

- Crash if file is unaccessible
- Recognize Makefile.builder files as Makefile

## [0.10.1] - 2020-03-29

### Added

- generated ebuild file
- pkgs/ to .gitignore
- support for Gentoo Portage ebuild packages

### Fixed

- custom extensions
- RPM spec add swipl dependency
- recognise RPM spec files

### Removed

- obsolete custom heuristics

## [0.10.0] - 2020-06-27

### Added

- AGPL header to source files
- `pkgs/` to `.gitignore`
- support for qubes-builder
- target `test` to `Makefile`
- tests for parsing filename extensions
- unittest for linguist samples

### Changed

- `file:list_files_recursive` to use relative paths
- project structure to recommended by SWI Prolog
- read buffer to 10 * 1024
- `t/linguist` to be deterministic
- speed up by ~16%
- move read_file to file module

### Fixed

- exception `max_files`
- handling files starting with a dot
- more failing tests
- `parse_extension`
- Singleton warning in `main([])` goal
- swi prolog warning
- typo in pydoc
- `parse_extension/2` handle files starting with dot and no extension

### Refactored

- file extension parsing to `module(file)`
- file guessing functions to `module(file_expert)`

## [0.9.0] - 2019-01-27

### Added

- First release
