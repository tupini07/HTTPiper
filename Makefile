GRAMMAR_NAME = phttp.g4

compile-grammar: 
	rm -rf vendor/parser
	antlr4 -Dlanguage=Python3 -visitor -o parser $(GRAMMAR_NAME)

	

extend-classpath:
	export CLASSPATH=".:$(shell cat $(shell which antlr4) | grep antlr-complete | cut -d" " -f3):$$CLASSPATH"

compile-for-grun:
	antlr4 phttp.g4 -o grun
