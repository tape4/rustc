# RustC

A learning‐by‐doing project  
**Simplified C compiler** implemented in Rust. Rather than supporting all of C, this tool focuses on a well‑defined subset that exercises key compiler components—lexing, parsing, AST construction, semantic checks, code generation.

---

## Implementation Scope

### Supported Subset

- **Basic types**  
  - `int` (32‑bit signed)  
  - `char` (8‑bit signed)  

- **Derived types**  
  - Single‑level pointers (`int*`, `char*`)  
  - Fixed‑size one‑dimensional arrays (`int a[10]`)  

- **Literals**  
  - Integer literals (decimal)  
  - Character literals (`'a'`, `'\t'`, `'\0'`, …)  

- **Identifiers & Keywords**  
  - User‑defined names (variables, functions, parameters)  
  - Keywords: `int`, `char`, `if` / `else`, `for`, `while`, `return`, `break`, `continue`  

- **Operators**  
  - Arithmetic: `+`, `-`, `*`, `/`, `%`  
  - Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`  
  - Logical: `!`, `&&`, `||`  
  - Bitwise: `&`, `|`  
  - Compound assignment: `+=`, `-=`  
  - Increment / decrement: `++`, `--`  
  - Assignment: `=`  

- **Control flow**  
  - Conditional: `if` / `else`  
  - Loops: `for`, `while`  
  - Loop control: `break`, `continue`  

- **Comments**  
  - Single‑line: `// …`  
  - Multi‑line: `/* … */`  

---

### Excluded C Features

- **Preprocessor**: `#include`, `#define`, etc. are ignored  
- **Complex types**: `struct`, `union`, `enum`  
- **Function pointers & varargs**  
- **Array initialization (outside declarations), dynamic memory (`malloc`/`free`)**  
- **Floating point**: `float`, `double`  
- **Storage classes & qualifiers**: `static`, `extern`, `const`, `volatile`  
- **Preprocessor macros & advanced preprocessing**  
