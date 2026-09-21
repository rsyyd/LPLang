import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter, RuntimeError


class TestMatchExpr(unittest.TestCase):
    def run_source(self, src):
        tokens, diags = tokenize(src)
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        tc = TypeChecker(diags)
        tc.check_program(ast)
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        interp = Interpreter()
        interp.eval_program(ast)
        return interp

    def test_match_expr_with_bindings(self):
        src = """
        enum Shape {
            Circle(int),
            Rect(int, int),
        }
        fn area(s: Shape) -> int {
            return match s {
                Circle(r) => 3 * r * r,
                Rect(w, h) => w * h,
            };
        }
        assert(area(Shape::Circle(5)) == 75, "circle");
        assert(area(Shape::Rect(4, 6)) == 24, "rect");
        """
        self.run_source(src)

    def test_match_expr_with_wildcard(self):
        src = """
        let msg = match 42 {
            1 => "one",
            _ => "other",
        };
        assert(msg == "other", "wildcard arm");
        """
        self.run_source(src)

    def test_match_expr_no_match_is_runtime_error(self):
        src = """
        let v = match 99 {
            1 => "one",
            2 => "two",
        };
        """
        tokens, diags = tokenize(src)
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        tc = TypeChecker(diags)
        tc.check_program(ast)
        interp = Interpreter()
        with self.assertRaises(RuntimeError) as ctx:
            interp.eval_program(ast)
        self.assertIn("no pattern matched", str(ctx.exception))


if __name__ == "__main__":
    unittest.main()
