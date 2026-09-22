import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter
from compiler.diagnostics import DiagnosticBag


class TestGenerics(unittest.TestCase):
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

    def test_identity_generic(self):
        src = """
        fn identity<T>(x: T) -> T {
            return x;
        }
        let a = identity(99);
        let b = identity("lumpo");
        assert(a == 99, "int identity");
        assert(b == "lumpo", "string identity");
        """
        self.run_source(src)

    def test_generic_two_params(self):
        src = """
        fn first<T>(a: T, b: T) -> T {
            return a;
        }
        let r = first(10, 20);
        assert(r == 10, "first returns first arg");
        """
        self.run_source(src)

    def test_generic_inferred_type_flows_to_checker(self):
        src = """
        fn wrap<T>(v: T) -> T {
            return v;
        }
        let x: int = wrap(42);
        assert(x == 42, "type inferred correctly");
        """
        self.run_source(src)

    def test_parsing_type_params(self):
        src = "fn id<A>(x: A) -> A { return x; }"
        tokens, diags = tokenize(src)
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors())
        fn = ast.statements[0]
        self.assertEqual(fn.type_params, ["A"])
        self.assertEqual(fn.params, [("x", "A")])
        self.assertEqual(fn.ret_type, "A")


if __name__ == "__main__":
    unittest.main()
