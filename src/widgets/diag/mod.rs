mod fps;
mod mem;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use std::time::Duration;

pub use fps::*;
pub use mem::*;

use crate::imports::*;

pub(super) fn plugin(app: &mut App) {
    app //
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .init_resource::<DiagnosticUpdate>()
        .add_systems(Update, update.run_if(should_update_diagnostics));
}

#[derive(Resource, Clone, PartialEq)]
struct DiagnosticUpdate {
    pub rate: f32,
    t: f32,
}

impl Default for DiagnosticUpdate {
    fn default() -> Self {
        Self { rate: 1.0, t: 0.0 }
    }
}

fn should_update_diagnostics(
    mut timer: Local<Duration>,
    diagnostic_update: Res<DiagnosticUpdate>,
    time: Res<Time<Real>>,
) -> bool {
    let delta = time.delta();
    if delta > *timer {
        *timer = Duration::from_secs(0);
    } else {
        *timer -= delta;
    }

    if timer.as_secs_f32() == 0.0 {
        *timer = Duration::from_secs_f32(diagnostic_update.rate);
        true
    } else {
        false
    }
}

fn update(mut diagnostic_update: ResMut<DiagnosticUpdate>) {
    diagnostic_update.t += diagnostic_update.rate;
}

// TODO:
// Generalize all diagnostic widgets code.
// Add a diagnostic popup (optional).
// Add more diagnostics.
// Use obsidian colors (for fps, just red when below a configurable threshold).

fn style_diagnostic(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .gap(4)
        .color(colors::FOREGROUND);
}

fn style_diagnostic_label(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(colors::FOREGROUND);
}

fn style_diagnostic_value(sb: &mut StyleBuilder) {
    sb.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(colors::Y_GREEN);
}
