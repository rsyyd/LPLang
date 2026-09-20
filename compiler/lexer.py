"""Lexer for LPLang (Stage 0 bootstrap).

Produces a token stream with 1-based line/column positions.
Design notes:
- Keywords are contextual: identifiers that match KEYWORDS become keyword tokens.
- Comments: // line comments only (block comments TBD in spec).
- String literals: double-quoted, escapes \n \t \\ \" \' \r \0.
- Numeric literals: integers and decimals. Underscore separators allowed
  between digits (e.g. 1_000_000). Scientific notation TBD (not yet supported).
- Operators are matched longest-first to avoid ambiguous splits.

Any unknown character or malformed literal produces a Diagnostic, not an
exception; the lexer recovers at the next line so one bad token does not
hide later diagnostics.
"""

from .diagnostics import DiagnosticBag

KEYWORDS = {
    "let", "var", "fn", "return", "if", "else", "while", "for", "in",
    "break", "continue", "true", "false", "struct", "enum", "match",
    "import", "export", "and", "or", "not",
}

# Two-character operators checked before single-character ones.
TWO_CHAR_OPS = {"==", "!=", "<=", ">=", "&&", "||", "->", "=>", "..", "<<", ">>"}
ONE_CHAR_OPS = set("+-*/%=<>!&|(){}[];,:.?@")

ESCAPES = {"n": "\n", "t": "\t", "r": "\r", "0": "\0", "\\": "\\", '"': '"', "'": "'"}


class Token:
    __slots__ = ("kind", "value", "line", "column")

    def __init__(self, kind, value, line, column):
        self.kind = kind      # KEYWORD | IDENT | INT | FLOAT | STRING | OP | EOF
        self.value = value
        self.line = line
        self.column = column

    def __repr__(self):
        return f"Token({self.kind}, {self.value!r}, {self.line}:{self.column})"

    def __eq__(self, other):
        return (isinstance(other, Token) and self.kind == other.kind
                and self.value == other.value
                and self.line == other.line and self.column == other.column)


def tokenize(source):
    """Tokenize source text. Returns (tokens, DiagnosticBag).

    tokens always ends with an EOF token. On lexical errors, tokens may be
    incomplete but well-formed up to the point of failure.
    """
    tokens = []
    diags = DiagnosticBag()
    line, col = 1, 1
    i, n = 0, len(source)

    def advance(k=1):
        nonlocal i, line, col
        for _ in range(k):
            if i < n and source[i] == "\n":
                line += 1
                col = 1
            else:
                col += 1
            i += 1

    while i < n:
        c = source[i]

        # Whitespace
        if c in " \t\r\n":
            advance()
            continue

        # Line comment
        if c == "/" and i + 1 < n and source[i + 1] == "/":
            while i < n and source[i] != "\n":
                advance()
            continue

        start_line, start_col = line, col

        # Identifier / keyword
        if c.isalpha() or c == "_":
            j = i
            while j < n and (source[j].isalnum() or source[j] == "_"):
                j += 1
            word = source[i:j]
            kind = "KEYWORD" if word in KEYWORDS else "IDENT"
            tokens.append(Token(kind, word, start_line, start_col))
            advance(j - i)
            continue

        # Numeric literal (int or float, underscore separators)
        if c.isdigit():
            j = i
            seen_dot = False
            while j < n:
                ch = source[j]
                if ch == ".":
                    if seen_dot or j + 1 >= n or not source[j + 1].isdigit():
                        break  # trailing dot is not part of the number
                    seen_dot = True
                    j += 1
                elif ch == "_":
                    # underscore must sit between digits
                    if j + 1 >= n or not source[j + 1].isdigit() or not source[j - 1].isdigit():
                        diags.error("underscore in number must be between digits",
                                    line, col + (j - i), hint="write 1_000 not 1__0 or 100_")
                        j += 1
                        break
                    j += 1
                elif ch.isdigit():
                    j += 1
                else:
                    break
            text = source[i:j].replace("_", "")
            kind = "FLOAT" if seen_dot else "INT"
            tokens.append(Token(kind, text, start_line, start_col))
            advance(j - i)
            continue

        # String literal
        if c == '"':
            advance()
            buf = []
            closed = False
            while i < n:
                ch = source[i]
                if ch == '"':
                    advance()
                    closed = True
                    break
                if ch == "\n":
                    diags.error("unterminated string literal", start_line, start_col,
                                hint='close the string with a " before the line break')
                    break
                if ch == "\\":
                    advance()
                    if i < n and source[i] in ESCAPES:
                        buf.append(ESCAPES[source[i]])
                        advance()
                    else:
                        got = source[i] if i < n else "end of file"
                        diags.error(f"unknown escape sequence \\{got}", start_line, start_col,
                                    hint="valid escapes: \\n \\t \\r \\0 \\\\ \\\" \\'")
                        if i < n:
                            advance()
                    continue
                buf.append(ch)
                advance()
            if not closed and (i >= n or buf == [] and i >= n):
                pass  # error already emitted for newline case
            if not closed and i >= n:
                diags.error("unterminated string literal", start_line, start_col,
                            hint='close the string with a "')
            tokens.append(Token("STRING", "".join(buf), start_line, start_col))
            continue

        # Operators
        two = source[i:i + 2]
        if two in TWO_CHAR_OPS:
            tokens.append(Token("OP", two, start_line, start_col))
            advance(2)
            continue
        if c in ONE_CHAR_OPS:
            tokens.append(Token("OP", c, start_line, start_col))
            advance()
            continue

        # Unknown character
        diags.error(f"unexpected character {c!r}", start_line, start_col,
                    hint="remove this character; only ASCII letters, digits, and known operators are supported")
        advance()

    tokens.append(Token("EOF", None, line, col))
    return tokens, diags
