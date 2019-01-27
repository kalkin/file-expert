file-expert: file-expert.pl github-extensions-kb.pl extra-extensions-kb.pl
	swipl -o file-expert -g main -c file-expert.pl github-extensions-kb.pl extra-extensions-kb.pl

github-extensions-kb.pl: transform.py languages.yml
	@echo ":- multifile file_extension/3." > github-extensions-kb.pl
	@echo ":- multifile file_extension/2." >> github-extensions-kb.pl
	@echo ":- multifile filename/2." >> github-extensions-kb.pl
	@echo ":- multifile interpreter/2." >> github-extensions-kb.pl
	python3 transform.py languages.yml >> github-extensions-kb.pl

check: file-expert linguist/samples
	./test

linguist/samples:
	git submodule update --init linguist

clean:
	rm -f github-extensions-kb.pl
	rm -f file-expert

distclean: clean
	git submodule deinit linguist/

.PHONY: test
test: file-expert
	./test
