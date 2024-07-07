use crate::imports::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app //
        .init_resource::<DiagnosticUpdate>()
        .add_systems(Update, update.run_if(should_update_diagnostics));
}

#[derive(Resource, Clone, PartialEq)]
pub(super) struct DiagnosticUpdate {
    pub rate: f32,
    pub(super) t: f32,
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
