import unittest
import sys
import os
import subprocess
import tempfile

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from compiler.lexer import tokenize
from compiler.parser import Parser
from compiler.codegen_c import CGenerator, CodegenError


class TestNativeCodegen(unittest.TestCase):
    def test_c_emission_and_compile(self):
        src = """
        fn double(x: int) -> int {
            return x * 2;
        }
        let res = double(21);
        println("Result:", res);
        """
        tokens, diags = tokenize(src)
        self.assertFalse(diags.has_errors())
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        self.assertFalse(diags.has_errors())

        gen = CGenerator()
        c_code = gen.generate(ast)
        self.assertIn("int lplang_double(int x)", c_code)
        self.assertIn("int main(void)", c_code)

        # Try compiling with gcc if present
        with tempfile.TemporaryDirectory() as tmp:
            c_file = os.path.join(tmp, "out.c")
            bin_file = os.path.join(tmp, "out")
            with open(c_file, "w", encoding="utf-8") as f:
                f.write(c_code)
            res = subprocess.run(["gcc", "-O2", "-o", bin_file, c_file],
                                 capture_output=True, text=True)
            self.assertEqual(res.returncode, 0, f"gcc failed: {res.stderr}")
            run_res = subprocess.run([bin_file], capture_output=True, text=True)
            self.assertEqual(run_res.returncode, 0)
            self.assertIn("Result: 42", run_res.stdout)

    def test_unsupported_node_raises_error(self):
        src = """
        struct Point { x: int }
        """
        tokens, diags = tokenize(src)
        parser = Parser(tokens, diags)
        ast = parser.parse_program()
        gen = CGenerator()
        with self.assertRaises(CodegenError) as ctx:
            gen.generate(ast)
        self.assertIn("not supported by the native backend yet", str(ctx.exception))


if __name__ == "__main__":
    unittest.main()
