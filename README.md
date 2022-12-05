# Lamb 🐑

## Introduce

This repo is a simple implement of untyped lambda calculus.

## EBNF

```ebnf
number     ::= INT_VAL, {INT_VAL};
variable   ::= CHAR_VAL, {CHAR_VAL};
bin_op     ::= "+"  |  "-"  |  "*"  |  "/"  |  ">"  |  "<"
             | "==" |  "!=" |  ">=" |  "<=" ;
unary_op   ::= "!";
func       ::= "\" variable "->" (func | expr);
expr       ::= variable {bin_op variable} | paren_expr;
paren_expr ::= "(" expr ")";
```