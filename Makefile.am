bin_PROGRAMS = file-expert
file_expert_SOURCES = lib/file.pl \
					  src/file_expert.pl \
					  load.pl \
					  run.pl \
					  extra-extensions-kb.pl \
					  github-extensions-kb.pl

EXTRA_DIST = transform.py named_patterns.yml languages.yml heuristics.yml @BKG_RPM_SPECFILE@.in
CLEANFILES = @BKG_RPM_SPECFILE@

generate-files: tests/linguist.rs src/linguist_heuristics.rs src/linguist_extensions.rs src/linguist_interpreters.rs src/linguist_aliases.rs

%.rs: %.rs.php
	$< > $@

src/linguist_heuristics.rs: src/linguist_heuristics.rs.php disambiguations.yml named_patterns.yml
	$< > $@
	rustfmt $@

src/linguist_extensions.rs: src/linguist_extensions.rs.php languages.yml
	$< > $@
	rustfmt $@

src/linguist_interpreters.rs: src/linguist_interpreters.rs.php languages.yml
	$< > $@
	rustfmt $@

src/linguist_aliases.rs: src/linguist_aliases.rs.php languages.yml
	$< > $@
	rustfmt $@

@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
