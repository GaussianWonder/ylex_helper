%{
#include "y.tab.h"
%}

%%

[A-Z]	{
		yylval.ival = *yytext - 'A';
		return VARIABLE;
	}

[a-z]	{
		yylval.ival = *yytext - 'a';
		return VARIABLE;
	}

[0-9]+	{
		yylval.ival = strtol(yytext, (char **)NULL, *yytext=='0' ? 8:10);
		return INTEGER;
	}

0x[0-9a-fA-F]+	{
		yylval.ival = strtol(yytext, (char **)NULL, 16);
		return INTEGER;
	}

[-()=+/*\n;]	return *yytext;

[ \t]+	        ;

.		yyerror("Unknown character");
