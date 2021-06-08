bin_PROGRAMS = file-expert
file_expert_SOURCES = lib/file.pl \
					  src/file_expert.pl \
					  load.pl \
					  run.pl \
					  extra-extensions-kb.pl \
					  github-extensions-kb.pl

EXTRA_DIST = transform.py named_patterns.yml languages.yml heuristics.yml @BKG_RPM_SPECFILE@.in
CLEANFILES = @BKG_RPM_SPECFILE@

generate-files: tests/linguist.rs

%.rs: %.rs.php
	$< > $@


@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
