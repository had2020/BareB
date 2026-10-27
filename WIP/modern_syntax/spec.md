# Language Grammar Specification

This document provides a formal, human-readable specification of the programming language grammar, covering lexical structure, types, expressions, statements, and top-level declarations.

---

## 1. Lexical Rules

The lexical grammar defines terminal tokens recognized by the scanner/lexer.

```ebnf
Letter          = "a" .. "z" | "A" .. "Z" | "_" ;
Digit           = "0" .. "9" ;

Keyword         = "fun" | "let" | "if" | "else" | "while" | "return" | "uint" | "bool" ;

Identifier      = ( Letter , { Letter | Digit } ) - Keyword ;
IntegerLiteral  = Digit , { Digit } ;
BooleanLiteral  = "true" | "false" ;
```

### Summary Table

| Token | Pattern / Values | Description |
| :--- | :--- | :--- |
| `Letter` | `[a-zA-Z_]` | Alphabetic character or underscore |
| `Digit` | `[0-9]` | Decimal digits |
| `Keyword` | `fun`, `let`, `if`, `else`, `while`, `return`, `uint`, `bool` | Reserved language keywords |
| `Identifier` | `(Letter (Letter \| Digit)*) \ Keyword` | User-defined identifiers |
| `IntegerLiteral` | `[0-9]+` | Unsigned decimal integer literals |
| `BooleanLiteral` | `true`, `false` | Boolean literal values |

---

## 2. Type System

Types specify primitives, pointers, fixed/dynamic array types, or user-defined aliases.

```ebnf
Type          = PrimitiveType | PointerType | ArrayType | Identifier ;

PrimitiveType = "uint" | "bool" ;
PointerType   = "*" , [ "mut" ] , Type ;
ArrayType     = "[" , Type , [ ";" , IntegerLiteral ] , "]" ;
```

* **Primitives:** `uint`, `bool`
* **Pointers:**
  * Immutable pointer: `*T`
  * Mutable pointer: `*mut T`
* **Arrays / Buffers:**
  * Unsized / slice view: `[T]`
  * Fixed-length array: `[T; N]`

---

## 3. Expressions & Operator Precedence

Expressions are parsed according to standard operator precedence, evaluated from lowest precedence (Level 1) to highest precedence (Level 12).

| Level | Operator Category | Operators | Associativity | Grammar Production |
| :---: | :--- | :--- | :---: | :--- |
| **1** | Logical OR | `\|\|` | Left | `LogicalOrExpr = LogicalAndExpr { "\|\|" LogicalAndExpr }` |
| **2** | Logical AND | `&&` | Left | `LogicalAndExpr = BitwiseOrExpr { "&&" BitwiseOrExpr }` |
| **3** | Bitwise OR | `\|` | Left | `BitwiseOrExpr = BitwiseXorExpr { "\|" BitwiseXorExpr }` |
| **4** | Bitwise XOR | `^` | Left | `BitwiseXorExpr = BitwiseAndExpr { "^" BitwiseAndExpr }` |
| **5** | Bitwise AND | `&` | Left | `BitwiseAndExpr = EqualityExpr { "&" EqualityExpr }` |
| **6** | Equality | `==`, `!=` | Left | `EqualityExpr = RelationalExpr { ("==" \| "!=") RelationalExpr }` |
| **7** | Relational | `<`, `<=`, `>`, `>=` | Left | `RelationalExpr = ShiftExpr { ("<" \| "<=" \| ">" \| ">=") ShiftExpr }` |
| **8** | Bit Shift | `<<`, `>>` | Left | `ShiftExpr = AdditiveExpr { ("<<" \| ">>") AdditiveExpr }` |
| **9** | Additive | `+`, `-` | Left | `AdditiveExpr = MultiplicativeExpr { ("+" \| "-") MultiplicativeExpr }` |
| **10** | Multiplicative | `*`, `/`, `%` | Left | `MultiplicativeExpr = UnaryExpr { ("*" \| "/" \| "%") UnaryExpr }` |
| **11** | Unary Prefix | `&`, `*`, `-`, `!`, `~` | Right | `UnaryExpr = ("&" \| "*" \| "-" \| "!" \| "~") UnaryExpr \| PostfixExpr` |
| **12** | Postfix / Primary | `()`, `[]` | Left | `PostfixExpr = PrimaryExpr { CallSuffix \| IndexSuffix }` |

### Grammar Productions

```ebnf
Expression         = LogicalOrExpr ;

LogicalOrExpr      = LogicalAndExpr , { "||" , LogicalAndExpr } ;
LogicalAndExpr     = BitwiseOrExpr , { "&&" , BitwiseOrExpr } ;
BitwiseOrExpr      = BitwiseXorExpr , { "|" , BitwiseXorExpr } ;
BitwiseXorExpr     = BitwiseAndExpr , { "^" , BitwiseAndExpr } ;
BitwiseAndExpr     = EqualityExpr , { "&" , EqualityExpr } ;
EqualityExpr       = RelationalExpr , { ( "==" | "!=" ) , RelationalExpr } ;
RelationalExpr     = ShiftExpr , { ( "<" | "<=" | ">" | ">=" ) , ShiftExpr } ;
ShiftExpr          = AdditiveExpr , { ( "<<" | ">>" ) , AdditiveExpr } ;
AdditiveExpr       = MultiplicativeExpr , { ( "+" | "-" ) , MultiplicativeExpr } ;
MultiplicativeExpr = UnaryExpr , { ( "*" | "/" | "%" ) , UnaryExpr } ;

UnaryExpr          = ( "&" | "*" | "-" | "!" | "~" ) , UnaryExpr
                   | PostfixExpr ;

PostfixExpr        = PrimaryExpr , { CallSuffix | IndexSuffix } ;
CallSuffix         = "(" , [ ArgList ] , ")" ;
IndexSuffix        = "[" , Expression , "]" ;
ArgList            = Expression , { "," , Expression } ;

PrimaryExpr        = Identifier 
                   | IntegerLiteral 
                   | BooleanLiteral 
                   | "(" , Expression , ")" ;
```

---

## 4. Assignment Targets (L-Values)

An `LValue` represents an expression denoting a mutable memory location valid on the left-hand side of an assignment:

```ebnf
LValue = Identifier                        (* Variable *)
       | "*" , UnaryExpr                   (* Pointer dereference *)
       | PostfixExpr , "[" , Expression , "]" ; (* Array element indexing *)
```

---

## 5. Statements

Statements define units of execution. All non-block statements end with a semicolon `;`.

```ebnf
Statement  = VarDecl 
           | Assignment 
           | WhileStmt 
           | IfStmt 
           | ReturnStmt 
           | ExprStmt ;

VarDecl    = "let" , Identifier , ":" , Type , [ "=" , Expression ] , ";" ;
Assignment = LValue , "=" , Expression , ";" ;
ExprStmt   = Expression , ";" ;
WhileStmt  = "while" , Expression , Block ;
IfStmt     = "if" , Expression , Block , [ "else" , ( Block | IfStmt ) ] ;
ReturnStmt = "return" , [ Expression ] , ";" ;
```

---

## 6. Declarations & Program Structure

A complete translation unit consists of a sequence of function definitions.

```ebnf
Program      = { FunctionDecl } ;

FunctionDecl = "fun" , Identifier , "(" , [ ParamList ] , ")" , [ "->" , Type ] , Block ;
ParamList    = Param , { "," , Param } ;
Param        = Identifier , ":" , Type ;
Block        = "{" , { Statement } , "}" ;
```
