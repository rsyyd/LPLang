import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.diagnostics import DiagnosticBag
from compiler.interpreter import Interpreter

class TestForInLoop(unittest.TestCase):
    def check_and_run(self, source):
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors(), f"Lexer error: {diags.render_all(source.splitlines())}")
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), f"Parser error: {diags.render_all(source.splitlines())}")
        tc = TypeChecker(diags)
        tc.check_program(ast)
        self.assertFalse(diags.has_errors(), f"Type checker error: {diags.render_all(source.splitlines())}")
        interp = Interpreter()
        interp.eval_program(ast)
        return interp.output, diags

    def test_for_in_range(self):
        src = "for i in 0..2 { println(i); }"
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["0", "1", "2"])

    def test_for_in_list(self):
        src = "let items = [10, 20]; for x in items { println(x); }"
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["10", "20"])

    def test_for_in_type_error(self):
        src = "for i in 42 { println(i); }"
        tokens, diags = tokenize(src)
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        tc = TypeChecker(diags)
        tc.check_program(ast)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("for-in expects a list or range" in d.message for d in diags.items))

if __name__ == "__main__":
    unittest.main()