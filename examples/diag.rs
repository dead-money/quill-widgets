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
        .init_state::<TestState>()
        .add_systems(Startup, setup_demo)
        .add_systems(Update, (change_state, close_on_esc))
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
                ecs::StateWatcher::<TestState>::new(),
            ))
    }
}

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .position(ui::PositionType::Absolute)
        .right(10.0)
        .bottom(10.0)
        .padding(4.0)
        .column_gap(12.0)
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

fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestState {
    #[default]
    Menu,
    Pause,
    Game,
}

impl std::fmt::Display for TestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_string = match self {
            TestState::Menu => "menu",
            TestState::Pause => "pause",
            TestState::Game => "game",
        };

        write!(f, "{}", state_string)
    }
}

fn change_state(
    current_state: Res<State<TestState>>,
    mut next_state: ResMut<NextState<TestState>>,
    mut count: Local<u32>,
) {
    *count += 1;
    if *count == 1000 {
        *count = 0;
        if *current_state == TestState::Menu {
            next_state.set(TestState::Game);
        } else {
            next_state.set(TestState::Menu);
        }
    }
}
