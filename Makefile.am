bin_PROGRAMS = file-expert
file_expert_SOURCES = file-expert.pl \
					  extra-extensions-kb.pl \
					  github-extensions-kb.pl

EXTRA_DIST = transform.py languages.yml heuristics.yml

file-expert$(EXEEXT): $(file_expert_SOURCES)
	swipl -o file-expert -g main -c file-expert.pl github-extensions-kb.pl extra-extensions-kb.pl

github-extensions-kb.pl: transform.py languages.yml
	@echo ":- multifile file_extension/3." > github-extensions-kb.pl
	@echo ":- multifile file_extension/2." >> github-extensions-kb.pl
	@echo ":- multifile filename/2." >> github-extensions-kb.pl
	@echo ":- multifile interpreter/2." >> github-extensions-kb.pl
	python3 transform.py languages.yml >> github-extensions-kb.pl

clean-local:
	-rm -f github-extensions-kb.pl

distclean:
	@cat .gitignore|grep -v '\s*#'|grep .|sed 's#^/#./#'|xargs rm -frv
