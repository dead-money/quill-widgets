use super::*;
use crate::imports::*;
use memory_stats::memory_stats;

#[derive(Component, Copy, Clone, PartialEq, Default)]
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

    fn format(&self) -> String {
        format!("{:.2}", self.usage)
    }
}

pub type MemDiagnostic = DiagnosticWidget<MemDiagnosticState>;
