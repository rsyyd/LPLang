"""AST node definitions for LPLang.

Design: concrete classes with explicit fields, no premature abstractions.
"""
from dataclasses import dataclass, field
from typing import Optional, Any


class Node:
    """Base for all AST nodes. Each carries a 1-based line/col for diagnostics."""
    def __init__(self, line=0, col=0):
        self.line = line
        self.col = col


# ---- Expressions ----

@dataclass
class IntLit(Node):
    value: int
    line: int = 0
    col: int = 0
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class FloatLit(Node):
    value: float
    line: int = 0
    col: int = 0
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class StrLit(Node):
    value: str
    line: int = 0
    col: int = 0
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class BoolLit(Node):
    value: bool
    line: int = 0
    col: int = 0
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class Ident(Node):
    name: str
    line: int = 0
    col: int = 0
    def __init__(self, name, line=0, col=0):
        super().__init__(line, col)
        self.name = name

@dataclass
class BinOp(Node):
    op: str
    left: Any
    right: Any
    line: int = 0
    col: int = 0
    def __init__(self, op, left, right, line=0, col=0):
        super().__init__(line, col)
        self.op = op
        self.left = left
        self.right = right

@dataclass
class UnaryOp(Node):
    op: str
    operand: Any
    line: int = 0
    col: int = 0
    def __init__(self, op, operand, line=0, col=0):
        super().__init__(line, col)
        self.op = op
        self.operand = operand

@dataclass
class Call(Node):
    func: Any       # Ident for now; later also method calls
    args: list
    line: int = 0
    col: int = 0
    def __init__(self, func, args, line=0, col=0):
        super().__init__(line, col)
        self.func = func
        self.args = args

@dataclass
class StructLit(Node):
    type_name: str
    fields: list      # list of (field_name, expr) tuples
    line: int = 0
    col: int = 0
    def __init__(self, type_name, fields, line=0, col=0):
        super().__init__(line, col)
        self.type_name = type_name
        self.fields = fields

@dataclass
class FieldAccess(Node):
    object: Any
    field: str
    line: int = 0
    col: int = 0
    def __init__(self, object, field, line=0, col=0):
        super().__init__(line, col)
        self.object = object
        self.field = field

@dataclass
class IfExpr(Node):
    cond: Any
    then_branch: Any
    else_branch: Optional[Any]
    line: int = 0
    col: int = 0
    def __init__(self, cond, then_branch, else_branch, line=0, col=0):
        super().__init__(line, col)
        self.cond = cond
        self.then_branch = then_branch
        self.else_branch = else_branch


# ---- Statements ----

@dataclass
class LetStmt(Node):
    name: str
    type_ann: Optional[str]   # explicit type annotation; None = infer
    value: Any
    mutable: bool             # True for var, False for let
    line: int = 0
    col: int = 0
    def __init__(self, name, type_ann, value, mutable, line=0, col=0):
        super().__init__(line, col)
        self.name = name
        self.type_ann = type_ann
        self.value = value
        self.mutable = mutable

@dataclass
class ExprStmt(Node):
    expr: Any
    line: int = 0
    col: int = 0
    def __init__(self, expr, line=0, col=0):
        super().__init__(line, col)
        self.expr = expr

@dataclass
class ReturnStmt(Node):
    value: Optional[Any]
    line: int = 0
    col: int = 0
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class IfStmt(Node):
    cond: Any
    then_body: list
    else_body: Optional[list]
    line: int = 0
    col: int = 0
    def __init__(self, cond, then_body, else_body, line=0, col=0):
        super().__init__(line, col)
        self.cond = cond
        self.then_body = then_body
        self.else_body = else_body

@dataclass
class WhileStmt(Node):
    cond: Any
    body: list
    line: int = 0
    col: int = 0
    def __init__(self, cond, body, line=0, col=0):
        super().__init__(line, col)
        self.cond = cond
        self.body = body

@dataclass
class FnDecl(Node):
    name: str
    params: list          # list of (name, type_ann) tuples
    ret_type: Optional[str]
    body: list            # list of statements
    line: int = 0
    col: int = 0
    def __init__(self, name, params, ret_type, body, line=0, col=0):
        super().__init__(line, col)
        self.name = name
        self.params = params
        self.ret_type = ret_type
        self.body = body

@dataclass
class StructDecl(Node):
    name: str
    fields: list          # list of (field_name, field_type) tuples
    line: int = 0
    col: int = 0
    def __init__(self, name, fields, line=0, col=0):
        super().__init__(line, col)
        self.name = name
        self.fields = fields

@dataclass
class EnumDecl(Node):
    name: str
    variants: list        # list of (variant_name, list_of_payload_types)
    line: int = 0
    col: int = 0
    def __init__(self, name, variants, line=0, col=0):
        super().__init__(line, col)
        self.name = name
        self.variants = variants

# ---- Pattern Matching ----

class Pattern(Node):
    """Marker base class for patterns (not a dataclass: has no fields)."""
    pass

@dataclass
class WildcardPattern(Pattern):
    def __init__(self, line=0, col=0):
        super().__init__(line, col)

@dataclass
class LitPattern(Pattern):
    value: Any
    def __init__(self, value, line=0, col=0):
        super().__init__(line, col)
        self.value = value

@dataclass
class IdentPattern(Pattern):
    name: str
    def __init__(self, name, line=0, col=0):
        super().__init__(line, col)
        self.name = name

@dataclass
class VariantPattern(Pattern):
    enum_name: Optional[str]
    variant_name: str
    sub_patterns: list
    def __init__(self, enum_name, variant_name, sub_patterns, line=0, col=0):
        super().__init__(line, col)
        self.enum_name = enum_name
        self.variant_name = variant_name
        self.sub_patterns = sub_patterns

@dataclass
class MatchArm(Node):
    pattern: Any
    body: list            # list of statements
    line: int = 0
    col: int = 0
    def __init__(self, pattern, body, line=0, col=0):
        super().__init__(line, col)
        self.pattern = pattern
        self.body = body

@dataclass
class MatchStmt(Node):
    target: Any
    arms: list            # list of MatchArm
    line: int = 0
    col: int = 0
    def __init__(self, target, arms, line=0, col=0):
        super().__init__(line, col)
        self.target = target
        self.arms = arms

@dataclass
class ImportStmtStub(Node):
    module_name: str
    line: int = 0
    col: int = 0
    def __init__(self, module_name, line=0, col=0):
        super().__init__(line, col)
        self.module_name = module_name


# ---- Top level ----

@dataclass
class Program(Node):
    statements: list = field(default_factory=list)
    def __init__(self, statements=None):
        super().__init__(0, 0)
        self.statements = statements or []
