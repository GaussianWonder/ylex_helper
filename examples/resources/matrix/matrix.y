%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 10000

typedef struct _line{
  int elems[MAX];
  int no_columns_used;
} line;
typedef struct _matr {
  line *rows[MAX];
  int no_rows_used;
} matr;

static int variables[26];
matr *mem[26];

void yyerror(char const *s){fprintf(stderr, "%s\n", s);}
int main(){yyparse();}

void free_matr(matr *m) {
  printf("Free matrix at %p\n", m);
  if (m == NULL) return;
  for (int i=0; i<m->no_rows_used; i++) {
    free(m->rows[i]);
  }
  free(m);
  printf("\n");
}

void print_matr(matr *matrix) {
  if (matrix == NULL) {
    printf("NULL matrix at %p\n", matrix);
    return;
  }

  printf("Matrix at %p:\n", matrix);
  for (int i=0; i<matrix->no_rows_used; i++) {
    for (int j=0; j<matrix->rows[i]->no_columns_used; j++) {
      printf("%d ", matrix->rows[i]->elems[j]);
    }
    printf("\n");
  }
  printf("\n");
}

line* add_row_elem(line *ln, int ival) {
  printf("Adding element %d to line %p\n", ival, ln);
  ln->elems[ln->no_columns_used++] = ival;
  printf("\n");
  return ln;
}

line* create_row_elem(int ival) {
  line* ln = (line*) malloc(sizeof(line));
  printf("Creating line %p containing 1 element: %d\n", ln, ival);
  ln->no_columns_used = 1;
  ln->elems[0] = ival;
  printf("\n");
  return ln;
}

matr* create_matrix_from_row(line *ln) {
  matr* m = (matr*) malloc(sizeof(matr));
  printf("Creating matrix %p from row %p\n", m, ln);
  if (ln != NULL) {
    m->no_rows_used = 1;
    m->rows[0] = ln;
  } else {
    m->no_rows_used = 0;
  }
  printf("\n");
  return m;
}

matr* add_row_to_matrix(matr* m, line *ln) {
  printf("Adding row %p to matrix %p\n", ln, m);
  m->rows[m->no_rows_used++] = ln;
  printf("\n");
  return m;
}

int maxOf(int a, int b) {
  return a > b ? a : b;
}

line* copy_row(line *ln) {
  if (ln == NULL) return NULL;
  line* new_ln = (line*) malloc(sizeof(line));
  printf("Copying line %p to line %p\n", ln, new_ln);
  new_ln->no_columns_used = ln->no_columns_used;
  for (int i=0; i<ln->no_columns_used; i++) {
    new_ln->elems[i] = ln->elems[i];
  }
  printf("\n");
  return new_ln;
}

matr* add_matrices(matr* a, matr* b) {
  if (a == NULL || b == NULL) {
    printf("Cannot add NULL matrices\n");
    return NULL;
  }
  if (a->no_rows_used != b->no_rows_used) {
    printf("Cannot add matrices with different number of rows\n");
    return NULL;
  }

  printf("Adding matrices %p and %p\n", a, b);
  matr* m = create_matrix_from_row(NULL);
  for (int i=0; i<a->no_rows_used; i++) {
    line* new_ln = copy_row(a->rows[i]);
    for (int j=0; j<b->rows[i]->no_columns_used; j++) {
      new_ln->elems[j] += b->rows[i]->elems[j];
    }
    m = add_row_to_matrix(m, new_ln);
  }
  print_matr(m);
  // TODO free a and b matrices here
  printf("\n");
  return m;
}

matr* sub_matrices(matr* a, matr* b) {
  if (a == NULL || b == NULL) {
    printf("Cannot add NULL matrices\n");
    return NULL;
  }
  if (a->no_rows_used != b->no_rows_used) {
    printf("Cannot add matrices with different number of rows\n");
    return NULL;
  }

  printf("Adding matrices %p and %p\n", a, b);
  matr* m = create_matrix_from_row(NULL);
  for (int i=0; i<a->no_rows_used; i++) {
    line* new_ln = copy_row(a->rows[i]);
    for (int j=0; j<b->rows[i]->no_columns_used; j++) {
      new_ln->elems[j] -= b->rows[i]->elems[j];
    }
    m = add_row_to_matrix(m, new_ln);
  }
  print_matr(m);
  // TODO free a and b matrices here
  printf("\n");
  return m;
}

matr* copy_matrix(matr *m) {
  printf("Copying matrix %p to matrix %p\n", m, m);
  if (m == NULL) return NULL;
  matr *new_m = create_matrix_from_row(NULL);
  for (int i=0; i<m->no_rows_used; i++) {
    new_m = add_row_to_matrix(new_m, copy_row(m->rows[i]));
  }
  printf("\n");
  return new_m;
}
%}

%union {
  struct _matr *mat;
  struct _line *lin;
  int ival;
  char *str;
}

%type <mat> matrix
%type <lin> row
%type <str> expr
%token <ival> INTEGER VARIABLE
%left '+' '-'
%left '*' '/'

%%

program : program stmt '\n'
        | program '\n'		                  { yyerrok; }
        | /* NULL */
	      ;

stmt  : VARIABLE '=' matrix ';'             { printf("Var %d = %p\n", $1, $3); mem[$1] = $3; print_matr($3); }
      | VARIABLE '=' VARIABLE ';'               { printf("Var %d = %p\n", $1, mem[$3]); mem[$1] = copy_matrix(mem[$3]); print_matr(mem[$1]); }
      | VARIABLE '=' expr ';'               { printf("Var %d = %p\n", $1, $3); mem[$1] = $3; print_matr($3); }
      | expr ';'
      ;

expr  : expr '+' expr                       {{ $$ = add_matrices($1, $3); }}
      | expr '-' expr                       {{ $$ = sub_matrices($1, $3); }}
      | VARIABLE                            { $$ = mem[$1]; }
      ;

matrix  : matrix '\n' row                   { $$ = add_row_to_matrix($1, $3); }
        | row                               { $$ = create_matrix_from_row($1); }
        ;

row : row INTEGER                           { $$ = add_row_elem($1, $2); }
    | INTEGER                               { $$ = create_row_elem($1); }
    ;
