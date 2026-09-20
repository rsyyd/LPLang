"""Diagnostics for the LPLang bootstrap compiler.

A diagnostic is a structured error/warning with location info and an
actionable message. Never emit bare "something went wrong" strings.
"""


class Diagnostic:
    def __init__(self, severity, message, line, column, hint=None):
        self.severity = severity          # "error" | "warning"
        self.message = message
        self.line = line
        self.column = column
        self.hint = hint

    def render(self, source_line=None):
        where = f"error[{self.line}:{self.column}]"
        text = f"{where}: {self.message}"
        if source_line is not None:
            caret = " " * (self.column - 1) + "^"
            text += f"\n  {self.line} | {source_line}\n  {caret}"
        if self.hint:
            text += f"\nhint: {self.hint}"
        return text


class DiagnosticBag:
    """Collects diagnostics during compilation."""

    def __init__(self):
        self.items = []

    def error(self, message, line, column, hint=None):
        self.items.append(Diagnostic("error", message, line, column, hint))

    def has_errors(self):
        return any(d.severity == "error" for d in self.items)

    def count_errors(self):
        return sum(1 for d in self.items if d.severity == "error")

    def render_all(self, source_lines):
        return "\n".join(d.render(source_lines[d.line - 1]) if 0 < d.line <= len(source_lines) else d.render() for d in self.items)
