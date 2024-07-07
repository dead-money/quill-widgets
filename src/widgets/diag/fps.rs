use super::*;
use crate::imports::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component, Copy, Clone, PartialEq)]
pub struct FpsDiagnosticState {
    smoothed_fps: f64,
    threshold: f64,
}

impl Default for FpsDiagnosticState {
    fn default() -> Self {
        Self {
            smoothed_fps: 0.0,
            threshold: 30.0,
        }
    }
}

impl DiagnosticView for FpsDiagnosticState {
    const LABEL: &'static str = "FPS";

    fn update(&mut self, cx: &mut Cx) {
        let diagnostics = cx.world().resource::<DiagnosticsStore>();
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            self.smoothed_fps = fps_diagnostic.smoothed().unwrap_or(0.0);
        }
    }

    fn format(&self) -> String {
        format!("{:3.0}", self.smoothed_fps)
    }

    fn color(&self) -> Srgba {
        if self.smoothed_fps > self.threshold {
            colors::Y_GREEN
        } else {
            colors::X_RED
        }
    }
}

pub type FpsDiagnostic = DiagnosticWidget<FpsDiagnosticState>;
