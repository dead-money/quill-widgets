pub mod diag;
pub mod ecs;

use crate::imports::*;

pub(super) fn plugin(app: &mut App) {
    app //
        .add_plugins((diag::plugin, ecs::plugin));
}
