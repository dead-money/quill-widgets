use super::*;
use crate::imports::*;
use memory_stats::memory_stats;

#[derive(Component, Clone, PartialEq, Default)]
pub struct MemDiagnosticState {
    usage: f32,
}

impl DiagnosticView for MemDiagnosticState {
    const LABEL: &'static str = "MEM";

    fn update(&mut self, _cx: &mut Cx) {
        if let Some(usage) = memory_stats() {
            self.usage = usage.physical_mem as f32 / 1024.0 / 1024.0;
        }
    }
}

impl fmt::Display for MemDiagnosticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.usage)
    }
}

pub type MemDiagnostic = DiagnosticWidget<MemDiagnosticState>;
