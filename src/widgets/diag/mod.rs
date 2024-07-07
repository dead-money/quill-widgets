mod fps;
mod mem;
mod update;
mod entity;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

pub use fps::*;
pub use mem::*;
pub use entity::*;

use crate::imports::*;

pub(super) fn plugin(app: &mut App) {
    app //
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            update::plugin,
        ));
}

pub trait DiagnosticView: Component + Copy + Clone + PartialEq + Default + fmt::Display {
    const LABEL: &'static str;

    fn update(&mut self, cx: &mut Cx);

    fn color(&self) -> Srgba {
        colors::Y_GREEN
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct DiagnosticWidget<T: DiagnosticView> {
    pub style: StyleHandle,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DiagnosticView> DiagnosticWidget<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl<T: DiagnosticView> ViewTemplate for DiagnosticWidget<T> {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();

        let mut entity = cx.world_mut().entity_mut(id);
        if !entity.contains::<T>() {
            entity.insert(T::default());
        }

        let state = cx.use_component::<T>(id).unwrap();
        let update = cx.use_resource::<update::DiagnosticUpdate>();
        let color = state.color();

        Element::<NodeBundle>::for_entity(id)
            .named(T::LABEL)
            .style((
                typography::text_default,
                style_diagnostic,
                self.style.clone(),
            ))
            .effect(
                // If the update resource is ticked, update the widget state.
                // For example, we might read the current memory utilization at the update rate.
                move |cx, ent, _| {
                    // Take and replace to avoid borrowing issues.
                    let mut ent_mut = cx.world_mut().entity_mut(ent);
                    let mut state = ent_mut.take::<T>().unwrap();
                    state.update(cx);

                    let mut ent_mut = cx.world_mut().entity_mut(ent);
                    ent_mut.insert(state);
                },
                update.t,
            )
            .children((
                Element::<NodeBundle>::new()
                    .named(&format!("{}::Label", T::LABEL))
                    .style(style_diagnostic_label)
                    .children(format!("{}:", T::LABEL)),
                Element::<NodeBundle>::new()
                    .named(&format!("{}::Counter", T::LABEL))
                    .style(style_diagnostic_value)
                    .style_dyn(
                        // Allow the state to change the value's color.
                        // This is useful for the FPS counter, which turns red when the FPS is below a threshold.
                        |(_t, color), sb| {
                            sb.color(color);
                        },
                        (update.t, color),
                    )
                    .children(state.to_string()),
            ))
    }
}

// TODO:
// Add a diagnostic popup (optional).
// Add more diagnostics.
// Use obsidian colors (for fps, just red when below a configurable threshold).

fn style_diagnostic(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .align_content(ui::AlignContent::Center)
        .gap(4)
        .color(colors::FOREGROUND);
}

fn style_diagnostic_label(ss: &mut StyleBuilder) {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(colors::FOREGROUND);
}

fn style_diagnostic_value(sb: &mut StyleBuilder) {
    sb.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(colors::Y_GREEN);
}
