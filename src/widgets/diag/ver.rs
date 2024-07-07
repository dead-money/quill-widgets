use super::*;
use crate::imports::*;
use version::version;

#[derive(Component, Clone, PartialEq, Default)]
pub struct VersionState {
    version: String,
}

impl DiagnosticView for VersionState {
    const LABEL: &'static str = "VER";

    fn update(&mut self, _cx: &mut Cx) {
        self.version = version!().to_string();
    }
}

impl fmt::Display for VersionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

pub type Version = DiagnosticWidget<VersionState>;
