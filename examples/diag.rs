#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::{colors, ObsidianUiPlugin};

use dead_money_quill_widgets::{prelude::*, DeadMoneyQuillWidgetsPlugin};

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
            DeadMoneyQuillWidgetsPlugin,
        ))
        .add_systems(Startup, setup_demo)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_demo(mut commands: Commands) {
    let camera = commands
        .spawn(Camera2dBundle {
            camera: Camera::default(),
            camera_2d: Camera2d {},
            ..default()
        })
        .id();

    commands.spawn(DiagnosticsDemo { camera }.to_root());
}

#[derive(Clone, PartialEq)]
struct DiagnosticsDemo {
    camera: Entity,
}

impl ViewTemplate for DiagnosticsDemo {
    type View = impl View;

    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .insert_dyn(TargetCamera, self.camera)
            .style(style_test)
            .children((
                diag::FpsDiagnostic::new(),
                diag::MemDiagnostic::new(),
                diag::EntityDiagnostic::new(),
                diag::Version::new(),
            ))
    }
}

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .position(ui::PositionType::Absolute)
        .right(10.0)
        .bottom(10.0)
        .padding(4.0)
        .background_color(colors::BACKGROUND)
        .border_color(colors::FOREGROUND)
        .border(1.0)
        .border_radius(8.0);
}

fn _style_row(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .column_gap(4);
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
