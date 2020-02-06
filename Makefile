GRAMMAR_NAME = phttp.g4

compile-grammar: 
	rm -rf piper/aparser
	antlr4 -Dlanguage=Python3 -visitor -o piper/aparser $(GRAMMAR_NAME)
	# touch piper/aparser/__init__.py

extend-classpath:
	export CLASSPATH=".:$(shell cat $(shell which antlr4) | grep antlr-complete | cut -d" " -f3):$$CLASSPATH"

compile-for-grun:
	antlr4 phttp.g4 -o grun
