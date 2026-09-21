import unittest
import sys
import os
import tempfile
import shutil

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.loader import load_program
from compiler.diagnostics import DiagnosticBag
from compiler.typechecker import TypeChecker
from compiler.interpreter import Interpreter


class TestModules(unittest.TestCase):
    def setUp(self):
        self.test_dir = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.test_dir)

    def test_multi_file_import_and_execution(self):
        math_file = os.path.join(self.test_dir, "math_utils.lp")
        with open(math_file, "w", encoding="utf-8") as f:
            f.write("""
            fn square(x: int) -> int {
                return x * x;
            }
            """)

        main_file = os.path.join(self.test_dir, "main.lp")
        with open(main_file, "w", encoding="utf-8") as f:
            f.write("""
            import math_utils;

            let result = square(5);
            assert(result == 25, "square of 5 is 25");
            println("Module test OK:", result);
            """)

        diags = DiagnosticBag()
        programs, _ = load_program(main_file, diags)
        self.assertFalse(diags.has_errors(), [d.message for d in diags.items])
        self.assertEqual(len(programs), 2)

        tc = TypeChecker(diags)
        for _, prog in programs:
            tc.check_program(prog)
        self.assertFalse(diags.has_errors())

        interp = Interpreter()
        for _, prog in programs:
            interp.eval_program(prog)

        self.assertIn("Module test OK: 25", interp.output)

    def test_missing_module_diagnostic(self):
        main_file = os.path.join(self.test_dir, "broken.lp")
        with open(main_file, "w", encoding="utf-8") as f:
            f.write("import nonexistent_mod;\n")

        diags = DiagnosticBag()
        programs, _ = load_program(main_file, diags)
        self.assertTrue(diags.has_errors())
        self.assertTrue(any("module file not found" in d.message for d in diags.items))


if __name__ == "__main__":
    unittest.main()
