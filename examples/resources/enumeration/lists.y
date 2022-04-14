%{
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string>
#include <iostream>
#include <vector>
#include <cstddef>

using namespace std;

extern int yylex();
void yyerror(char *s);

typedef struct _IARR {
  vector<int> elements;
} IARR;

void print_iarr(IARR* a)
{
  size_t len = a->elements.size();
  for (size_t i=0; i<len; ++i) {
    cout << a->elements[i] << ' ';
  }
  cout << '\n';
}

IARR* create_iarr(int element)
{
  IARR* a = new IARR;
  a->elements.resize(1);
  a->elements[0] = element;
  return a;
}
%}

%union {
  int ival;
  char *str;
  struct _IARR *iarr;
}

%token DROP TAKE CONCAT
%token <ival> VARIABLE INTEGER
%type <iarr> enum

%%

program   : program enum '\n'
          | program '\n'		                  { yyerrok; }
          | /* NULL */
          ;

expr      : expr CONCAT expr
          | TAKE expr INTEGER
          | DROP expr INTEGER
          | '(' expr ')'
          | '[' enum ']'
          ;

enum      : INTEGER ',' enum                  { $3->elements.push_back($1); $$ = $3; print_iarr($3); }
          | INTEGER                           { $$ = create_iarr($1); cout << "Created a new enum with integer: " << $1 << '\n';}
          ;

%%

void yyerror(char *s)
{
  extern int yylineno;
  extern char *yytext;
  printf("ERROR: %s on line %d\n", yytext, yylineno);
}
