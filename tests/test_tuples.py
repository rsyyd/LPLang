import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter

class TestTuples(unittest.TestCase):
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

    def test_tuple_literal_and_indexing(self):
        src = 'let t = (1, "two", 3.0); println(t[0]); println(t[1]);'
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["1", "two"])

    def test_empty_tuple(self):
        src = 'let t = (); println(t);'
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["()"])

    def test_nested_tuples(self):
        src = 'let t = ((1, 2), 3); let inner = t[0]; println(inner[1]);'
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["2"])

    def test_parenthesized_expr_not_tuple(self):
        # (1 + 2) is integer 3, not a tuple
        src = 'let x = (1 + 2) * 3; println(x);'
        output, diags = self.check_and_run(src)
        self.assertEqual(output, ["9"])

if __name__ == "__main__":
    unittest.main()
