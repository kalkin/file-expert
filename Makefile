file-expert: file-expert.pl github-extensions-kb.pl
	swipl -o file-expert -g main -c file-expert.pl

github-extensions-kb.pl: languages.yml
	python3 transform.py languages.yml > github-extensions-kb.pl
