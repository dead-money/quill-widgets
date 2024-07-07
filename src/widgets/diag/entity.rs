use super::*;
use crate::imports::*;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin};

#[derive(Component, Copy, Clone, PartialEq, Default)]
pub struct EntityDiagnosticState {
    entity_count: f64,
}

impl DiagnosticView for EntityDiagnosticState {
    const LABEL: &'static str = "ENT";

    fn update(&mut self, cx: &mut Cx) {
        let diagnostics = cx.world().resource::<DiagnosticsStore>();
        let Some(entity_count) = diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        else {
            return;
        };

        let Some(entity_count) = entity_count.value() else {
            return;
        };

        self.entity_count = entity_count;
    }
}

impl fmt::Display for EntityDiagnosticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.entity_count)
    }
}

pub type EntityDiagnostic = DiagnosticWidget<EntityDiagnosticState>;
