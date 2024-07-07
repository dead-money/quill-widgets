#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

mod widgets;

pub mod prelude {
    pub use crate::widgets::*;
}

pub mod imports {
    // Standard Library
    pub(crate) use std::fmt;

    // Bevy
    // A refreshingly simple data-driven game engine built in Rust.
    // https://docs.rs/bevy/latest/bevy/
    pub(crate) use bevy::{prelude::*, ui};

    // Quill
    pub(crate) use bevy_mod_stylebuilder::*;
    pub(crate) use bevy_quill::prelude::*;
    pub(crate) use bevy_quill_obsidian::{colors, typography};
}

use crate::imports::*;

pub struct DeadMoneyQuillWidgetsPlugin;

impl Plugin for DeadMoneyQuillWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(widgets::plugin);
    }
}
