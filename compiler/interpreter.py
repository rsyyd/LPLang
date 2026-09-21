"""Stage 0 Tree-walk Interpreter and Semantic Evaluator for LPLang.

Provides direct execution of AST for immediate feedback, REPL, and testing.
Follows LPLang semantics:
- Lexical scoping
- Explicit mutability checks (let vs var)
- Built-in functions: print(), println(), assert()
"""

class ReturnValue(Exception):
    def __init__(self, value):
        self.value = value


class RuntimeError(Exception):
    def __init__(self, message, line=0, col=0):
        super().__init__(message)
        self.message = message
        self.line = line
        self.col = col


class Environment:
    def __init__(self, parent=None):
        self.parent = parent
        self.values = {}       # name -> value
        self.mutability = {}   # name -> bool (True if var, False if let)

    def define(self, name, value, mutable=False):
        self.values[name] = value
        self.mutability[name] = mutable

    def assign(self, name, value, line=0, col=0):
        if name in self.values:
            if not self.mutability.get(name, False):
                raise RuntimeError(f"cannot reassign immutable variable '{name}'", line, col)
            self.values[name] = value
            return
        if self.parent:
            self.parent.assign(name, value, line, col)
            return
        raise RuntimeError(f"undefined variable '{name}'", line, col)

    def get(self, name, line=0, col=0):
        if name in self.values:
            return self.values[name]
        if self.parent:
            return self.parent.get(name, line, col)
        raise RuntimeError(f"undefined variable '{name}'", line, col)


class LPLangStruct:
    """Runtime representation of an LPLang struct instance."""
    def __init__(self, type_name, fields):
        self.type_name = type_name
        self.fields = fields

    def __repr__(self):
        inner = ", ".join(f"{k}: {v}" for k, v in self.fields.items())
        return f"{self.type_name} {{ {inner} }}"

    def __eq__(self, other):
        return (isinstance(other, LPLangStruct)
                and self.type_name == other.type_name
                and self.fields == other.fields)


class LPLangEnumVariant:
    """Runtime representation of an enum variant."""
    def __init__(self, enum_name, variant_name, values):
        self.enum_name = enum_name
        self.variant_name = variant_name
        self.values = values  # list of payload values

    def __repr__(self):
        if not self.values:
            return f"{self.enum_name}::{self.variant_name}"
        vals = ", ".join(repr(v) for v in self.values)
        return f"{self.enum_name}::{self.variant_name}({vals})"

    def __eq__(self, other):
        return (isinstance(other, LPLangEnumVariant)
                and self.enum_name == other.enum_name
                and self.variant_name == other.variant_name
                and self.values == other.values)


class Interpreter:
    def __init__(self):
        self.globals = Environment()
        self.output = []
        self._init_builtins()

    def _init_builtins(self):
        def _println(*args):
            line = " ".join(str(a) for a in args)
            self.output.append(line)
            print(line)
            return None

        def _print(*args):
            line = " ".join(str(a) for a in args)
            if self.output and not self.output[-1].endswith("\n"):
                self.output[-1] += line
            else:
                self.output.append(line)
            print(line, end="")
            return None

        def _assert(cond, msg="Assertion failed"):
            if not cond:
                raise RuntimeError(f"assert failed: {msg}")
            return True

        self.globals.define("println", _println)
        self.globals.define("print", _print)
        self.globals.define("assert", _assert)

    def eval_program(self, program):
        last_val = None
        for stmt in program.statements:
            last_val = self.exec_stmt(stmt, self.globals)
        return last_val

    def exec_stmt(self, stmt, env):
        kind = stmt.__class__.__name__

        if kind == "LetStmt":
            val = self.eval_expr(stmt.value, env) if stmt.value else None
            env.define(stmt.name, val, stmt.mutable)
            return None

        elif kind == "FnDecl":
            # Store function closure in environment
            env.define(stmt.name, stmt, mutable=False)
            return None

        elif kind == "StructDecl":
            # Store struct definition (schema) in environment
            env.define(stmt.name, stmt, mutable=False)
            return None

        elif kind == "EnumDecl":
            # Store enum definition in environment
            env.define(stmt.name, stmt, mutable=False)
            # Register variant constructors or values directly if needed
            for vname, payloads in stmt.variants:
                key = f"{stmt.name}::{vname}"
                if len(payloads) == 0:
                    val = LPLangEnumVariant(stmt.name, vname, [])
                    env.define(key, val, mutable=False)
                else:
                    # constructor function
                    def make_ctor(s_name, v_name, num_args):
                        def ctor(*args):
                            if len(args) != num_args:
                                raise RuntimeError(f"{s_name}::{v_name} expects {num_args} args, got {len(args)}")
                            return LPLangEnumVariant(s_name, v_name, list(args))
                        return ctor
                    env.define(key, make_ctor(stmt.name, vname, len(payloads)), mutable=False)
            return None

        elif kind == "MatchStmt":
            target_val = self.eval_expr(stmt.target, env)
            for arm in stmt.arms:
                matched, bindings = self.match_pattern(arm.pattern, target_val)
                if matched:
                    arm_env = Environment(env)
                    for bname, bval in bindings.items():
                        arm_env.define(bname, bval, mutable=True)
                    for s in arm.body:
                        self.exec_stmt(s, arm_env)
                    break
            return None

        elif kind == "ReturnStmt":
            val = self.eval_expr(stmt.value, env) if stmt.value else None
            raise ReturnValue(val)

        elif kind == "IfStmt":
            cond = self.eval_expr(stmt.cond, env)
            if cond:
                sub_env = Environment(env)
                for s in stmt.then_body:
                    self.exec_stmt(s, sub_env)
            elif stmt.else_body:
                sub_env = Environment(env)
                for s in stmt.else_body:
                    self.exec_stmt(s, sub_env)
            return None

        elif kind == "WhileStmt":
            while self.eval_expr(stmt.cond, env):
                sub_env = Environment(env)
                for s in stmt.body:
                    self.exec_stmt(s, sub_env)
            return None

        elif kind == "ExprStmt":
            return self.eval_expr(stmt.expr, env)

        raise RuntimeError(f"unknown statement kind: {kind}", stmt.line, stmt.col)

    def match_pattern(self, pattern, val):
        pkind = pattern.__class__.__name__
        if pkind == "WildcardPattern":
            return True, {}
        elif pkind == "LitPattern":
            return val == pattern.value, {}
        elif pkind == "IdentPattern":
            return True, {pattern.name: val}
        elif pkind == "VariantPattern":
            if not isinstance(val, LPLangEnumVariant):
                return False, {}
            if pattern.enum_name is not None and pattern.enum_name != val.enum_name:
                return False, {}
            if pattern.variant_name != val.variant_name:
                return False, {}
            if len(pattern.sub_patterns) != len(val.values):
                return False, {}
            all_bindings = {}
            for sub_p, sub_v in zip(pattern.sub_patterns, val.values):
                matched, sub_b = self.match_pattern(sub_p, sub_v)
                if not matched:
                    return False, {}
                all_bindings.update(sub_b)
            return True, all_bindings
        return False, {}

    def eval_expr(self, expr, env):
        kind = expr.__class__.__name__

        if kind in ("IntLit", "FloatLit", "StrLit", "BoolLit"):
            return expr.value

        elif kind == "Ident":
            return env.get(expr.name, expr.line, expr.col)

        elif kind == "UnaryOp":
            val = self.eval_expr(expr.operand, env)
            if expr.op == "-":
                return -val
            elif expr.op in ("!", "not"):
                return not val
            raise RuntimeError(f"unsupported unary op '{expr.op}'", expr.line, expr.col)

        elif kind == "StructLit":
            decl = env.get(expr.type_name, expr.line, expr.col)
            if decl.__class__.__name__ != "StructDecl":
                raise RuntimeError(f"'{expr.type_name}' is not a struct", expr.line, expr.col)
            field_dict = {}
            for fname, fexpr in expr.fields:
                field_dict[fname] = self.eval_expr(fexpr, env)
            return LPLangStruct(expr.type_name, field_dict)

        elif kind == "FieldAccess":
            obj = self.eval_expr(expr.object, env)
            if not isinstance(obj, LPLangStruct):
                raise RuntimeError(f"cannot access field '{expr.field}' on non-struct", expr.line, expr.col)
            if expr.field not in obj.fields:
                raise RuntimeError(f"struct '{obj.type_name}' has no field '{expr.field}'", expr.line, expr.col)
            return obj.fields[expr.field]

        elif kind == "BinOp":
            if expr.op == "=":
                if expr.left.__class__.__name__ != "Ident":
                    raise RuntimeError("invalid assignment target", expr.line, expr.col)
                val = self.eval_expr(expr.right, env)
                env.assign(expr.left.name, val, expr.line, expr.col)
                return val

            l = self.eval_expr(expr.left, env)
            r = self.eval_expr(expr.right, env)

            if expr.op == "+": return l + r
            if expr.op == "-": return l - r
            if expr.op == "*": return l * r
            if expr.op == "/":
                if r == 0:
                    raise RuntimeError("division by zero", expr.line, expr.col)
                return l / r if isinstance(l, float) or isinstance(r, float) else l // r
            if expr.op == "%": return l % r
            if expr.op == "==": return l == r
            if expr.op == "!=": return l != r
            if expr.op == "<": return l < r
            if expr.op == "<=": return l <= r
            if expr.op == ">": return l > r
            if expr.op == ">=": return l >= r
            if expr.op in ("and", "&&"): return l and r
            if expr.op in ("or", "||"): return l or r

            raise RuntimeError(f"unsupported binary op '{expr.op}'", expr.line, expr.col)

        elif kind == "Call":
            fn = env.get(expr.func.name, expr.line, expr.col)
            args = [self.eval_expr(a, env) for a in expr.args]

            # Built-in python callable
            if callable(fn):
                return fn(*args)

            # User-defined FnDecl
            if fn.__class__.__name__ == "FnDecl":
                if len(args) != len(fn.params):
                    raise RuntimeError(
                        f"function '{fn.name}' expects {len(fn.params)} args, got {len(args)}",
                        expr.line, expr.col
                    )
                fn_env = Environment(self.globals)
                for (param_name, _), arg_val in zip(fn.params, args):
                    fn_env.define(param_name, arg_val, mutable=True)
                
                try:
                    for s in fn.body:
                        self.exec_stmt(s, fn_env)
                except ReturnValue as ret:
                    return ret.value
                return None

            raise RuntimeError(f"'{expr.func.name}' is not callable", expr.line, expr.col)

        raise RuntimeError(f"unknown expression kind: {kind}", expr.line, expr.col)
