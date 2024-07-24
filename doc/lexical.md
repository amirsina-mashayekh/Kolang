# Kolang Tokens

Here is a brief list of Kolang tokens.

|Token              |Description                            |Regex pattern                      |
|-------------------|---------------------------------------|-----------------------------------|
|iden               |variable name, function name, etc.     |`[A-Za-z_][A-Za-z1-9_]*`           |
|literal_int_dec    |decimal integer literal: 123, 0, etc.  |`[0-9]+`                           |
|literal_int_bin    |binary integer literal: 0b1101, etc.   |`0[bB][01]+`                       |
|literal_int_oct    |octal integer literal: 0o7231, etc.    |`0[oO][0-7]+`                      |
|literal_int_hex    |hexadecimal integer literal: 0xff, etc.|`0[xX][0-9a-fA-F]+`                |
|literal_char       |character literal: 'a', '\0', etc.     |`'(\\.\|[^'\\])'`                  |
|literal_float      |floating-point literal: 9.1,e3, etc.   |`[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?` |
|literal_str        |string literal: "Hello\tworld!"        |`"(\\.\|[^"\\])*"`                 |
|lpar               |left parenthesis                       |`(`                                |
|rpar               |right parenthesis                      |`)`                                |
|lbracket           |left bracket                           |`[`                                |
|rbracket           |right bracket                          |`]`                                |
|lbrace             |left curly bracket                     |`{`                                |
|rbrace             |right curly bracket                    |`}`                                |
|lt                 |less than                              |`<`                                |
|gt                 |greater than                           |`>`                                |
|leq                |less than or equal                     |`<=`                               |
|geq                |greater than or equal                  |`>=`                               |
|eq                 |equals                                 |`==`                               |
|neq                |not equal                              |`!=`                               |
|assign             |assignment                             |`=`                                |
|plus               |plus sign                              |`+`                                |
|minus              |minus sign                             |`-`                                |
|asterisk           |asterisk                               |`*`                                |
|slash              |slash                                  |`/`                                |
|percent            |percent                                |`%`                                |
|pipe               |pipe (bitwise or)                      |`\|`                               |
|amp                |ampersand (bitwise and)                |`&`                                |
|tilde              |tilde (bitwise not)                    |`~`                                |
|semicolon          |statement terminator                   |`;`                                |
|colon              |colon                                  |`:`                                |
|coma               |coma                                   |`,`                                |
|period             |period                                 |`.`                                |
|lc                 |line comment                           |`//.*`                             |
|bc                 |block comment (not nested)             |`/\*[^*]*\*/`                      |
|kw_for             |`for` keyword (loop)                   |`for`                              |
|kw_to              |`to` keyword (loop range)              |`to`                               |
|kw_while           |`while` keyword (loop)                 |`while`                            |
|kw_if              |`if` keyword (conditional)             |`if`                               |
|kw_else            |`else` keyword (conditional)           |`else`                             |
|kw_true            |`true` keyword (boolean)               |`true`                             |
|kw_false           |`false` keyword (boolean)              |`false`                            |
|kw_or              |`or` keyword (logical)                 |`or`                               |
|kw_and             |`and` keyword (logical)                |`and`                              |
|kw_not             |`not` keyword (logical)                |`not`                              |
|kw_let             |`let` keyword (variable def.)          |`let`                              |
|kw_fn              |`fn` keyword (function def.)           |`fn`                               |
|kw_int             |`int` keyword (integer type)           |`int`                              |
|kw_char            |`char` keyword (character type)        |`char`                             |
|kw_bool            |`bool` keyword (boolean type)          |`bool`                             |
|kw_float           |`float` keyword (floating-point type)  |`float`                            |
|kw_str             |`str` keyword (string type)            |`str`                              |