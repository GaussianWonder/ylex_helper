#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

extern void yyparse(void);
extern FILE *yyin;

int main(int argc, char **argv)
{
  if(argc < 2)
  {
    printf("Usage: main <filename> \n");
    exit(-1);
  }

  yyin = fopen(argv[1],"r");

  if(yyin==NULL)
  {
    printf("Could not open file !\n");
    exit(-1);
  }

  do
  {
    yyparse();
  } while(!feof(yyin));

  // yyparse();
}
