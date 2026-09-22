import unittest
import sys
import os
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.loader import load_program
from compiler.diagnostics import DiagnosticBag
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter, RuntimeError


class TestTryOperator(unittest.TestCase):
    def test_result_try_propagation(self):
        diags = DiagnosticBag()
        programs, _ = load_program("examples/09_try_operator.lp", diags)
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        tc = TypeChecker(diags)
        for _, p in programs:
            tc.check_program(p)
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        interp = Interpreter()
        for _, p in programs:
            interp.eval_program(p)

    def test_try_on_non_result_fails(self):
        src = """
        fn bad() -> int {
            let x = 42?;
            return x;
        }
        """
        import tempfile
        with tempfile.NamedTemporaryFile("w", suffix=".lp", delete=False) as f:
            f.write(src)
            path = f.name
        try:
            diags = DiagnosticBag()
            programs, _ = load_program(path, diags)
            tc = TypeChecker(diags)
            for _, p in programs:
                tc.check_program(p)
            self.assertTrue(diags.has_errors())
            self.assertTrue(any("operator '?' can only be used on Result or Option" in d.message for d in diags.items))
        finally:
            os.unlink(path)


if __name__ == "__main__":
    unittest.main()
