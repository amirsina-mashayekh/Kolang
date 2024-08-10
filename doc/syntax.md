# Kolang syntax
Here is EBNF definition of Kolang syntax.

``` ebnf
(* Program Structure *)
prog                = { func_def } ;
func_def            = "fn" ident "(" [ param_list ] ")" [ ":" type ] stmt ;

(* Parameters *)
param_list          = typed_ident { "," typed_ident } [ "," ] ;

(* Statements *)
stmt                = let_stmt
                    | expr_stmt
                    | if_stmt
                    | while_stmt
                    | for_stmt
                    | return_stmt
                    | block_stmt ;

let_stmt            = "let" typed_ident [ "=" expr ] ";" ;
expr_stmt           = expr ";" ;
if_stmt             = "if" expr stmt [ "else" stmt ] ;
while_stmt          = "while" expr stmt ;
for_stmt            = "for" ident "=" expr "to" expr stmt ;
return_stmt         = "return" [ expr ] ";" ;
block_stmt          = "{" { stmt } "}" ;

(* Expressions *)
expr                = assign_expr ;
assign_expr         = ( ident "=" expr ) | log_or_expr ;
log_or_expr         = log_and_expr { "or" log_and_expr } ;
log_and_expr        = eq_expr { "and" eq_expr } ;
eq_expr             = rel_expr { ( "==" | "!=" ) rel_expr } ;
rel_expr            = add_expr { ( "<" | ">" | "<=" | ">=" ) add_expr } ;
add_expr            = mul_expr { ( "+" | "-" ) mul_expr } ;
mul_expr            = unary_expr { ( "*" | "/" | "%" ) unary_expr } ;
unary_expr          = [ ( "not" | "~" | "-" ) ] primary_expr ;
primary_expr        = ident
                    | lit
                    | func_call
                    | array_index
                    | "(" expr ")" ;

(* Array Indexing *)
array_index         = ident "[" expr "]" ;

(* Function Calls *)
func_call           = ident "(" [ arg_list ] ")" ;
arg_list            = expr { "," expr } [ "," ] ;

(* Literals *)
lit                 = int_lit
                    | float_lit
                    | char_lit
                    | str_lit
                    | bool_lit
                    | array_lit ;

int_lit             = dec_lit | bin_lit | oct_lit | hex_lit ;
dec_lit             = digit { digit } ;
bin_lit             = ( "0b" | "0B" ) bin_digit { bin_digit } ;
oct_lit             = ( "0o" | "0O" ) oct_digit { oct_digit } ;
hex_lit             = ( "0x" | "0X" ) hex_digit { hex_digit } ;
float_lit           = ( digit { digit } "." digit { digit } | "." digit { digit } ) [ ( "e" | "E" ) [ "+" | "-" ] digit { digit } ] ;
char_lit            = "'" ( char | esc_seq ) "'" ;
str_lit             = "\"" { char | esc_seq | newline } "\"" ;
bool_lit            = "true" | "false" ;
array_lit           = "[" [ expr { "," expr } [ "," ] ] "]" ;

(* Identifiers and Types *)
typed_ident         = ident ":" type ;
ident               = ( letter | "_" ) { letter | digit | "_" } ;
type                = base_type [ "[" [ int_lit ] "]" ] ;
base_type           = "int" | "float" | "char" | "bool" | "str" ;

(* Characters and Digits *)
letter              = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
                    | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
                    | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M"
                    | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" ;
digit               = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
bin_digit           = "0" | "1" ;
oct_digit           = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" ;
hex_digit           = digit | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" ;
char                = letter | digit | symbol ;
esc_seq             = "\\" ( "n" | "t" | "r" | "0" | "\\" | "'" | "\"" ) ;
symbol              = "+" | "-" | "*" | "/" | "%" | "=" | "<" | ">" | "!" | "&" | "|" | "^" | "~" | "?" | ":" | ";" | "," | "." | "(" | ")" | "[" | "]" | "{" | "}" ;
newline             = "\n" | "\r\n" ;
```