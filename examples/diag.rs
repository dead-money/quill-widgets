#![feature(impl_trait_in_assoc_type, associated_type_defaults)]

use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::*;
use bevy_mod_stylebuilder::*;
use bevy_quill::*;
use bevy_quill_obsidian::ObsidianUiPlugin;

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
            ))
    }
}

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .position(ui::PositionType::Absolute)
        // .padding(3)
        // .left(0)
        // .right(0)
        // .top(0)
        // .bottom(0)
        // .row_gap(4)
        // .background_color(colors::U2)
        ;
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
