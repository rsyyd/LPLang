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
    FnDecl, LetStmt, ReturnStmt, ForInStmt, IfStmt, WhileStmt, ExprStmt,
    IntLit, FloatLit, StrLit, BoolLit, Ident, BinOp, UnaryOp, Call, IfExpr,
    StructLit, TupleLit, IndexAccess
)

PRIMITIVES = {"int", "float", "string", "bool", "list", "tuple"}

# Map AST literal/expr class name -> static type
_LIT_TYPES = {
    "IntLit": "int",
    "FloatLit": "float",
    "StrLit": "string",
    "BoolLit": "bool",
    "TupleLit": "tuple",
    "ListLit": "list",
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
        self.structs = {}           # name -> dict[field_name, field_type]
        self.enums = {}             # name -> dict[variant_name, list[payload_type]]
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

    def is_valid_type(self, typ):
        return typ in PRIMITIVES or typ in self.structs or typ in self.enums

    def check_program(self, program):
        # Pass 0: collect struct and enum declarations
        for stmt in program.statements:
            if stmt.__class__.__name__ == "StructDecl":
                self._register_struct(stmt)
            elif stmt.__class__.__name__ == "EnumDecl":
                self._register_enum(stmt)

        # Pass 1: collect function signatures (allows forward references)
        for stmt in program.statements:
            if stmt.__class__.__name__ in ("FnDecl", "AsyncFnDecl"):
                self._register_fn(stmt)

        # Pass 2: check bodies
        for stmt in program.statements:
            self.check_stmt(stmt)

    def _register_enum(self, edecl):
        if edecl.name in self.enums or edecl.name in self.structs or edecl.name in PRIMITIVES:
            self.diags.error(f"duplicate type definition '{edecl.name}'", edecl.line, edecl.col)
            return
        vmap = {}
        for vname, payloads in edecl.variants:
            for p in payloads:
                if not self.is_valid_type(p):
                    self.diags.error(f"unknown payload type '{p}' in enum '{edecl.name}'", edecl.line, edecl.col)
            vmap[vname] = payloads
        self.enums[edecl.name] = vmap

    def _register_struct(self, sdecl):
        if sdecl.name in self.structs or sdecl.name in PRIMITIVES:
            self.diags.error(f"duplicate type definition '{sdecl.name}'", sdecl.line, sdecl.col)
            return
        tparams = getattr(sdecl, 'type_params', []) or []
        fields = {}
        for fname, ftype in sdecl.fields:
            if not self.is_valid_type(ftype) and ftype not in tparams:
                self.diags.error(f"unknown type '{ftype}' for field '{fname}' in struct '{sdecl.name}'",
                                 sdecl.line, sdecl.col)
            fields[fname] = ftype
        self.structs[sdecl.name] = {"fields": fields, "type_params": tparams}

    def _register_fn(self, fn):
        # Collect type params from generic functions
        tparams = getattr(fn, 'type_params', []) or []

        params = []
        for pname, ptype in fn.params:
            if ptype is None:
                self.diags.error(
                    f"parameter '{pname}' of function '{fn.name}' needs a type annotation",
                    fn.line, fn.col,
                    hint="write e.g. fn f(x: int) so calls can be type-checked")
                params.append((pname, None))
            elif not self.is_valid_type(ptype) and ptype not in tparams:
                self.diags.error(f"unknown type '{ptype}' for parameter '{pname}'",
                                 fn.line, fn.col)
                params.append((pname, None))
            else:
                params.append((pname, ptype))

        if fn.ret_type is not None and not self.is_valid_type(fn.ret_type) and fn.ret_type not in tparams:
            self.diags.error(f"unknown return type '{fn.ret_type}' for function '{fn.name}'",
                             fn.line, fn.col)
            ret = None
        else:
            ret = fn.ret_type

        sig = FnSignature(params, ret)
        sig.type_params = tparams
        self.functions[fn.name] = sig

    # -- statements --

    def check_stmt(self, stmt):
        kind = stmt.__class__.__name__

        if kind in ("FnDecl", "AsyncFnDecl"):
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
        elif kind == "ForInStmt":
            coll_t = self.check_expr(stmt.coll)
            if coll_t is not None and coll_t != "list" and coll_t != "range":
                self.diags.error(f"for-in expects a list or range, got '{coll_t}'",
                                 stmt.line, stmt.col)
            self.push()
            self.declare(stmt.var, "int", True, stmt.line, stmt.col)
            for s in stmt.body:
                self.check_stmt(s)
            self.pop()
        elif kind == "MatchStmt":
            target_t = self.check_expr(stmt.target)
            if target_t is None:
                return
            has_wildcard = False
            covered_variants = set()
            for arm in stmt.arms:
                self.push()
                self.check_pattern(arm.pattern, target_t)
                for s in arm.body:
                    self.check_stmt(s)
                self.pop()
                pk = arm.pattern.__class__.__name__
                if pk == "WildcardPattern":
                    has_wildcard = True
                elif pk == "IdentPattern":
                    has_wildcard = True  # ident binds anything = catch-all
                elif pk == "VariantPattern":
                    covered_variants.add(arm.pattern.variant_name)
            # Exhaustiveness check for enums
            if target_t in self.enums and not has_wildcard:
                all_variants = set(self.enums[target_t].keys())
                missing = all_variants - covered_variants
                if missing:
                    self.diags.error(
                        f"match on '{target_t}' is not exhaustive: missing variant(s) {', '.join(sorted(missing))}",
                        stmt.line, stmt.col,
                        hint="add arms for the missing variants or a '_' wildcard")
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
            if not self.is_valid_type(stmt.type_ann):
                self.diags.error(f"unknown type '{stmt.type_ann}' for variable '{stmt.name}'",
                                 stmt.line, stmt.col)
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

        # Declare even when ann_t is None (e.g. spawn/await/index results)
        # so the variable is in scope; its type is simply unchecked.
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

    def check_pattern(self, pattern, expected_type):
        pkind = pattern.__class__.__name__
        if pkind == "WildcardPattern":
            return
        if pkind == "LitPattern":
            lit_type = _LIT_TYPES.get(pattern.value.__class__.__name__)
            if lit_type is not None and expected_type != lit_type:
                self.diags.error(f"literal pattern type '{lit_type}' does not match expected type '{expected_type}'",
                                 pattern.line, pattern.col)
            return
        if pkind == "IdentPattern":
            # Bind the variable to the expected type
            self.declare(pattern.name, expected_type, mutable=True, line=pattern.line, col=pattern.col)
            return
        if pkind == "VariantPattern":
            if expected_type not in self.enums:
                self.diags.error(f"pattern expects enum type, got '{expected_type}'",
                                 pattern.line, pattern.col)
                return
            enum_variants = self.enums[expected_type]
            if pattern.enum_name is not None and pattern.enum_name != expected_type:
                self.diags.error(f"pattern enum name '{pattern.enum_name}' does not match target enum '{expected_type}'",
                                 pattern.line, pattern.col)
                return
            if pattern.variant_name not in enum_variants:
                self.diags.error(f"enum '{expected_type}' has no variant '{pattern.variant_name}'",
                                 pattern.line, pattern.col)
                return
            payload_types = enum_variants[pattern.variant_name]
            if len(pattern.sub_patterns) != len(payload_types):
                self.diags.error(f"variant '{pattern.variant_name}' expects {len(payload_types)} payload(s), got {len(pattern.sub_patterns)}",
                                 pattern.line, pattern.col)
                return
            for sub_p, ptype in zip(pattern.sub_patterns, payload_types):
                self.check_pattern(sub_p, ptype)
            return
        # Unknown pattern type
        return

    # -- expressions --

    def check_expr(self, expr):
        """Returns static type name (str) or None when unknown/unchecked."""
        kind = expr.__class__.__name__

        if kind in _LIT_TYPES:
            return _LIT_TYPES[kind]

        if kind == "ListLit":
            for e in expr.elements:
                self.check_expr(e)
            return "list"
        if kind == "TupleLit":
            for e in expr.elements:
                self.check_expr(e)
            return "tuple"

        if kind == "IndexAccess":
            t = self.check_expr(expr.target)
            idx_t = self.check_expr(expr.index)
            if idx_t is not None and idx_t != "int":
                self.diags.error(f"index must be int, got '{idx_t}'", expr.index.line, expr.index.col)
            # ponytail: returns None (element type unknown without generics); upgrade when generic list<T> is added.
            return None

        if kind == "AwaitExpr":
            # Awaiting resolves the task to its result type; element type
            # unknown without full async generics, so return None (unchecked).
            self.check_expr(expr.expr)
            return None

        if kind == "MatchExpr":
            target_t = self.check_expr(expr.target)
            if target_t is None:
                return None
            result_types = []
            for pattern, arm_expr in expr.arms:
                self.push()
                self.check_pattern(pattern, target_t)
                arm_t = self.check_expr(arm_expr)
                self.pop()
                result_types.append(arm_t)
            # All arms must agree on the result type (when known)
            known = [t for t in result_types if t is not None]
            if known and all(t == known[0] for t in known):
                return known[0]
            return known[0] if known else None

        if kind == "TryExpr":
            target_t = self.check_expr(expr.expr)
            if target_t in ("Result", "Option"):
                # Enclosing function must return the same enum type
                if self.current_fn_ret is not None and self.current_fn_ret != target_t:
                    self.diags.error(
                        f"operator '?' on '{target_t}' inside function returning '{self.current_fn_ret}'",
                        expr.line, expr.col,
                        hint=f"the enclosing function must return '{target_t}' to use '?' with it")
                # Look up payload type of Ok or Some
                variants = self.enums.get(target_t, {})
                ok_payloads = variants.get("Ok", variants.get("Some", []))
                return ok_payloads[0] if ok_payloads else None
            elif target_t is not None:
                self.diags.error(
                    f"operator '?' can only be used on Result or Option, got '{target_t}'",
                    expr.line, expr.col)
            return None

        if kind == "SpawnExpr":
            self.check_expr(expr.expr)
            return None

        if kind == "Ident":
            # First, check if it's a variable or function in scope
            entry = self.lookup(expr.name)
            if entry is not None:
                return entry[0]
            # If not found, check if it's an enum variant constructor (or unit variant)
            # Format: VariantName or EnumName::VariantName
            if "::" in expr.name:
                enum_name, variant_name = expr.name.split("::", 1)
                if enum_name in self.enums and variant_name in self.enums[enum_name]:
                    # It's a variant constructor (or value)
                    payload_types = self.enums[enum_name][variant_name]
                    if len(payload_types) == 0:
                        # Unit variant: it's a value of the enum type
                        return enum_name
                    else:
                        # It's a constructor function: takes payload_types and returns enum_name
                        # We don't return a type here because it's a function; the call will be handled in _check_call
                        # For now, we can return None and let _check_call handle it, or we can return a special marker.
                        # Simpler: return None and let _check_call fail if not handled.
                        return None
            # Also check for unit variant without enum prefix (if the enum is in scope? Not typical)
            # We'll leave it as undefined.
            self.diags.error(f"undefined variable or enum variant '{expr.name}'", expr.line, expr.col)
            return None

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

        if kind == "StructLit":
            if expr.type_name not in self.structs:
                self.diags.error(f"unknown struct type '{expr.type_name}'", expr.line, expr.col)
                return None
            struct_info = self.structs[expr.type_name]
            expected_fields = struct_info["fields"] if isinstance(struct_info, dict) else struct_info
            provided_fields = set()
            for fname, fexpr in expr.fields:
                provided_fields.add(fname)
                ft = self.check_expr(fexpr)
                if fname not in expected_fields:
                    self.diags.error(f"struct '{expr.type_name}' has no field named '{fname}'", fexpr.line, fexpr.col)
                else:
                    exp_t = expected_fields[fname]
                    # Don't fail if expected type is a generic type param
                    tparams = struct_info.get("type_params", []) if isinstance(struct_info, dict) else []
                    if exp_t not in tparams and ft is not None and ft != exp_t:
                        self.diags.error(f"field '{fname}' of '{expr.type_name}': expected '{exp_t}', got '{ft}'",
                                         fexpr.line, fexpr.col)
            for ef in expected_fields:
                if ef not in provided_fields:
                    self.diags.error(f"missing field '{ef}' in literal for '{expr.type_name}'", expr.line, expr.col)
            return expr.type_name

        if kind == "FieldAccess":
            obj_t = self.check_expr(expr.object)
            if obj_t is None:
                return None
            if obj_t not in self.structs:
                self.diags.error(f"cannot access field on non-struct type '{obj_t}'", expr.line, expr.col)
                return None
            struct_info = self.structs[obj_t]
            fields = struct_info["fields"] if isinstance(struct_info, dict) else struct_info
            if expr.field not in fields:
                self.diags.error(f"struct '{obj_t}' has no field named '{expr.field}'", expr.line, expr.col)
                return None
            f_type = fields[expr.field]
            tparams = struct_info.get("type_params", []) if isinstance(struct_info, dict) else []
            # If the field is a generic type parameter, we can't statically know its concrete type yet
            # without monomorphization/full generic types, so return None (unchecked)
            return None if f_type in tparams else f_type

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
            # Assignment: left must be a mutable variable or index access
            left_kind = expr.left.__class__.__name__
            if left_kind == "IndexAccess":
                self.check_expr(expr.left.target)
                idx_t = self.check_expr(expr.left.index)
                if idx_t is not None and idx_t != "int":
                    self.diags.error(f"index must be int, got '{idx_t}'", expr.left.index.line, expr.left.index.col)
                self.check_expr(expr.right)
                return None
            if left_kind != "Ident":
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

        # Check if it's an enum variant constructor: EnumName::VariantName
        if "::" in name:
            enum_name, variant_name = name.split("::", 1)
            if enum_name in self.enums and variant_name in self.enums[enum_name]:
                payload_types = self.enums[enum_name][variant_name]
                if len(expr.args) != len(payload_types):
                    self.diags.error(
                        f"enum constructor '{name}' expects {len(payload_types)} argument(s), got {len(expr.args)}",
                        expr.line, expr.col)
                for i, (arg, expected) in enumerate(zip(expr.args, payload_types)):
                    at = self.check_expr(arg)
                    if at is not None and at != expected:
                        self.diags.error(
                            f"argument {i + 1} of '{name}': expected '{expected}', got '{at}'",
                            arg.line, arg.col)
                return enum_name  # constructor returns the enum type

        # Builtins
        if name in ("print", "println"):
            for a in expr.args:
                self.check_expr(a)
            return None
        if name == "len":
            if len(expr.args) != 1:
                self.diags.error("len takes exactly 1 argument", expr.line, expr.col)
            else:
                self.check_expr(expr.args[0])
            return "int"
        if name == "append":
            if len(expr.args) != 2:
                self.diags.error("append takes exactly 2 arguments", expr.line, expr.col)
            else:
                self.check_expr(expr.args[0])
                self.check_expr(expr.args[1])
            return None
        if name == "sleep":
            if len(expr.args) != 1:
                self.diags.error("sleep takes exactly 1 argument", expr.line, expr.col)
            else:
                self.check_expr(expr.args[0])
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

        # Check argument count
        if len(expr.args) != len(sig.params):
            self.diags.error(
                f"function '{name}' expects {len(sig.params)} argument(s), got {len(expr.args)}",
                expr.line, expr.col)
        # Infer type parameters and substitute
        type_map = {}
        for (pname, ptype), arg_expr in zip(sig.params, expr.args):
            at = self.check_expr(arg_expr)
            if ptype is not None and not self.is_valid_type(ptype) and ptype in sig.type_params:
                # This is a type parameter, infer from argument
                if at is not None:
                    type_map[ptype] = at
            elif at is not None and ptype is not None and at != ptype:
                self.diags.error(
                    f"argument mismatch: expected type '{ptype}', got '{at}'",
                    arg_expr.line, arg_expr.col)
        # Substitute return type
        ret_type = sig.ret_type
        if ret_type is not None and ret_type in type_map:
            ret_type = type_map[ret_type]
        # Check arguments with substitution (for non-type-param types)
        for i, (pname, ptype) in enumerate(sig.params):
            if ptype is not None and not self.is_valid_type(ptype) and ptype in sig.type_params:
                # Skip type parameters, already handled above
                continue
            at = self.check_expr(expr.args[i])
            if ptype is not None and ptype in type_map:
                ptype = type_map[ptype]
            if at is not None and ptype is not None and at != ptype:
                self.diags.error(
                    f"argument {i + 1} of '{name}': expected type '{ptype}', got '{at}'",
                    expr.args[i].line, expr.args[i].col)
        return ret_type
