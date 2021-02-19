bin_PROGRAMS = file-expert
file_expert_SOURCES = lib/file.pl \
					  src/file_expert.pl \
					  load.pl \
					  run.pl \
					  extra-extensions-kb.pl \
					  github-extensions-kb.pl

EXTRA_DIST = transform.py languages.yml heuristics.yml

file-expert$(EXEEXT): $(file_expert_SOURCES)
	swipl -O -o file-expert --goal=main -c run.pl github-extensions-kb.pl extra-extensions-kb.pl load.pl

github-extensions-kb.pl: transform.py languages.yml heuristics.yml
	python3 $^ > $@

clean-local:
	-rm -f github-extensions-kb.pl


.PHONY: distclean-generated
distclean-generated:
	git clean -fdx --quiet

distclean: distclean-generated

test: debug.pl $(file_expert_SOURCES)
	swipl -s debug.pl -g 'run_tests, halt(0); halt(1)'

save-skipped:
	swipl -s debug.pl -g 'generate_skipped, halt(0); halt(1).'|sort > skipped.pl

@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
