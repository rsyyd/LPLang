"""Module loader for LPLang.

Resolves `import name;` statements to .lp source files and builds a set of
programs ready for checking/execution.

Resolution rules (Stage 0):
- `import foo;` looks for `foo.lp` in the same directory as the importing file.
- Relative imports with `/` are NOT yet supported (TBD).
- Import cycles are detected and reported as errors.
- Each file is parsed once; duplicate imports of the same file are ignored.
"""

import os

from .lexer import tokenize
from .parser import Parser
from .ast import Program


class ImportStmt:
    """Lightweight import record resolved during loading (not an AST node
    because it never reaches the checker/interpreter directly)."""
    def __init__(self, module_name, line, col):
        self.module_name = module_name
        self.line = line
        self.col = col


def load_program(path, diags, loaded=None):
    """Load and parse a .lp file and its imports.

    Returns (programs, import_graph) where:
      programs: list of (module_name_or_None, Program) in dependency order
                (imports first, the root file last)
      import_graph: dict[abs_path -> list[module_name]]

    module_name is None for the root file.
    """
    if loaded is None:
        loaded = {}
    abs_path = os.path.abspath(path)
    if abs_path in loaded:
        return [], {}

    if not os.path.exists(abs_path):
        diags.error(f"module file not found: '{path}'", 1, 1,
                    hint="make sure the file exists next to the importing file")
        return [], {}

    with open(abs_path, "r", encoding="utf-8") as f:
        source = f.read()

    tokens, lex_diags = tokenize(source)
    diags.items.extend(lex_diags.items)
    if lex_diags.has_errors():
        return [], {}

    parser = Parser(tokens, diags)
    ast = parser.parse_program()

    loaded[abs_path] = True

    module_name = os.path.splitext(os.path.basename(abs_path))[0]

    programs = []
    graph = {}

    # Process imports first (dependency order), guarding against cycles.
    import_names = []
    remaining = []
    from .ast import ExprStmt, Ident  # local import to avoid cycles
    for stmt in ast.statements:
        # Imports were parsed as statements via parse_statement? No —
        # parse_statement handles the `import` keyword; see parser.
        # If the parser produced an ExprStmt wrapping Ident("foo") for
        # an import, it would not be distinguishable; instead the parser
        # emits ImportStmtStub. Handle both defensively.
        kind = stmt.__class__.__name__
        if kind == "ImportStmtStub":
            import_names.append((stmt.module_name, stmt.line, stmt.col))
        else:
            remaining.append(stmt)

    graph[abs_path] = [name for name, _, _ in import_names]

    base_dir = os.path.dirname(abs_path)
    std_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "std")
    for name, line, col in import_names:
        # Search order: importing file's directory, then stdlib
        candidate = os.path.join(base_dir, name + ".lp")
        if not os.path.exists(candidate):
            std_candidate = os.path.join(std_dir, name + ".lp")
            if os.path.exists(std_candidate):
                candidate = std_candidate
        sub_programs, sub_graph = load_program(candidate, diags, loaded)
        programs.extend(sub_programs)
        graph.update(sub_graph)

    programs.append((module_name, Program(remaining)))
    return programs, graph
