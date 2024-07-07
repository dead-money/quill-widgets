pub mod diag;

use crate::imports::*;

pub(super) fn plugin(app: &mut App) {
    app //
        .add_plugins(diag::plugin);
}
