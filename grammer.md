# Grammar

```bnf
program               ::= function*  

function              ::= function_declaration
                        | function_definition

function_declaration  ::= type_specifier identifier "(" ( "void" | parameter_list )? ")" ";"
function_definition   ::= type_specifier identifier "(" ( "void" | parameter_list )? ")" block  

parameter_list        ::= parameter ( "," parameter )*  
parameter             ::= type_specifier identifier ( "[" int_literal? "]" )?  

type_specifier        ::= ( "int" | "char" | "void" ) "*"*  

block                 ::= "{" statement* "}"  

statement             ::= block  
                        | if_statement  
                        | while_statement  
                        | for_statement  
                        | return_statement  
                        | break_statement  
                        | continue_statement  
                        | declaration_statement  
                        | expression_statement  

declaration_statement ::= type_specifier init_declarator_list ";"  
init_declarator_list  ::= init_declarator ( "," init_declarator )*  
init_declarator       ::= declarator ( "=" initializer )?  
declarator            ::= identifier ( "[" int_literal "]" )?  

initializer           ::= expression  
                       | "{" initializer_list? "}"  
initializer_list      ::= initializer ( "," initializer )* ","?  

expression_statement  ::= expression? ";"  

if_statement          ::= "if" "(" expression ")" statement ( "else" statement )?  
while_statement       ::= "while" "(" expression ")" statement  
for_statement         ::= "for" "(" expression? ";" expression? ";" expression? ")" statement  
return_statement      ::= "return" expression? ";"  
break_statement       ::= "break" ";"  
continue_statement    ::= "continue" ";"  

expression            ::= assignment  
assignment            ::= logical_or ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" ) assignment )?  

logical_or            ::= logical_and ( "||" logical_and )*  
logical_and           ::= bitwise_or ( "&&" bitwise_or )*  
bitwise_or            ::= bitwise_xor ( "|" bitwise_xor )*  
bitwise_xor           ::= bitwise_and ( "^" bitwise_and )*  
bitwise_and           ::= equality ( "&" equality )*  

equality              ::= relational ( ( "==" | "!=" ) relational )*  
relational            ::= additive ( ( "<" | "<=" | ">" | ">=" ) additive )*  
additive              ::= multiplicative ( ( "+" | "-" ) multiplicative )*  
multiplicative        ::= unary ( ( "*" | "/" | "%" ) unary )*  

unary                 ::= ( "!" | "-" | "&" | "*" | "++" | "--" ) unary  
                        | postfix  
postfix               ::= primary postfix_op*  
postfix_op            ::= "(" argument_list? ")"  
                        | "[" expression "]"  
                        | "++"  
                        | "--"  

primary               ::= identifier  
                        | int_literal  
                        | char_literal  
                        | "(" expression ")"  
                        | "{" initializer_list? "}"  

argument_list         ::= expression ( "," expression )*  

identifier            ::= /* Ident(String) */  
int_literal           ::= /* IntLiteral(i64) */  
char_literal          ::= /* CharLiteral(char) */  