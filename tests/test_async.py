import unittest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter


class TestAsync(unittest.TestCase):
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

    def test_basic_async_await(self):
        src = """
        async fn compute(n: int) -> int {
            return n * 3;
        }
        let res = await compute(14);
        assert(res == 42, "computed 42");
        """
        interp = self.run_source(src)

    def test_spawn_concurrent_tasks(self):
        src = """
        async fn slow_val(v: int) -> int {
            sleep(10);
            return v;
        }
        let a = spawn slow_val(100);
        let b = spawn slow_val(200);
        let ra = await a;
        let rb = await b;
        assert(ra + rb == 300, "sum 300");
        """
        interp = self.run_source(src)

    def test_idempotent_await_on_primitive(self):
        src = """
        let x = await 42;
        assert(x == 42, "awaiting primitive returns value");
        """
        interp = self.run_source(src)


if __name__ == "__main__":
    unittest.main()
