use ariadne::{Config, Label, Report, ReportKind, Source};
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

    pub fn push(&mut self, label: Label<Span>) {
        self.diagnostics.push(Diagnostic {
            kind: match label.kind {
                ariadne::ReportKind::Error => DiagnosticKind::Error,
                ariadne::ReportKind::Warning => DiagnosticKind::Warning,
                ariadne::ReportKind::Advice => DiagnosticKind::Help,
                ariadne::ReportKind::Note => DiagnosticKind::Info,
                _ => DiagnosticKind::Info,
            },
            message: label.message.unwrap_or_default(),
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
                DiagnosticKind::Info => ReportKind::Note,
                DiagnosticKind::Help => ReportKind::Advice,
            };

            let mut report = Report::build(kind, (), 0)
                .with_message(&diag.message)
                .with_labels(diag.labels.clone())
                .with_notes(diag.notes.clone())
                .finish();

            let mut cache = |id: &usize| {
                self.sources.get(id).map(|s| s.as_str()).unwrap_or("")
            };

            report.eprint(Config::default().with_source_cache(&mut cache)).unwrap();
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