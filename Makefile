file-expert: file-expert.pl github-extensions-kb.pl
	swipl -o file-expert -g main -c file-expert.pl

github-extensions-kb.pl: languages.yml
	python3 transform.py languages.yml > github-extensions-kb.pl

check: file-expert linguist/samples
	./test

linguist/samples:
	git submodule update --depth=1 --init linguist/

clean:
	rm -f github-extensions-kb.pl
	rm -f file-expert

distclean: clean
	git submodule deinit linguist/
