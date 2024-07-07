use super::*;
use crate::imports::*;
use memory_stats::memory_stats;

#[derive(Component, Copy, Clone)]
struct MemDiagnosticState {
    usage: f32,
}

/// A memory diagnostic widget.
#[derive(Default, Clone, PartialEq)]
pub struct MemDiagnostic {
    /// Additional styles to be applied to the widget.
    pub style: StyleHandle,
}

impl MemDiagnostic {
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

impl ViewTemplate for MemDiagnostic {
    type View = impl View;

    /// Construct a MemDiagnostic widget.
    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();

        let mut entity = cx.world_mut().entity_mut(id);
        if !entity.contains::<MemDiagnosticState>() {
            entity.insert(MemDiagnosticState { usage: 0.0 });
        }

        let state = cx.use_component::<MemDiagnosticState>(id).unwrap();
        let update = cx.use_resource::<DiagnosticUpdate>();

        Element::<NodeBundle>::for_entity(id)
            .named("MEM")
            .style((
                typography::text_default,
                style_diagnostic,
                self.style.clone(),
            ))
            .effect(
                move |cx, ent, _| {
                    let usage = some!(memory_stats());
                    let mem = usage.physical_mem as f32 / 1024.0 / 1024.0;

                    let mut ent_mut = cx.world_mut().entity_mut(ent);
                    let mut mem_state = ent_mut.get_mut::<MemDiagnosticState>().unwrap();

                    mem_state.usage = mem;
                },
                update.t,
            )
            .children((
                Element::<NodeBundle>::new()
                    .named("MEM::Label")
                    .style(style_diagnostic_label)
                    .children("MEM:"),
                Element::<NodeBundle>::new()
                    .named("MEM::Counter")
                    .style(style_diagnostic_value)
                    .children(format!("{:.2}", state.usage)),
            ))
    }
}

