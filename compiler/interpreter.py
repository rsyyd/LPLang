"""Stage 0 Tree-walk Interpreter and Semantic Evaluator for LPLang.

Provides direct execution of AST for immediate feedback, REPL, and testing.
Follows LPLang semantics:
- Lexical scoping
- Explicit mutability checks (let vs var)
- Built-in functions: print(), println(), assert()
"""

import time
import threading


class ReturnValue(Exception):
    def __init__(self, value):
        self.value = value


class RuntimeError(Exception):
    def __init__(self, message, line=0, col=0):
        super().__init__(message)
        self.message = message
        self.line = line
        self.col = col


class Task:
    """Represents an asynchronous computation running or resolved."""
    def __init__(self, fn_or_val=None):
        self.result_val = None
        self.error = None
        self.completed = threading.Event()
        if fn_or_val is not None:
            def runner():
                try:
                    self.result_val = fn_or_val()
                except ReturnValue as ret:
                    self.result_val = ret.value
                except Exception as err:
                    self.error = err
                finally:
                    self.completed.set()
            self.thread = threading.Thread(target=runner, daemon=True)
            self.thread.start()
        else:
            self.completed.set()

    def wait(self):
        self.completed.wait()
        if self.error:
            raise self.error
        return self.result_val

    def __repr__(self):
        status = "completed" if self.completed.is_set() else "pending"
        return f"<Task status={status}>"


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

        def _len(val):
            if isinstance(val, (list, str)):
                return len(val)
            raise RuntimeError("len() requires a list or string")

        def _append(lst, val):
            if not isinstance(lst, list):
                raise RuntimeError("append() requires a list")
            lst.append(val)
            return None

        def _sleep(ms):
            if not isinstance(ms, (int, float)):
                raise RuntimeError("sleep() requires a number in milliseconds")
            time.sleep(ms / 1000.0)
            return None

        self.globals.define("println", _println)
        self.globals.define("print", _print)
        self.globals.define("assert", _assert)
        self.globals.define("len", _len)
        self.globals.define("append", _append)
        self.globals.define("sleep", _sleep)

    def eval_program(self, program):
        last_val = None
        for stmt in program.statements:
            last_val = self.exec_stmt(stmt, self.globals)
        return last_val

    def exec_stmt(self, stmt, env):
        kind = stmt.__class__.__name__

        if kind == "LetStmt":
            val = self.eval_expr(stmt.value, env) if stmt.value else None
            matched, bindings = self.match_pattern(stmt.pattern, val)
            if not matched:
                raise RuntimeError("pattern matching failed in variable declaration", stmt.line, stmt.col)
            for name, v in bindings.items():
                env.define(name, v, stmt.mutable)
            return None

        elif kind == "FnDecl":
            # Store function closure in environment
            env.define(stmt.name, stmt, mutable=False)
            return None

        elif kind == "AsyncFnDecl":
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

        elif kind == "ForInStmt":
            coll = self.eval_expr(stmt.coll, env)
            # Support Python iterable (list, range, etc.)
            for item in coll:
                sub_env = Environment(env)
                sub_env.define(stmt.var, item, mutable=True)
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
        if pkind == "TuplePattern":
            if not isinstance(val, tuple):
                return False, {}
            if len(pattern.patterns) != len(val):
                return False, {}
            all_bindings = {}
            for sub_p, sub_v in zip(pattern.patterns, val):
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

        elif kind == "ListLit":
            return [self.eval_expr(e, env) for e in expr.elements]
        elif kind == "TupleLit":
            return tuple(self.eval_expr(e, env) for e in expr.elements)

        elif kind == "AwaitExpr":
            target = self.eval_expr(expr.expr, env)
            if isinstance(target, Task):
                return target.wait()
            # If not a task, return directly (idempotent await)
            return target

        elif kind == "MatchExpr":
            target_val = self.eval_expr(expr.target, env)
            for pattern, arm_expr in expr.arms:
                matched, bindings = self.match_pattern(pattern, target_val)
                if matched:
                    arm_env = Environment(env)
                    for bname, bval in bindings.items():
                        arm_env.define(bname, bval, mutable=True)
                    return self.eval_expr(arm_expr, arm_env)
            raise RuntimeError("match expression: no pattern matched", expr.line, expr.col)

        elif kind == "TryExpr":
            val = self.eval_expr(expr.expr, env)
            if isinstance(val, LPLangEnumVariant):
                # Result::Err or Result::Ok
                if val.enum_name == "Result":
                    if val.variant_name == "Err":
                        raise ReturnValue(val)  # early return Err
                    elif val.variant_name == "Ok":
                        return val.values[0] if val.values else None
                # Option::None or Option::Some
                if val.enum_name == "Option":
                    if val.variant_name == "None":
                        raise ReturnValue(val)  # early return None
                    elif val.variant_name == "Some":
                        return val.values[0] if val.values else None
            raise RuntimeError("operator '?' can only be used on Result or Option values", expr.line, expr.col)

        elif kind == "SpawnExpr":
            # Runs the expression in a background thread task.
            # If the expr already returns a Task (async fn call), return it directly.
            def task_work():
                return self.eval_expr(expr.expr, env)
            result = task_work()
            if isinstance(result, Task):
                return result
            # For non-async, wrap in a completed task
            t = Task()
            t.result_val = result
            return t

        elif kind == "IndexAccess":
            target = self.eval_expr(expr.target, env)
            index = self.eval_expr(expr.index, env)
            if isinstance(target, (list, tuple)):
                if not isinstance(index, int):
                    raise RuntimeError("index must be int", expr.line, expr.col)
                if index < 0 or index >= len(target):
                    raise RuntimeError(f"index out of bounds: {index}", expr.line, expr.col)
                return target[index]
            elif isinstance(target, str):
                if not isinstance(index, int):
                    raise RuntimeError("string index must be int", expr.line, expr.col)
                if index < 0 or index >= len(target):
                    raise RuntimeError(f"string index out of bounds: {index}", expr.line, expr.col)
                return target[index]
            else:
                raise RuntimeError("index access only supported on list, tuple, and string", expr.line, expr.col)

        elif kind == "BinOp":
            if expr.op == "=":
                val = self.eval_expr(expr.right, env)
                left_kind = expr.left.__class__.__name__
                if left_kind == "Ident":
                    env.assign(expr.left.name, val, expr.line, expr.col)
                    return val
                elif left_kind == "IndexAccess":
                    target = self.eval_expr(expr.left.target, env)
                    index = self.eval_expr(expr.left.index, env)
                    if isinstance(target, list):
                        if not isinstance(index, int):
                            raise RuntimeError("list index must be int", expr.line, expr.col)
                        if index < 0 or index >= len(target):
                            raise RuntimeError(f"list index out of bounds: {index}", expr.line, expr.col)
                        target[index] = val
                        return val
                    raise RuntimeError("index assignment only supported on lists", expr.line, expr.col)
                else:
                    raise RuntimeError("invalid assignment target", expr.line, expr.col)

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
            if expr.op == "..": return range(int(l), int(r) + 1)  # inclusive range

            raise RuntimeError(f"unsupported binary op '{expr.op}'", expr.line, expr.col)

        elif kind == "Call":
            fn = env.get(expr.func.name, expr.line, expr.col)
            args = [self.eval_expr(a, env) for a in expr.args]

            # Built-in python callable
            if callable(fn):
                return fn(*args)

            # User-defined FnDecl or AsyncFnDecl
            if fn.__class__.__name__ in ("FnDecl", "AsyncFnDecl"):
                if len(args) != len(fn.params):
                    raise RuntimeError(
                        f"function '{fn.name}' expects {len(fn.params)} args, got {len(args)}",
                        expr.line, expr.col
                    )
                fn_env = Environment(self.globals)
                for (param_name, _), arg_val in zip(fn.params, args):
                    fn_env.define(param_name, arg_val, mutable=True)

                def run_body():
                    try:
                        for s in fn.body:
                            self.exec_stmt(s, fn_env)
                    except ReturnValue as ret:
                        return ret.value
                    return None

                if fn.__class__.__name__ == "AsyncFnDecl":
                    return Task(run_body)
                else:
                    return run_body()

            raise RuntimeError(f"'{expr.func.name}' is not callable", expr.line, expr.col)

        raise RuntimeError(f"unknown expression kind: {kind}", expr.line, expr.col)
