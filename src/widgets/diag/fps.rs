use super::*;
use crate::imports::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component, Copy, Clone)]
struct FpsDiagnosticState {
    smoothed_fps: f64,
}

/// An fps diagnostic widget.
#[derive(Default, Clone, PartialEq)]
pub struct FpsDiagnostic {
    /// Additional styles to be applied to the widget.
    pub style: StyleHandle,
}

impl FpsDiagnostic {
    /// Create a new fps diagnostic widget.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the style of the widget.
    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl ViewTemplate for FpsDiagnostic {
    type View = impl View;

    /// Construct a FpsDiagnostic widget.
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();

        let mut entity = cx.world_mut().entity_mut(id);
        if !entity.contains::<FpsDiagnosticState>() {
            entity.insert(FpsDiagnosticState { smoothed_fps: 0.0 });
        }

        let state = cx.use_component::<FpsDiagnosticState>(id).unwrap();
        let update = cx.use_resource::<DiagnosticUpdate>();

        Element::<NodeBundle>::for_entity(id)
            .named("FPS")
            .style((
                typography::text_default,
                style_diagnostic,
                self.style.clone(),
            ))
            .effect(
                move |cx, ent, _| {
                    let diagnostics = cx.world().resource::<DiagnosticsStore>();
                    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
                    {
                        let smoothed_fps = fps_diagnostic.smoothed();
                        let mut ent_mut = cx.world_mut().entity_mut(ent);
                        let mut fps_state = ent_mut.get_mut::<FpsDiagnosticState>().unwrap();
                        fps_state.smoothed_fps = some!(smoothed_fps);
                    }
                },
                update.t,
            )
            .children((
                Element::<NodeBundle>::new()
                    .named("FPS::Label")
                    .style(style_diagnostic_label)
                    .children("FPS:"),
                Element::<NodeBundle>::new()
                    .named("FPS::Counter")
                    .style_dyn(
                        |fps, sb| style_diagnostic_value(sb, fps),
                        state.smoothed_fps,
                    )
                    .children(format!("{:3.0}", state.smoothed_fps)),
            ))
    }
}

fn style_diagnostic_value(sb: &mut StyleBuilder, fps: f64) {
    let color = if fps > 30.0 {
        colors::Y_GREEN
    } else {
        colors::X_RED
    };

    sb.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .align_items(ui::AlignItems::Center)
        .color(color);
}
