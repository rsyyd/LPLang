"""C code generator for LPLang (Stage 0 spike).

Emits C99 from a subset of the AST:
- int/bool/string literals, let/var, arithmetic/comparison/logic ops
- if/else, while, return
- function declarations and calls
- println (via printf), assert (via assert.h)

NOT yet emitted (falls back to interpreter): enums, match, structs,
lists, modules. lp build reports which features blocked compilation.

Design: one C source file per LPLang file; user functions become C
functions with a generated main().
"""

from .ast import (
    IntLit, FloatLit, StrLit, BoolLit, Ident, BinOp, UnaryOp, Call,
    LetStmt, ReturnStmt, IfStmt, WhileStmt, ExprStmt, FnDecl,
)


class CodegenError(Exception):
    def __init__(self, message, node=None):
        super().__init__(message)
        self.message = message
        self.node = node


class CGenerator:
    def __init__(self):
        self.lines = []
        self.indent = 1
        self.emitted_fns = set()
        # C reserved words that can't be used as function names
        self.c_keywords = {
            "auto", "break", "case", "char", "const", "continue", "default",
            "do", "double", "else", "enum", "extern", "float", "for", "goto",
            "if", "inline", "int", "long", "register", "restrict", "return",
            "short", "signed", "sizeof", "static", "struct", "switch",
            "typedef", "union", "unsigned", "void", "volatile", "while",
            "_Alignas", "_Alignof", "_Atomic", "_Bool", "_Complex", "_Generic",
            "_Imaginary", "_Noreturn", "_Static_assert", "_Thread_local",
        }

    def _mangle(self, name):
        if name in self.c_keywords:
            return f"lplang_{name}"
        return name

    # -- helpers --

    def emit(self, text):
        self.lines.append("    " * self.indent + text)

    def emit_raw(self, text):
        self.lines.append(text)

    # -- program --

    def generate(self, program):
        self.emit_raw("#include <stdio.h>")
        self.emit_raw("#include <stdbool.h>")
        self.emit_raw("#include <assert.h>")
        self.emit_raw("")

        fn_decls = []
        other_stmts = []
        for stmt in program.statements:
            kind = stmt.__class__.__name__
            if kind == "FnDecl":
                fn_decls.append(stmt)
            elif kind in ("StructDecl", "EnumDecl", "ImportStmtStub"):
                raise CodegenError(
                    f"'{kind}' is not supported by the native backend yet", stmt)
            else:
                other_stmts.append(stmt)

        # Forward declarations
        for fn in fn_decls:
            self.emit_raw(self._fn_signature(fn) + ";")
        if fn_decls:
            self.emit_raw("")

        # Main statements first (so top-level code runs before main uses nothing weird)
        main_body = []
        for stmt in other_stmts:
            main_body.append(self.gen_stmt(stmt))

        self.emit_raw("int main(void) {")
        self.indent = 1
        for line in main_body:
            for sub in line.split("\n"):
                self.emit(sub)
        self.emit("return 0;")
        self.indent = 0
        self.emit_raw("}")
        self.emit_raw("")

        # Function definitions
        for fn in fn_decls:
            self.gen_fn(fn)

        return "\n".join(self.lines) + "\n"

    # -- functions --

    def _fn_signature(self, fn):
        ret = "int"  # default; refined by return type below
        if fn.ret_type == "int":
            ret = "int"
        elif fn.ret_type == "bool":
            ret = "bool"
        elif fn.ret_type == "float":
            ret = "double"
        elif fn.ret_type == "string":
            ret = "const char*"
        elif fn.ret_type is None:
            ret = "void"
        params = []
        for pname, ptype in fn.params:
            if ptype == "int":
                params.append(f"int {pname}")
            elif ptype == "bool":
                params.append(f"bool {pname}")
            elif ptype == "float":
                params.append(f"double {pname}")
            elif ptype == "string":
                params.append(f"const char* {pname}")
            else:
                raise CodegenError(
                    f"parameter type '{ptype}' not supported by native backend", fn)
        return f"{ret} {self._mangle(fn.name)}({', '.join(params)})"

    def gen_fn(self, fn):
        self.emit_raw(self._fn_signature(fn) + " {")
        self.indent = 1
        for stmt in fn.body:
            for line in self.gen_stmt(stmt).split("\n"):
                self.emit(line)
        self.indent = 0
        self.emit_raw("}")
        self.emit_raw("")

    # -- statements --

    def gen_stmt(self, stmt):
        kind = stmt.__class__.__name__

        if kind == "LetStmt":
            ctype = "int"
            if stmt.type_ann == "bool":
                ctype = "bool"
            elif stmt.type_ann == "float":
                ctype = "double"
            elif stmt.type_ann == "string":
                ctype = "const char*"
            val = self.gen_expr(stmt.value)
            return f"{ctype} {self._mangle(stmt.name)} = {val};"

        elif kind == "ExprStmt":
            return self.gen_expr(stmt.expr) + ";"

        elif kind == "ReturnStmt":
            if stmt.value is None:
                return "return;"
            return f"return {self.gen_expr(stmt.value)};"

        elif kind == "IfStmt":
            cond = self.gen_expr(stmt.cond)
            lines = [f"if ({cond}) {{"]
            for s in stmt.then_body:
                lines.append(self.gen_stmt(s))
            lines.append("}")
            if stmt.else_body:
                lines.append("else {")
                for s in stmt.else_body:
                    lines.append(self.gen_stmt(s))
                lines.append("}")
            return "\n".join(lines)

        elif kind == "WhileStmt":
            cond = self.gen_expr(stmt.cond)
            lines = [f"while ({cond}) {{"]
            for s in stmt.body:
                lines.append(self.gen_stmt(s))
            lines.append("}")
            return "\n".join(lines)

        raise CodegenError(f"'{kind}' is not supported by the native backend yet", stmt)

    # -- expressions --

    def gen_expr(self, expr):
        kind = expr.__class__.__name__

        if kind == "IntLit":
            return str(expr.value)
        if kind == "FloatLit":
            return repr(expr.value)
        if kind == "BoolLit":
            return "true" if expr.value else "false"
        if kind == "StrLit":
            escaped = expr.value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n").replace("\t", "\\t")
            return f'"{escaped}"'
        if kind == "Ident":
            return self._mangle(expr.name)

        if kind == "UnaryOp":
            return f"({expr.op}{self.gen_expr(expr.operand)})"

        if kind == "BinOp":
            op = expr.op
            if op == "and":
                op = "&&"
            elif op == "or":
                op = "||"
            elif op == "not":
                op = "!"
            return f"({self.gen_expr(expr.left)} {op} {self.gen_expr(expr.right)})"

        if kind == "Call":
            fname = expr.func.name if expr.func.__class__.__name__ == "Ident" else None
            if fname == "println":
                return self._gen_printf(expr.args, newline=True)
            if fname == "print":
                return self._gen_printf(expr.args, newline=False)
            if fname == "assert":
                cond = self.gen_expr(expr.args[0])
                if len(expr.args) > 1:
                    msg = self.gen_expr(expr.args[1])
                    return f'(assert({cond}), (void)({msg}), true)'
                return f"assert({cond}), true"
            args = ", ".join(self.gen_expr(a) for a in expr.args)
            return f"{self._mangle(fname)}({args})"

        raise CodegenError(f"'{kind}' is not supported by the native backend yet", expr)

    def _gen_printf(self, args, newline):
        fmt_parts = []
        c_args = []
        for a in args:
            ak = a.__class__.__name__
            if ak == "StrLit":
                fmt_parts.append(a.value.replace("%", "%%"))
            elif ak == "IntLit" or ak == "Call" or ak == "Ident" or ak == "BinOp":
                # ints: %d, others unknown -> cast to int via %d for Ident
                fmt_parts.append("%d")
                c_args.append(self.gen_expr(a))
            elif ak == "BoolLit":
                fmt_parts.append("%s")
                c_args.append('true' if a.value else 'false')
            elif ak == "FloatLit":
                fmt_parts.append("%g")
                c_args.append(self.gen_expr(a))
            else:
                raise CodegenError(f"cannot print '{ak}' in native backend yet", a)
        fmt = " ".join(fmt_parts) + ("\\n" if newline else "")
        if c_args:
            return f'printf("{fmt}", {", ".join(c_args)});'
        return f'printf("{fmt}");'
