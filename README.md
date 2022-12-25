# Lamb 🐑

## Introduce

This repo is a simple implement of untyped lambda calculus.

## EBNF

```ebnf
comp_unit  ::= {func_def}
number     ::= INT_VAL, {INT_VAL};
variable   ::= CHAR_VAL, {CHAR_VAL};
bin_op     ::= "+"  |  "-"  |  "*"  |  "/"  |  ">"  |  "<"
             | "==" |  "!=" |  ">=" |  "<=" ;
unary_op   ::= "!";
func_def   ::= variable "=" func;
func       ::= { "\" variable "->" } expr;
bin_expr   ::= bin_op expr
unary_expr ::= unary_op expr
expr       ::= (variable | number | paren_expr | unary_op) {(expr | bin_expr)};
paren_expr ::= "(" expr | func ")";
```