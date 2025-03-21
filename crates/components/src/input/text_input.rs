use std::ops::Range;

use gpui::{
    Bounds, Context, EntityInputHandler, EventEmitter, InteractiveElement, ParentElement, Pixels,
    Render, SharedString, Styled, UTF16Selection, Window, div, prelude::FluentBuilder,
};

use super::text_element::TextElement;

pub enum InputEvent {
    Change(SharedString),
    PressEnter,
    Focus,
    Blur,
}

pub struct TextInput {
    pub text: SharedString,
    multi_line: bool,
    pub selected_range: Range<usize>,
    pub marked_range: Option<Range<usize>>,
    pub last_line_height: Pixels,
}

impl EventEmitter<InputEvent> for TextInput {}

impl TextInput {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            text: String::new().into(),
            multi_line: false,
            selected_range: 0..0,
            marked_range: None,
            last_line_height: Pixels(20.),
        }
    }

    pub fn multi_line(mut self) -> Self {
        self.multi_line = true;
        self
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.text.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.text.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range: std::ops::Range<usize>,
        adjusted_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range);
        adjusted_range.replace(self.range_from_utf16(&range));
        Some(self.text[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<gpui::UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<std::ops::Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        _range: Option<std::ops::Range<usize>>,
        _text: &str,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        //todo!()
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range: Option<std::ops::Range<usize>>,
        _new_text: &str,
        _new_selected_range: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        //todo!()
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        element_bounds: gpui::Bounds<gpui::Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<gpui::Bounds<gpui::Pixels>> {
        // TODO
        Some(Bounds::from_corners(
            element_bounds.origin,
            element_bounds.origin,
        ))
    }

    fn character_index_for_point(
        &mut self,
        _point: gpui::Point<gpui::Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        // todo
        None
    }
}

impl Render for TextInput {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .id("input")
            .size_full()
            .when(self.multi_line, |this| this.h_auto())
            .items_center()
            .child(
                div()
                    .id("TextElement")
                    .when(self.multi_line, |this| this.h_auto())
                    .flex_grow()
                    .overflow_x_hidden()
                    .child(TextElement::new(cx.entity().clone())),
            )
            .when(self.multi_line, |this| {
                //let entity_id = cx.entity().entity_id();
                this.relative().child(
                    div().absolute().top_0().right_0().bottom_0().left_0(), //.child()
                )
            })
    }
}
