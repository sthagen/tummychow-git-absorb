build-manpage: git-absorb.1

git-absorb.1: git-absorb.adoc
	$(if $(shell command -v a2x),,$(error "No a2x in PATH; install asciidoc."))
	$(info Building manpage. This may take a few moments...)
	a2x -L -d manpage -f manpage git-absorb.adoc --attribute man-version=${GA_VERSION}

clean:
	rm -f git-absorb.1 git-absorb.xml
