#!/bin/bash

echo "=== Testing Lexer Errors ==="
echo "1. Unterminated char literal:"
cargo run tests/fixtures/error_unterminated_char.c 2>&1 | head -5
echo

echo "2. Illegal character:"
cargo run tests/fixtures/error_invalid_token.c 2>&1 | head -5
echo

echo "=== Testing Parser Errors ==="
echo "3. Missing semicolon:"
cargo run tests/fixtures/error_missing_semicolon.c 2>&1 | head -5
echo

echo "=== Testing Semantic Errors ==="
echo "4. Undefined variable:"
cargo run tests/fixtures/error_undefined_var.c 2>&1 | head -5
echo

echo "5. Type mismatch:"
cargo run tests/fixtures/error_type_mismatch.c 2>&1 | head -5
echo

echo "6. Break outside loop:"
cargo run tests/fixtures/error_break_outside_loop.c 2>&1 | head -5
echo

echo "7. Duplicate declaration:"
cargo run tests/fixtures/error_duplicate_declaration.c 2>&1 | head -5
echo

echo "=== Testing Successful Compilation ==="
echo "8. Valid program (should output IR):"
cargo run tests/fixtures/error_success.c 2>&1 | head -10