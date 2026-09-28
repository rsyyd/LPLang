use ariadne::{Label, Report, ReportKind, FnCache};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub labels: Vec<Label<Span>>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: usize,
}

impl ariadne::Span for Span {
    type SourceId = usize;

    fn source(&self) -> &Self::SourceId {
        &self.file_id
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}

pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
    sources: HashMap<usize, String>,
    file_names: HashMap<usize, String>,
    next_file_id: usize,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            sources: HashMap::new(),
            file_names: HashMap::new(),
            next_file_id: 1,
        }
    }

    pub fn add_source(&mut self, name: &str, source: String) -> usize {
        let id = self.next_file_id;
        self.next_file_id += 1;
        self.sources.insert(id, source);
        self.file_names.insert(id, name.to_string());
        id
    }

    pub fn push(&mut self, kind: DiagnosticKind, label: Label<Span>, message: String) {
        self.diagnostics.push(Diagnostic {
            kind,
            message,
            labels: vec![label],
            notes: Vec::new(),
        });
    }

    pub fn error(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Error,
            message: message.into(),
            labels: vec![Label::new(span).with_message("here")],
            notes: Vec::new(),
        });
    }

    pub fn warning(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Warning,
            message: message.into(),
            labels: vec![Label::new(span).with_message("here")],
            notes: Vec::new(),
        });
    }

    pub fn note(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(Diagnostic {
            kind: DiagnosticKind::Info,
            message: message.into(),
            labels: vec![Label::new(span).with_message("note")],
            notes: Vec::new(),
        });
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.kind == DiagnosticKind::Error)
    }

    pub fn emit(&self) {
        for diag in &self.diagnostics {
            let kind = match diag.kind {
                DiagnosticKind::Error => ReportKind::Error,
                DiagnosticKind::Warning => ReportKind::Warning,
                DiagnosticKind::Info => ReportKind::Advice,
                DiagnosticKind::Help => ReportKind::Advice,
            };

            let file_id: usize = 0; // Use a default file ID

            let report = Report::build(kind, file_id, 0)
                .with_message(&diag.message)
                .with_labels(diag.labels.clone())
                .finish();

            // Notes are printed separately since add_note doesn't exist
            for note in &diag.notes {
                eprintln!("  note: {}", note);
            }

            let cache = |id: &usize| -> Result<&str, Box<dyn std::fmt::Debug>> {
                Ok(self.sources.get(id).map(|s| s.as_str()).unwrap_or(""))
            };

            let _ = report.eprint(FnCache::new(cache));
        }
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl Default for DiagnosticBag {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}