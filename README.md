# Rust Procedural Macros for Enums: BitOp and ArithOp

This Rust code defines two procedural macros—**BitOp** and **ArithOp**—that automatically generate operator implementations for enums. Below is a detailed breakdown of its key aspects.

## 1. Purpose and Overview

### Custom Derives
- **Usage:** The macros are applied with the annotations `#[derive(BitOp)]` and `#[derive(ArithOp)]` on enums.
- **Goal:** They extend enums by automatically implementing bitwise and arithmetic operations.

### Operator Implementations

- **BitOp:** Implements bitwise operators including:
  - `Not`
  - `BitAnd`
  - `BitOr`
  - `BitXor`
  - Shift operators: `Shl` and `Shr`

- **ArithOp:** Implements arithmetic operators including:
  - `Add`
  - `Sub`
  - `Mul`
  - `Div`
  - `Rem`

## 2. Input Validation and Setup

### Enum Check
- Both macros begin by verifying that the input type is an enum.
- If the input is not an enum, the macro panics with an error message (e.g., "`#[derive(BitOp)] is only defined for enums`").

### Repr Attribute Requirement
- The macros require that the enum is annotated with a `#[repr(...)]` attribute.
- They search through the enum's attributes for one with the identifier `"repr"`.
- If the attribute is not found, the macro panics (e.g., "`#[derive(BitOp)] not find repr attribute`").
- The value in the `repr` must be one of a specific set of integer types (such as `u8`, `i32`, etc.).
- If the provided type is unacceptable, the macro panics with a message like "`invalid repr attribute for #[derive(BitOp)]`" (or a corresponding message for `ArithOp`).

### Extracting Enum Name and Representation Type
- The enum’s name and its underlying integer type (specified in the `repr` attribute) are extracted as strings.
- These values are later used to generate the operator implementations.

## 3. Code Generation via String Manipulation

### Template Strings
- The macros define code templates as strings that include placeholders (e.g., `@enum_name`, `@int_type`, `@trait_name`, etc.).
- **For Bitwise Operations:**
  - Separate templates are defined for:
    - `Not`
    - Binary bitwise operators (`BitAnd`, `BitOr`, `BitXor`)
    - Shift operators (`Shl` and `Shr`)
- **For Arithmetic Operations:**
  - The template covers:
    - `Add`
    - `Sub`
    - `Mul`
    - `Div`
    - `Rem`

### Replacing Placeholders
- The code uses string replacement to substitute placeholders with actual values:
  - The enum’s name and integer type replace `@enum_name` and `@int_type`.
  - Operator-specific tokens (like `&`, `|`, `^`, `<<`, `>>`, `+`, `-`, etc.) replace the placeholder `@op`.
  - Trait names (e.g., `"BitAnd"`, `"Add"`) and corresponding method names (e.g., `"bitand"`, `"add"`) are also substituted.

### Handling Multiple Shift Types
- For shift operators (`Shl` and `Shr`), the code iterates over an array of integer types.
- This ensures that implementations are generated for each possible shift type (e.g., `u8`, `u16`, …, `isize`).

### Concatenation and TokenStream Conversion
- After generating all the individual operator implementations, the resulting strings are concatenated.
- The concatenated string is then parsed into a `TokenStream` using `TokenStream::from_str` and returned as the macro's output.

## 4. Design Choices and Considerations

### String-Based Code Generation
- Instead of using typical approaches like the `quote!` macro for direct Rust code generation, this code uses string manipulation.
- While functional, this method is less robust and can be more error-prone compared to using structured macros.

### Underlying Type Conversions
- All operator implementations cast the enum to its underlying integer type (as specified by the `repr` attribute) before performing operations.
- Consequently, the operations return the underlying integer type rather than a value of the enum type.

### Compile-Time Safety
- The macros include compile-time checks to ensure:
  - They are applied only to enums.
  - The enum has a valid `repr` attribute.
  - The provided integer type is among the accepted types.
- These validations help catch potential misuse during compilation.

## Summary

This code automates the implementation of a suite of bitwise and arithmetic operations for enums with a defined integer representation. By enforcing specific requirements (i.e., the enum must have a valid `repr` attribute) and generating operator trait implementations via string replacement, the macros enable developers to write cleaner and more expressive code without manually implementing each operator trait.

