all:	lex yacc compile

lex:	matrix.l
	lex matrix.l

yacc:	matrix.y
	yacc -d matrix.y

compile: y.tab.c lex.yy.c
	gcc -o matrix lex.yy.c y.tab.c -ly -lfl

clean:
	rm -rf y.tab.c y.tab.h lex.yy.c matrix
