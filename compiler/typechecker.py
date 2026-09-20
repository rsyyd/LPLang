"""Static type checker for LPLang (Stage 0 subset).

Checks primitive types: int, float, string, bool.

Semantics implemented:
- Variable declarations: inferred from initializer, or validated against
  explicit annotation. Conflict = error.
- Binary ops:
    int op int -> int            (+ - * / % comparisons)
    float involved (+ - * /) -> float
    + also allows string + string (concatenation)
    comparisons (== != < <= > >=) -> bool
    and/or/&&/|| require bool operands -> bool
- Integer division (/ on two ints) yields int; mixing int and float yields float.
- Function parameters and return types must be annotated to be checked;
  unannotated functions are checked structurally where possible but calls
  to them are not type-verified (gradual behavior, marked in spec as TBD).

Unknown types (not in PRIMITIVES) produce a diagnostic at declaration site.
"""

from .diagnostics import DiagnosticBag
from .ast import (
    FnDecl, LetStmt, ReturnStmt, IfStmt, WhileStmt, ExprStmt,
    IntLit, FloatLit, StrLit, BoolLit, Ident, BinOp, UnaryOp, Call, IfExpr
)

PRIMITIVES = {"int", "float", "string", "bool"}

# Map AST literal/expr class name -> static type
_LIT_TYPES = {
    "IntLit": "int",
    "FloatLit": "float",
    "StrLit": "string",
    "BoolLit": "bool",
}


class FnSignature:
    def __init__(self, params, ret_type):
        self.params = params        # list[(name, type)]
        self.ret_type = ret_type


class TypeChecker:
    def __init__(self, diags=None):
        self.diags = diags if diags is not None else DiagnosticBag()
        self.scopes = [{}]          # stack of name -> (type, mutable)
        self.functions = {}         # name -> FnSignature
        self.current_fn_ret = None  # expected return type of enclosing fn

    # -- scope helpers --

    def push(self):
        self.scopes.append({})

    def pop(self):
        self.scopes.pop()

    def declare(self, name, typ, mutable, line, col):
        scope = self.scopes[-1]
        if name in scope:
            self.diags.error(f"variable '{name}' is already declared in this scope",
                             line, col, hint="rename the variable or remove the duplicate declaration")
            return
        scope[name] = (typ, mutable)

    def lookup(self, name):
        for scope in reversed(self.scopes):
            if name in scope:
                return scope[name]
        return None

    # -- program --

    def check_program(self, program):
        # Pass 1: collect function signatures (allows forward references)
        for stmt in program.statements:
            if stmt.__class__.__name__ == "FnDecl":
                self._register_fn(stmt)
        # Pass 2: check bodies
        for stmt in program.statements:
            self.check_stmt(stmt)

    def _register_fn(self, fn):
        params = []
        for pname, ptype in fn.params:
            if ptype is None:
                self.diags.error(
                    f"parameter '{pname}' of function '{fn.name}' needs a type annotation",
                    fn.line, fn.col,
                    hint="write e.g. fn f(x: int) so calls can be type-checked")
                params.append((pname, None))
            elif ptype not in PRIMITIVES:
                self.diags.error(f"unknown type '{ptype}' for parameter '{pname}'",
                                 fn.line, fn.col,
                                 hint=f"supported primitive types: {', '.join(sorted(PRIMITIVES))}")
                params.append((pname, None))
            else:
                params.append((pname, ptype))

        if fn.ret_type is not None and fn.ret_type not in PRIMITIVES:
            self.diags.error(f"unknown return type '{fn.ret_type}' for function '{fn.name}'",
                             fn.line, fn.col,
                             hint=f"supported primitive types: {', '.join(sorted(PRIMITIVES))}")
            ret = None
        else:
            ret = fn.ret_type

        self.functions[fn.name] = FnSignature(params, ret)

    # -- statements --

    def check_stmt(self, stmt):
        kind = stmt.__class__.__name__

        if kind == "FnDecl":
            self._check_fn_body(stmt)
        elif kind == "LetStmt":
            self._check_let(stmt)
        elif kind == "ReturnStmt":
            self._check_return(stmt)
        elif kind == "IfStmt":
            cond_t = self.check_expr(stmt.cond)
            if cond_t is not None and cond_t != "bool":
                self.diags.error(f"if condition must be bool, got '{cond_t}'",
                                 stmt.line, stmt.col)
            self.push()
            for s in stmt.then_body:
                self.check_stmt(s)
            self.pop()
            if stmt.else_body:
                self.push()
                for s in stmt.else_body:
                    self.check_stmt(s)
                self.pop()
        elif kind == "WhileStmt":
            cond_t = self.check_expr(stmt.cond)
            if cond_t is not None and cond_t != "bool":
                self.diags.error(f"while condition must be bool, got '{cond_t}'",
                                 stmt.line, stmt.col)
            self.push()
            for s in stmt.body:
                self.check_stmt(s)
            self.pop()
        elif kind == "ExprStmt":
            self.check_expr(stmt.expr)

    def _check_fn_body(self, fn):
        sig = self.functions.get(fn.name)
        if sig is None:
            return
        self.current_fn_ret = sig.ret_type
        self.push()
        for pname, ptype in sig.params:
            if ptype is not None:
                self.declare(pname, ptype, mutable=True, line=fn.line, col=fn.col)
        for s in fn.body:
            self.check_stmt(s)
        self.pop()
        self.current_fn_ret = None

    def _check_let(self, stmt):
        init_t = self.check_expr(stmt.value) if stmt.value is not None else None

        if stmt.type_ann is not None:
            if stmt.type_ann not in PRIMITIVES:
                self.diags.error(f"unknown type '{stmt.type_ann}' for variable '{stmt.name}'",
                                 stmt.line, stmt.col,
                                 hint=f"supported primitive types: {', '.join(sorted(PRIMITIVES))}")
                ann_t = None
            else:
                ann_t = stmt.type_ann
            if ann_t is not None and init_t is not None and ann_t != init_t:
                self.diags.error(
                    f"cannot initialize '{stmt.type_ann}' variable '{stmt.name}' with '{init_t}' value",
                    stmt.line, stmt.col,
                    hint=f"change the annotation to '{init_t}' or fix the initializer")
        else:
            ann_t = init_t

        if ann_t is not None:
            self.declare(stmt.name, ann_t, stmt.mutable, stmt.line, stmt.col)

    def _check_return(self, stmt):
        val_t = self.check_expr(stmt.value) if stmt.value is not None else None
        expected = self.current_fn_ret
        if expected is not None:
            if stmt.value is None:
                self.diags.error(f"function must return a value of type '{expected}'",
                                 stmt.line, stmt.col)
            elif val_t is not None and val_t != expected:
                self.diags.error(f"return type mismatch: expected '{expected}', got '{val_t}'",
                                 stmt.line, stmt.col,
                                 hint="fix the returned expression or the function's declared return type")
        # TODO: missing-return-path analysis (function with declared non-None
        # return type but some path falls off the end) is NOT yet detected.

    # -- expressions --

    def check_expr(self, expr):
        """Returns static type name (str) or None when unknown/unchecked."""
        kind = expr.__class__.__name__

        if kind in _LIT_TYPES:
            return _LIT_TYPES[kind]

        if kind == "Ident":
            entry = self.lookup(expr.name)
            if entry is None:
                self.diags.error(f"undefined variable '{expr.name}'", expr.line, expr.col)
                return None
            return entry[0]

        if kind == "UnaryOp":
            t = self.check_expr(expr.operand)
            if expr.op == "-":
                if t in ("int", "float"):
                    return t
                if t is not None:
                    self.diags.error(f"unary '-' requires int or float, got '{t}'",
                                     expr.line, expr.col)
                return None
            if expr.op in ("!", "not"):
                if t is not None and t != "bool":
                    self.diags.error(f"'{expr.op}' requires bool, got '{t}'",
                                     expr.line, expr.col)
                return "bool"
            return None

        if kind == "BinOp":
            return self._check_binop(expr)

        if kind == "Call":
            return self._check_call(expr)

        if kind == "IfExpr":
            # If-expressions are parsed only in expression position; Stage 0
            # does not fully support them in the interpreter yet.
            self.diags.error("if-expressions are not supported yet", expr.line, expr.col)
            return None

        return None

    def _check_binop(self, expr):
        lt = self.check_expr(expr.left)
        rt = self.check_expr(expr.right)
        op = expr.op

        if op == "=":
            # Assignment: left must be a mutable variable
            if expr.left.__class__.__name__ != "Ident":
                self.diags.error("invalid assignment target", expr.line, expr.col)
                return None
            entry = self.lookup(expr.left.name)
            if entry is None:
                self.diags.error(f"undefined variable '{expr.left.name}'",
                                 expr.left.line, expr.left.col)
                return None
            var_t, mutable = entry
            if not mutable:
                self.diags.error(f"cannot reassign immutable variable '{expr.left.name}'",
                                 expr.left.line, expr.left.col,
                                 hint="declare it with 'var' if it needs to change")
            if rt is not None and var_t is not None and rt != var_t:
                self.diags.error(f"cannot assign '{rt}' to '{var_t}' variable '{expr.left.name}'",
                                 expr.line, expr.col)
            return var_t

        if op in ("and", "&&", "or", "||"):
            if lt is not None and lt != "bool":
                self.diags.error(f"'{op}' requires bool operands, left is '{lt}'",
                                 expr.left.line, expr.left.col)
            if rt is not None and rt != "bool":
                self.diags.error(f"'{op}' requires bool operands, right is '{rt}'",
                                 expr.right.line, expr.right.col)
            return "bool"

        if op in ("==", "!="):
            if lt is not None and rt is not None and lt != rt:
                self.diags.error(f"cannot compare '{lt}' with '{rt}' using '{op}'",
                                 expr.line, expr.col)
            return "bool"

        if op in ("<", "<=", ">", ">="):
            for side, t, node in (("left", lt, expr.left), ("right", rt, expr.right)):
                if t is not None and t not in ("int", "float", "string"):
                    self.diags.error(f"'{op}' requires int, float, or string on the {side}, got '{t}'",
                                     node.line, node.col)
            return "bool"

        if op in ("+", "-", "*", "/", "%"):
            if lt is None or rt is None:
                return None  # operand type unknown; skip further checks
            if op == "%" and ("float" in (lt, rt)):
                self.diags.error("'%' requires int operands", expr.line, expr.col)
                return None
            if op == "+":
                if lt == "string" and rt == "string":
                    return "string"
                if lt == "string" or rt == "string":
                    side = "left" if lt == "string" else "right"
                    other = rt if lt == "string" else lt
                    self.diags.error(f"cannot add '{other}' to string ({side} operand is a string)",
                                     expr.line, expr.col,
                                     hint="only string + string concatenation is supported")
                    return None
            if lt not in ("int", "float") or rt not in ("int", "float"):
                self.diags.error(f"arithmetic '{op}' requires int or float operands, got '{lt}' and '{rt}'",
                                 expr.line, expr.col)
                return None
            if lt == "float" or rt == "float":
                if op == "/":
                    return "float"
                return "float"
            return "int"

        return None

    def _check_call(self, expr):
        if expr.func.__class__.__name__ != "Ident":
            return None
        name = expr.func.name

        # Builtins
        if name in ("print", "println"):
            for a in expr.args:
                self.check_expr(a)
            return None
        if name == "assert":
            if len(expr.args) < 1 or len(expr.args) > 2:
                self.diags.error("assert takes 1 or 2 arguments", expr.line, expr.col)
            else:
                t = self.check_expr(expr.args[0])
                if t is not None and t != "bool":
                    self.diags.error(f"assert condition must be bool, got '{t}'",
                                     expr.args[0].line, expr.args[0].col)
            return "bool"

        sig = self.functions.get(name)
        if sig is None:
            entry = self.lookup(name)
            if entry is not None:
                self.diags.error(f"'{name}' is a variable, not a function",
                                 expr.line, expr.col)
            else:
                self.diags.error(f"call to undefined function '{name}'",
                                 expr.line, expr.col)
            return None

        if len(expr.args) != len(sig.params):
            self.diags.error(
                f"function '{name}' expects {len(sig.params)} argument(s), got {len(expr.args)}",
                expr.line, expr.col)
        for i, arg in enumerate(expr.args):
            at = self.check_expr(arg)
            if i < len(sig.params) and sig.params[i][1] is not None \
                    and at is not None and at != sig.params[i][1]:
                self.diags.error(
                    f"argument {i + 1} of '{name}': expected '{sig.params[i][1]}', got '{at}'",
                    arg.line, arg.col)
        return sig.ret_type
