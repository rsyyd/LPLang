import unittest
import sys
import os

# Include LPLang root directory in python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.interpreter import Interpreter, RuntimeError


class TestLexer(unittest.TestCase):
    def test_token_kinds(self):
        source = 'let x: int = 42; var y = "hello"; fn test() {}'
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors())
        kinds = [t.kind for t in tokens]
        self.assertIn("KEYWORD", kinds)
        self.assertIn("IDENT", kinds)
        self.assertIn("INT", kinds)
        self.assertIn("STRING", kinds)
        self.assertEqual(tokens[-1].kind, "EOF")

    def test_lexical_diagnostics_unterminated_string(self):
        source = 'let str = "unclosed'
        _, diags = tokenize(source)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("unterminated string literal" in d.message for d in diags.items))


class TestParserAndInterpreter(unittest.TestCase):
    def execute(self, source):
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors(), "Lexer reported unexpected errors")
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), "Parser reported unexpected errors")
        interp = Interpreter()
        interp.eval_program(ast)
        return interp

    def test_arithmetic_and_let(self):
        src = """
        let a = 10;
        let b = 20;
        let c = a + b * 2;
        assert(c == 50, "Precedence check");
        """
        self.execute(src)

    def test_immutable_reassignment_fails(self):
        src = """
        let a = 10;
        a = 20;
        """
        tokens, diags = tokenize(src)
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        interp = Interpreter()
        with self.assertRaises(RuntimeError) as ctx:
            interp.eval_program(ast)
        self.assertIn("cannot reassign immutable variable", str(ctx.exception))

    def test_mutable_reassignment_succeeds(self):
        src = """
        var x = 1;
        x = x + 5;
        assert(x == 6, "Mutation check");
        """
        self.execute(src)

    def test_function_and_recursion(self):
        src = """
        fn fib(n: int) -> int {
            if n <= 1 {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }
        assert(fib(7) == 13, "Fibonacci 7");
        """
        self.execute(src)


if __name__ == "__main__":
    unittest.main()
