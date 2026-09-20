import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker


class TestStructs(unittest.TestCase):
    def check(self, source):
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors(), f"Lexer error: {diags.render_all(source.splitlines())}")
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), f"Parser error: {diags.render_all(source.splitlines())}")
        tc = TypeChecker(diags)
        tc.check_program(ast)
        return diags

    # This test will fail until struct syntax is implemented
    def test_struct_declaration_and_field_access(self):
        src = """
        struct Point {
            x: int,
            y: int,
        }

        fn make_point(x: int, y: int) -> Point {
            return Point { x: x, y: y };
        }

        fn distance_sq(p: Point) -> int {
            return p.x * p.x + p.y * p.y;
        }

        let origin = make_point(0, 0);
        let p = make_point(3, 4);
        assert(p.x == 3, "field access");
        assert(distance_sq(p) == 25, "distance squared");
        """
        diags = self.check(src)
        # This will fail initially - structural test for future implementation
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))


if __name__ == "__main__":
    unittest.main()