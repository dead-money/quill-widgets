use crate::imports::*;

#[derive(Default, Clone, PartialEq)]
pub struct StateWatcher<T: States + std::fmt::Display + Default> {
    pub style: StyleHandle,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: States + std::fmt::Display + Default> StateWatcher<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style<S: StyleTuple + 'static>(mut self, style: S) -> Self {
        self.style = style.into_handle();
        self
    }
}

impl<T: States + std::fmt::Display + Default> ViewTemplate for StateWatcher<T> {
    type View = impl View;

    fn create(&self, cx: &mut Cx) -> Self::View {
        let id = cx.create_entity();

        let state = cx.use_resource::<State<T>>();

        Element::<NodeBundle>::for_entity(id)
            // .named(T::LABEL)
            .style((
                typography::text_default,
                style_diagnostic,
                self.style.clone(),
            ))
            .children((Element::<NodeBundle>::new()
                .style(style_diagnostic_value)
                .children(state.to_string()),))
    }
}

fn style_diagnostic(sb: &mut StyleBuilder) {
    sb //
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .gap(4)
        .color(colors::FOREGROUND);
}

fn style_diagnostic_value(sb: &mut StyleBuilder) {
    sb //
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .color(colors::FOREGROUND);
}
