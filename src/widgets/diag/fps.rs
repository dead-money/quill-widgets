use super::*;
use crate::imports::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component, Clone, PartialEq)]
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

    fn color(&self) -> Srgba {
        if self.smoothed_fps > self.threshold {
            colors::Y_GREEN
        } else {
            colors::X_RED
        }
    }
}

impl fmt::Display for FpsDiagnosticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:3.0}", self.smoothed_fps)
    }
}

pub type FpsDiagnostic = DiagnosticWidget<FpsDiagnosticState>;
