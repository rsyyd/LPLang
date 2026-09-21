import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter, RuntimeError


class TestEnums(unittest.TestCase):
    def compile(self, source):
        tokens, diags = tokenize(source)
        self.assertFalse(diags.has_errors(), f"Lexer error: {diags.render_all(source.splitlines())}")
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors(), f"Parser error: {diags.render_all(source.splitlines())}")
        tc = TypeChecker(diags)
        tc.check_program(ast)
        return ast, diags

    def test_enum_declaration_and_unit_variant(self):
        src = """
        enum Direction {
            North,
            South,
            East,
            West,
        }
        let d = Direction::North;
        assert(d == Direction::North, "unit variant equality");
        assert(d != Direction::South, "unit variant inequality");
        """
        ast, diags = self.compile(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))
        interp = Interpreter()
        interp.eval_program(ast)

    def test_payload_variant_construction_and_match(self):
        src = """
        enum Shape {
            Circle(int),
            Rectangle(int, int),
            Point,
        }
        let s = Shape::Circle(10);
        match s {
            Circle(r) => { assert(r == 10, "circle radius bound"); },
            Rectangle(w, h) => { assert(false, "should not match rectangle"); },
            Point => { assert(false, "should not match point"); },
        }
        """
        ast, diags = self.compile(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))
        interp = Interpreter()
        interp.eval_program(ast)

    def test_match_with_wildcard_and_literals(self):
        src = """
        enum State {
            Idle,
            Running,
            Stopped,
        }
        let s = State::Running;
        match s {
            Idle => { assert(false, "should not match idle"); },
            Running => { assert(true, "running matched"); },
            _ => { assert(false, "should not match wildcard"); },
        }
        """
        ast, diags = self.compile(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))
        interp = Interpreter()
        interp.eval_program(ast)

    def test_match_with_nested_payload_patterns(self):
        src = """
        enum Op {
            Add(int, int),
            Mul(int, int),
        }
        let expr = Op::Add(3, 4);
        match expr {
            Add(a, b) => { assert(a + b == 7, "add result"); },
            Mul(a, b) => { assert(a * b == 12, "mul result"); },
        }
        """
        ast, diags = self.compile(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))
        interp = Interpreter()
        interp.eval_program(ast)

    def test_enum_type_in_function_signature(self):
        src = """
        enum Option {
            Some(int),
            None,
        }
        fn unwrap_or(o: Option, default: int) -> int {
            match o {
                Some(v) => { return v; },
                None => { return default; },
            }
            return default;
        }
        let a = unwrap_or(Option::Some(42), 0);
        let b = unwrap_or(Option::None, 99);
        assert(a == 42, "some unwraps to 42");
        assert(b == 99, "none unwraps to default 99");
        """
        ast, diags = self.compile(src)
        self.assertFalse(diags.has_errors(), diags.render_all(src.splitlines()))
        interp = Interpreter()
        interp.eval_program(ast)

    def test_unknown_variant_diagnostic(self):
        src = """
        enum Color {
            Red,
            Green,
        }
        let c = Color::Blue;
        """
        ast, diags = self.compile(src)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("undefined" in d.message.lower() for d in diags.items))


if __name__ == "__main__":
    unittest.main()