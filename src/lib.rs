#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

// mod sickle;
mod widgets;

pub mod prelude {
    pub use crate::{
        // sickle::*,
        widgets::*,
    };
}

pub mod imports {
    // Standard Library
    // pub(crate) use std::collections::VecDeque;
    // pub(crate) use std::marker::PhantomData;
    // pub(crate) use std::path::{Path, PathBuf};

    // Bevy
    // A refreshingly simple data-driven game engine built in Rust.
    // https://docs.rs/bevy/latest/bevy/
    pub(crate) use bevy::{prelude::*, ui};

    // Quill
    pub(crate) use bevy_mod_stylebuilder::*;
    pub(crate) use bevy_quill::prelude::*;
    pub(crate) use bevy_quill_obsidian::{colors, typography};

    // Dead Money
    // pub(crate) use crate::prelude::*;
    // pub(crate) use dead_money_bevy_ext::prelude::*;
}

use crate::imports::*;

pub struct DeadMoneyQuillWidgetsPlugin;

impl Plugin for DeadMoneyQuillWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(widgets::plugin);
    }
}
