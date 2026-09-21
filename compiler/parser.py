"""Recursive descent + Pratt parser for LPLang.

Parses token stream into AST defined in ast.py.
Collects diagnostics into DiagnosticBag rather than crashing on syntax errors.
"""

from .diagnostics import DiagnosticBag
from .ast import (
    Program, FnDecl, LetStmt, ReturnStmt, IfStmt, WhileStmt, ExprStmt,
    IntLit, FloatLit, StrLit, BoolLit, Ident, BinOp, UnaryOp, Call, IfExpr,
    StructDecl, StructLit, FieldAccess,
    EnumDecl, MatchStmt, MatchArm,
    WildcardPattern, LitPattern, IdentPattern, VariantPattern,
    ImportStmtStub,
    ListLit, IndexAccess
)

# Precedence table for binary operators (higher = tighter binding)
PRECEDENCE = {
    "or": 1,
    "||": 1,
    "and": 2,
    "&&": 2,
    "==": 3,
    "!=": 3,
    "<": 4,
    "<=": 4,
    ">": 4,
    ">=": 4,
    "+": 5,
    "-": 5,
    "*": 6,
    "/": 6,
    "%": 6,
}


class Parser:
    def __init__(self, tokens, diags=None):
        self.tokens = tokens
        self.pos = 0
        self.diags = diags if diags is not None else DiagnosticBag()

    def current(self):
        if self.pos < len(self.tokens):
            return self.tokens[self.pos]
        return self.tokens[-1]  # EOF token

    def peek(self, offset=1):
        idx = self.pos + offset
        if idx < len(self.tokens):
            return self.tokens[idx]
        return self.tokens[-1]

    def advance(self):
        tok = self.current()
        if tok.kind != "EOF":
            self.pos += 1
        return tok

    def match(self, kind, value=None):
        tok = self.current()
        if tok.kind == kind and (value is None or tok.value == value):
            self.advance()
            return tok
        return None

    def expect(self, kind, value=None, hint=None):
        tok = self.match(kind, value)
        if tok is None:
            curr = self.current()
            expected_desc = f"{kind}:{value}" if value else kind
            actual_desc = f"{curr.kind}:{curr.value}" if curr.value else curr.kind
            self.diags.error(
                f"expected {expected_desc}, found {actual_desc}",
                curr.line,
                curr.column,
                hint=hint
            )
            return None
        return tok

    def parse_program(self):
        stmts = []
        while self.current().kind != "EOF":
            stmt = self.parse_statement()
            if stmt is not None:
                stmts.append(stmt)
            else:
                # Synchronize to next statement boundary on syntax error
                self.synchronize()
        return Program(stmts)

    def synchronize(self):
        self.advance()
        while self.current().kind != "EOF":
            if self.peek(-1).kind == "OP" and self.peek(-1).value == ";":
                return
            if self.current().kind == "KEYWORD" and self.current().value in ("let", "var", "fn", "return", "if", "while"):
                return
            self.advance()

    def parse_statement(self):
        tok = self.current()
        if tok.kind == "KEYWORD":
            if tok.value in ("let", "var"):
                return self.parse_let_or_var()
            elif tok.value == "fn":
                return self.parse_fn_decl()
            elif tok.value == "struct":
                return self.parse_struct_decl()
            elif tok.value == "enum":
                return self.parse_enum_decl()
            elif tok.value == "match":
                return self.parse_match_stmt()
            elif tok.value == "import":
                return self.parse_import()
            elif tok.value == "return":
                return self.parse_return()
            elif tok.value == "if":
                return self.parse_if_stmt()
            elif tok.value == "while":
                return self.parse_while_stmt()

        # Expression statement (including assignment or bare function call)
        expr = self.parse_expression()
        if expr is None:
            return None
        
        # Semicolons are optional at statement ends if followed by newline/block end,
        # but allowed and consumed if present.
        self.match("OP", ";")
        return ExprStmt(expr, expr.line, expr.col)

    def parse_let_or_var(self):
        kw = self.advance()
        mutable = (kw.value == "var")

        ident = self.expect("IDENT", hint="provide variable name after 'let' or 'var'")
        if not ident:
            return None

        type_ann = None
        if self.match("OP", ":"):
            type_tok = self.expect("IDENT", hint="provide type name after ':'")
            if type_tok:
                type_ann = type_tok.value

        val = None
        if self.match("OP", "="):
            val = self.parse_expression()
        else:
            self.diags.error("variable declaration requires initial value", kw.line, kw.column,
                             hint="assign with '=' or specify default value")

        self.match("OP", ";")
        return LetStmt(ident.value, type_ann, val, mutable, kw.line, kw.column)

    def parse_import(self):
        kw = self.advance()  # consume 'import'
        ident = self.expect("IDENT", hint="provide module name after 'import'")
        if not ident:
            return None
        self.match("OP", ";")
        return ImportStmtStub(ident.value, kw.line, kw.column)

    def parse_struct_decl(self):
        kw = self.advance()  # consume 'struct'
        ident = self.expect("IDENT", hint="provide struct name after 'struct'")
        if not ident:
            return None

        self.expect("OP", "{", hint="open struct body with '{'")
        fields = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            fname = self.expect("IDENT", hint="expected field name")
            if not fname:
                break
            self.expect("OP", ":", hint="expected ':' after field name")
            ftype_tok = self.expect("IDENT", hint="expected field type")
            if not ftype_tok:
                break
            fields.append((fname.value, ftype_tok.value))
            self.match("OP", ",")  # optional trailing comma
        
        self.expect("OP", "}", hint="close struct body with '}'")
        return StructDecl(ident.value, fields, kw.line, kw.column)

    def parse_enum_decl(self):
        kw = self.advance()  # consume 'enum'
        ident = self.expect("IDENT", hint="provide enum name after 'enum'")
        if not ident:
            return None

        self.expect("OP", "{", hint="open enum body with '{'")
        variants = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            vname = self.expect("IDENT", hint="expected variant name")
            if not vname:
                break
            payloads = []
            if self.match("OP", "("):
                if not self.match("OP", ")"):
                    while True:
                        ptype = self.expect("IDENT", hint="expected payload type name")
                        if ptype:
                            payloads.append(ptype.value)
                        if not self.match("OP", ","):
                            break
                    self.expect("OP", ")", hint="expected closing ')' after variant payload types")
            variants.append((vname.value, payloads))
            self.match("OP", ",")
        
        self.expect("OP", "}", hint="close enum body with '}'")
        return EnumDecl(ident.value, variants, kw.line, kw.column)

    def parse_match_stmt(self):
        kw = self.advance()  # consume 'match'
        target = self.parse_expression()
        self.expect("OP", "{", hint="expected '{' after match target")
        arms = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            pattern = self.parse_pattern()
            self.expect("OP", "=>", hint="expected '=>' after match pattern")
            body = []
            if self.match("OP", "{"):
                while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
                    stmt = self.parse_statement()
                    if stmt:
                        body.append(stmt)
                self.expect("OP", "}", hint="expected '}' to close arm block")
            else:
                stmt = self.parse_statement()
                if stmt:
                    body.append(stmt)
            self.match("OP", ",")
            arms.append(MatchArm(pattern, body, pattern.line, pattern.col))

        self.expect("OP", "}", hint="expected '}' to close match body")
        return MatchStmt(target, arms, kw.line, kw.column)

    def parse_pattern(self):
        tok = self.current()
        # Wildcard _
        if tok.kind == "IDENT" and tok.value == "_":
            self.advance()
            return WildcardPattern(tok.line, tok.column)

        # Literals (int, float, string, bool)
        if tok.kind == "INT":
            self.advance()
            return LitPattern(int(tok.value), tok.line, tok.column)
        if tok.kind == "FLOAT":
            self.advance()
            return LitPattern(float(tok.value), tok.line, tok.column)
        if tok.kind == "STRING":
            self.advance()
            return LitPattern(tok.value, tok.line, tok.column)
        if tok.kind == "KEYWORD" and tok.value in ("true", "false"):
            self.advance()
            return LitPattern(tok.value == "true", tok.line, tok.column)

        # Identifier or Enum::Variant(...) or Variant(...)
        if tok.kind == "IDENT":
            first = self.advance()
            enum_name = None
            var_name = first.value
            if self.match("OP", "::") or (self.current().kind == "OP" and self.current().value == ":" and self.peek().value == ":"):
                # Handle double colon (either single token or two colons)
                if not self.match("OP", "::"):
                    self.advance() # :
                    self.advance() # :
                enum_name = first.value
                vtok = self.expect("IDENT", hint="expected variant name after '::'")
                if vtok:
                    var_name = vtok.value

            # Payload patterns: Variant(p1, p2)
            if self.match("OP", "("):
                sub_patterns = []
                if not self.match("OP", ")"):
                    while True:
                        sub_pat = self.parse_pattern()
                        if sub_pat:
                            sub_patterns.append(sub_pat)
                        if not self.match("OP", ","):
                            break
                    self.expect("OP", ")", hint="expected ')' after variant pattern arguments")
                return VariantPattern(enum_name, var_name, sub_patterns, first.line, first.column)

            # If it starts with uppercase and has enum_name or is capital, treat as unit variant
            if enum_name is not None or var_name[0].isupper():
                return VariantPattern(enum_name, var_name, [], first.line, first.column)

            # Otherwise it's a variable binding pattern
            return IdentPattern(var_name, first.line, first.column)

        self.diags.error(f"unexpected token in pattern: {tok.kind}:{tok.value}", tok.line, tok.column,
                         hint="expected identifier, literal, or variant pattern")
        self.advance()
        return WildcardPattern(tok.line, tok.column)

    def parse_fn_decl(self):
        kw = self.advance()
        ident = self.expect("IDENT", hint="provide function name after 'fn'")
        if not ident:
            return None

        self.expect("OP", "(", hint="open parameter list with '('")
        params = []
        if not self.match("OP", ")"):
            while True:
                pname = self.expect("IDENT", hint="expected parameter name")
                ptype = None
                if pname and self.match("OP", ":"):
                    ptok = self.expect("IDENT", hint="expected parameter type")
                    if ptok:
                        ptype = ptok.value
                if pname:
                    params.append((pname.value, ptype))
                if not self.match("OP", ","):
                    break
            self.expect("OP", ")", hint="close parameter list with ')'")

        ret_type = None
        if self.match("OP", "->"):
            rtok = self.expect("IDENT", hint="expected return type after '->'")
            if rtok:
                ret_type = rtok.value

        self.expect("OP", "{", hint="open function body with '{'")
        body = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            stmt = self.parse_statement()
            if stmt:
                body.append(stmt)
        self.expect("OP", "}", hint="close function body with '}'")

        return FnDecl(ident.value, params, ret_type, body, kw.line, kw.column)

    def parse_return(self):
        kw = self.advance()
        val = None
        # If not immediately followed by semicolon or right brace, parse expression
        if not (self.current().kind == "OP" and self.current().value in (";", "}")):
            val = self.parse_expression()
        self.match("OP", ";")
        return ReturnStmt(val, kw.line, kw.column)

    def parse_if_stmt(self):
        kw = self.advance()
        cond = self.parse_expression()
        self.expect("OP", "{", hint="expected '{' after if condition")
        then_body = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            stmt = self.parse_statement()
            if stmt:
                then_body.append(stmt)
        self.expect("OP", "}", hint="expected '}' to close if block")

        else_body = None
        if self.match("KEYWORD", "else"):
            if self.current().kind == "KEYWORD" and self.current().value == "if":
                # else if chained as nested single statement
                else_stmt = self.parse_if_stmt()
                else_body = [else_stmt] if else_stmt else []
            else:
                self.expect("OP", "{", hint="expected '{' after else")
                else_body = []
                while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
                    stmt = self.parse_statement()
                    if stmt:
                        else_body.append(stmt)
                self.expect("OP", "}", hint="expected '}' to close else block")

        return IfStmt(cond, then_body, else_body, kw.line, kw.column)

    def parse_while_stmt(self):
        kw = self.advance()
        cond = self.parse_expression()
        self.expect("OP", "{", hint="expected '{' after while condition")
        body = []
        while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
            stmt = self.parse_statement()
            if stmt:
                body.append(stmt)
        self.expect("OP", "}", hint="expected '}' to close while block")
        return WhileStmt(cond, body, kw.line, kw.column)

    # Expression parsing (Pratt precedence climbing)
    def parse_expression(self, min_prec=0):
        left = self.parse_prefix()
        if left is None:
            return None

        while True:
            tok = self.current()
            if tok.kind in ("OP", "KEYWORD") and tok.value in PRECEDENCE:
                prec = PRECEDENCE[tok.value]
                if prec < min_prec:
                    break
                op_tok = self.advance()
                right = self.parse_expression(prec + 1)
                left = BinOp(op_tok.value, left, right, op_tok.line, op_tok.column)
            else:
                break

        return left

    def parse_prefix(self):
        tok = self.current()

        # Unary operators: -, !, not
        if (tok.kind == "OP" and tok.value in ("-", "!")) or (tok.kind == "KEYWORD" and tok.value == "not"):
            op_tok = self.advance()
            operand = self.parse_expression(min_prec=6)
            return UnaryOp(op_tok.value, operand, op_tok.line, op_tok.column)

        # Parenthesized expression
        if tok.kind == "OP" and tok.value == "(":
            self.advance()
            expr = self.parse_expression()
            self.expect("OP", ")", hint="expected closing ')'")
            return expr

        # Literals
        if tok.kind == "INT":
            self.advance()
            return IntLit(int(tok.value), tok.line, tok.column)

        if tok.kind == "FLOAT":
            self.advance()
            return FloatLit(float(tok.value), tok.line, tok.column)

        if tok.kind == "STRING":
            self.advance()
            return StrLit(tok.value, tok.line, tok.column)

        if tok.kind == "KEYWORD" and tok.value in ("true", "false"):
            self.advance()
            return BoolLit(tok.value == "true", tok.line, tok.column)

        # List literal [1, 2, 3]
        if tok.kind == "OP" and tok.value == "[":
            self.advance()
            elements = []
            if not self.match("OP", "]"):
                while True:
                    elem = self.parse_expression()
                    if elem:
                        elements.append(elem)
                    if not self.match("OP", ","):
                        break
                self.expect("OP", "]", hint="expected ']' to close list literal")
            return self._parse_postfix(ListLit(elements, tok.line, tok.column))

        # Identifier or Call / Struct Literal / Field Access
        if tok.kind == "IDENT":
            ident_tok = self.advance()
            
            # Struct literal: TypeName { field: expr, ... }
            if self.current().kind == "OP" and self.current().value == "{":
                # Lookahead to distinguish struct lit from block/if/while body is tricky in pure LL(1)
                # Heuristic: If we are in an expression context and see IDENT {, assume struct lit.
                # This might conflict with if/while blocks if not careful, but parse_expression is usually RHS.
                # Better: Check if next tokens look like `field :` pattern? 
                # For now, simple approach: if it's a capitalized identifier (convention) OR just try parsing as struct.
                # Actually, let's use a flag or check context. 
                # Since this is a bootstrap, let's assume any IDENT followed by { in expression position is a struct literal.
                # We need to save pos to backtrack if it fails? No, parser doesn't backtrack easily here.
                
                # Let's refine: Only treat as struct literal if the identifier starts with uppercase (common convention)
                # OR if we explicitly want to support lowercase structs. 
                # Given LPLang spec draft says `struct Point`, let's stick to Capitalized for literals to avoid ambiguity with blocks.
                if ident_tok.value[0].isupper():
                    self.advance() # consume '{'
                    fields = []
                    while self.current().kind != "EOF" and not (self.current().kind == "OP" and self.current().value == "}"):
                        fname = self.expect("IDENT", hint="expected field name in struct literal")
                        if not fname: break
                        self.expect("OP", ":", hint="expected ':' after field name in struct literal")
                        fexpr = self.parse_expression()
                        if fexpr is None: break
                        fields.append((fname.value, fexpr))
                        self.match("OP", ",")
                    self.expect("OP", "}", hint="close struct literal with '}'")
                    return StructLit(ident_tok.value, fields, ident_tok.line, ident_tok.column)

            node = Ident(ident_tok.value, ident_tok.line, ident_tok.column)
            return self._parse_postfix(node)

        self.diags.error(f"unexpected token {tok.kind}:{tok.value}", tok.line, tok.column,
                         hint="expected expression (number, string, identifier, parentheses)")
        return None

    def _parse_postfix(self, node):
        while True:
            if self.current().kind == "OP" and self.current().value == ".":
                self.advance()
                field_tok = self.expect("IDENT", hint="expected field name after '.'")
                if not field_tok:
                    break
                node = FieldAccess(node, field_tok.value, field_tok.line, field_tok.column)
            elif self.current().kind == "OP" and self.current().value == "[":
                self.advance()
                idx_expr = self.parse_expression()
                self.expect("OP", "]", hint="expected ']' to close index")
                node = IndexAccess(node, idx_expr, node.line, node.col)
            elif self.current().kind == "OP" and (self.current().value == "::" or (self.current().value == ":" and self.peek().value == ":")):
                if not self.match("OP", "::"):
                    self.advance()
                    self.advance()
                vtok = self.expect("IDENT", hint="expected variant name after '::'")
                if not vtok:
                    break
                node = Ident(f"{getattr(node, 'name', '')}::{vtok.value}", node.line, node.col)
            elif self.current().kind == "OP" and self.current().value == "(":
                self.advance()
                args = []
                if not self.match("OP", ")"):
                    while True:
                        arg = self.parse_expression()
                        if arg:
                            args.append(arg)
                        if not self.match("OP", ","):
                            break
                    self.expect("OP", ")", hint="expected closing ')' in function call")
                node = Call(node, args, node.line, node.col)
            else:
                break

        if isinstance(node, (Ident, FieldAccess, IndexAccess)):
            if self.match("OP", "="):
                val = self.parse_expression()
                return BinOp("=", node, val, node.line, node.col)

        return node
