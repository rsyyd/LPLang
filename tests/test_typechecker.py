import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker


class TestTypeChecker(unittest.TestCase):
    def check(self, source):
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors(), f"Lexer error: {diags.render_all(source.splitlines())}")
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), f"Parser error: {diags.render_all(source.splitlines())}")
        tc = TypeChecker(diags)
        tc.check_program(ast)
        return diags

    def test_valid_types(self):
        src = """
        let a: int = 10;
        let b: float = 3.14;
        let c: string = "hello";
        let d: bool = true;
        fn add(x: int, y: int) -> int {
            return x + y;
        }
        let res: int = add(a, 5);
        """
        diags = self.check(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))

    def test_type_mismatch_in_let(self):
        src = 'let a: int = "wrong";'
        diags = self.check(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("cannot initialize 'int' variable 'a' with 'string' value" in d.message for d in diags.items))

    def test_cannot_assign_to_immutable_var(self):
        src = """
        let x = 10;
        x = 20;
        """
        diags = self.check(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("cannot reassign immutable variable 'x'" in d.message for d in diags.items))

    def test_wrong_argument_type_reported(self):
        src = """
        fn double(n: int) -> int {
            return n * 2;
        }
        let res = double("string_arg");
        """
        diags = self.check(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("expected 'int', got 'string'" in d.message for d in diags.items))

    def test_return_type_mismatch_reported(self):
        src = """
        fn compute() -> int {
            return "not an int";
        }
        """
        diags = self.check(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("return type mismatch: expected 'int', got 'string'" in d.message for d in diags.items))

    def test_condition_must_be_bool(self):
        src = """
        if 42 {
            let x = 1;
        }
        """
        diags = self.check(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("if condition must be bool, got 'int'" in d.message for d in diags.items))


if __name__ == "__main__":
    unittest.main()
