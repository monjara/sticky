use std::ops::Range;

use gpui::{
    App, AppContext, Bounds, Context, DefiniteLength, Entity, EntityInputHandler, EventEmitter,
    FocusHandle, Focusable, InteractiveElement, KeyBinding, KeyDownEvent, MouseMoveEvent,
    ParentElement, Pixels, Point, Render, ScrollHandle, SharedString, Styled, Subscription,
    UTF16Selection, Window, WrappedLine, actions, div, point, prelude::FluentBuilder, px, relative,
};
use smallvec::SmallVec;
use unicode_segmentation::UnicodeSegmentation;

use super::{blink_cursor::BlinkCursor, text_element::TextElement};

actions!(
    input,
    [
        Backspace,
        Delete,
        DeleteToBeginningOfLine,
        DeleteToEndOfLine,
        DeleteToPreviousWordStart,
        DeleteToNextWordEnd,
        Enter,
        Up,
        Down,
        Left,
        Right,
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        SelectToStartOfLine,
        SelectToEndOfLine,
        SelectToStart,
        SelectToEnd,
        SelectToPreviousWordStart,
        SelectToNextWordEnd,
        ShowCharacterPalette,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo,
        MoveToStartOfLine,
        MoveToEndOfLine,
        MoveToStart,
        MoveToEnd,
        MoveToPreviousWord,
        MoveToNextWord,
        TextChanged,
    ]
);

const CONTEXT: &str = "Input";

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-backspace", DeleteToBeginningOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-delete", DeleteToEndOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-backspace", DeleteToPreviousWordStart, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-backspace", DeleteToPreviousWordStart, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-delete", DeleteToNextWordEnd, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-delete", DeleteToNextWordEnd, Some(CONTEXT)),
        KeyBinding::new("enter", Enter, Some(CONTEXT)),
        KeyBinding::new("up", Up, Some(CONTEXT)),
        KeyBinding::new("down", Down, Some(CONTEXT)),
        KeyBinding::new("left", Left, Some(CONTEXT)),
        KeyBinding::new("right", Right, Some(CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
        KeyBinding::new("shift-up", SelectUp, Some(CONTEXT)),
        KeyBinding::new("shift-down", SelectDown, Some(CONTEXT)),
        KeyBinding::new("home", Home, Some(CONTEXT)),
        KeyBinding::new("end", End, Some(CONTEXT)),
        KeyBinding::new("shift-home", SelectToStartOfLine, Some(CONTEXT)),
        KeyBinding::new("shift-end", SelectToEndOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-shift-a", SelectToStartOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-shift-e", SelectToEndOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("shift-cmd-left", SelectToStartOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("shift-cmd-right", SelectToEndOfLine, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-shift-left", SelectToPreviousWordStart, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-left", SelectToPreviousWordStart, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-shift-right", SelectToNextWordEnd, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-right", SelectToNextWordEnd, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-a", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", Home, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-e", End, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", End, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-z", Undo, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-z", Redo, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-up", MoveToStart, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-down", MoveToEnd, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-left", MoveToPreviousWord, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("alt-right", MoveToNextWord, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-left", MoveToPreviousWord, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-right", MoveToNextWord, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-up", SelectToStart, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-down", SelectToEnd, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-z", Undo, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-y", Redo, Some(CONTEXT)),
    ]);
}

pub enum InputEvent {
    Change(SharedString),
    PressEnter,
    Focus,
    Blur,
}

pub struct TextInput {
    pub(super) focus_handle: FocusHandle,
    pub(super) text: SharedString,
    multi_line: bool,
    pub(super) blink_cursor: Entity<BlinkCursor>,
    pub(super) placeholder: SharedString,
    pub(super) selected_range: Range<usize>,
    /// Range for save the selected word, use to keep word range when drag move.
    pub(super) selected_word_range: Option<Range<usize>>,
    pub(super) selection_reversed: bool,
    pub(super) marked_range: Option<Range<usize>>,
    pub(super) last_layout: Option<SmallVec<[WrappedLine; 1]>>,
    pub(super) last_cursor_offset: Option<usize>,
    /// The line_height of text layout, this will change will InputElement painted.
    pub(super) last_line_height: Pixels,
    /// The input container bounds
    pub(super) input_bounds: Bounds<Pixels>,
    /// The text bounds
    pub(super) last_bounds: Option<Bounds<Pixels>>,
    pub(super) last_selected_range: Option<Range<usize>>,
    pub(super) masked: bool,
    pub(super) rows: usize,
    /// For special case, e.g.: NumberInput + - button
    pub(super) height: Option<gpui::DefiniteLength>,
    pub scroll_handle: ScrollHandle,
    //scrollbar_state: Rc<Cell<ScrollbarState>>,
    /// The size of the scrollable content.
    pub(crate) scroll_size: gpui::Size<Pixels>,
    /// To remember the horizontal column (x-coordinate) of the cursor position.
    preferred_x_offset: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InputEvent> for TextInput {}

impl TextInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let blink_cursor = cx.new(|_| BlinkCursor::new());
        //let history = History::new().group_interval(std::time::Duration::from_secs(1));

        let _subscriptions = vec![
            // Observe the blink cursor to repaint the view when it changes.
            cx.observe(&blink_cursor, |_, _, cx| cx.notify()),
            // Blink the cursor when the window is active, pause when it's not.
            cx.observe_window_activation(window, |input, window, cx| {
                if window.is_window_active() {
                    let focus_handle = input.focus_handle.clone();
                    if focus_handle.is_focused(window) {
                        input.blink_cursor.update(cx, |blink_cursor, cx| {
                            blink_cursor.start(cx);
                        });
                    }
                }
            }),
            cx.on_focus(&focus_handle, window, Self::on_focus),
            cx.on_blur(&focus_handle, window, Self::on_blur),
        ];

        Self {
            focus_handle: focus_handle.clone(),
            text: "".into(),
            multi_line: false,
            blink_cursor,
            placeholder: "".into(),
            selected_range: 0..0,
            selected_word_range: None,
            selection_reversed: false,
            marked_range: None,
            input_bounds: Bounds::default(),
            masked: false,
            height: None,
            rows: 2,
            last_layout: None,
            last_bounds: None,
            last_selected_range: None,
            last_line_height: px(20.),
            last_cursor_offset: None,
            scroll_handle: ScrollHandle::new(),
            scroll_size: gpui::size(px(0.), px(0.)),
            preferred_x_offset: None,
            _subscriptions,
        }
    }

    pub fn multi_line(mut self) -> Self {
        self.multi_line = true;
        self
    }

    pub fn h_full(mut self) -> Self {
        self.height = Some(relative(1.));
        self
    }

    pub fn h(mut self, height: impl Into<DefiniteLength>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn set_text(
        &mut self,
        text: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text: SharedString = text.into();
        let range = 0..self.text.chars().map(|c| c.len_utf16()).sum();
        self.replace_text_in_range(Some(range), &text, window, cx);
    }

    pub fn is_single_line(&self) -> bool {
        !self.multi_line
    }

    pub fn is_multi_line(&self) -> bool {
        self.multi_line
    }

    pub fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    pub fn show_cursor(&self, window: &Window, cx: &App) -> bool {
        self.focus_handle.is_focused(window) && self.blink_cursor.read(cx).visible()
    }

    pub fn on_drag_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.text.is_empty() {
            return;
        }

        if self.last_layout.is_none() {
            return;
        }

        if !self.focus_handle.is_focused(window) {
            return;
        }

        // if !self.selecting {
        //     return;
        // }

        let offset = self.index_for_mouse_position(event.position, window, cx);
        self.select_to(offset, window, cx);
    }

    fn index_for_mouse_position(
        &self,
        position: Point<Pixels>,
        _window: &Window,
        _cx: &App,
    ) -> usize {
        // If the text is empty, always return 0
        if self.text.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(lines)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };

        let line_height = self.last_line_height;

        // TIP: About the IBeam cursor
        //
        // If cursor style is IBeam, the mouse mouse position is in the middle of the cursor (This is special in OS)

        // The position is relative to the bounds of the text input

        //
        // - included the input padding.
        // - included the scroll offset.
        let inner_position = position - bounds.origin;

        let mut index = 0;
        let mut y_offset = px(0.);

        for line in lines.iter() {
            let line_origin = self.line_origin_with_y_offset(&mut y_offset, line, line_height);
            let pos = inner_position - line_origin;
            let closest_index = line.unwrapped_layout.closest_index_for_x(pos.x);

            // Return offset by use closest_index_for_x if is single line mode.
            if self.is_single_line() {
                return closest_index;
            }

            let index_result = line.closest_index_for_position(pos, line_height);
            if let Ok(v) = index_result {
                index += v;
                break;
            } else if let Ok(_) = line.index_for_position(point(px(0.), pos.y), line_height) {
                // Click in the this line but not in the text, move cursor to the end of the line.
                // The fallback index is saved in Err from `index_for_position` method.
                index += index_result.unwrap_err();
                break;
            } else if line.len() == 0 {
                // empty line
                let line_bounds = Bounds {
                    origin: line_origin,
                    size: gpui::size(bounds.size.width, line_height),
                };
                let pos = inner_position;
                if line_bounds.contains(&pos) {
                    break;
                }
            } else {
                index += line.len();
            }

            // add 1 for \n
            index += 1;
        }

        if index > self.text.len() {
            self.text.len()
        } else {
            index
        }
    }

    fn line_origin_with_y_offset(
        &self,
        y_offset: &mut Pixels,
        line: &WrappedLine,
        line_height: Pixels,
    ) -> Point<Pixels> {
        // NOTE: About line.wrap_boundaries.len()
        //
        // If only 1 line, the value is 0
        // If have 2 line, the value is 1
        if self.is_multi_line() {
            let p = point(px(0.), *y_offset);
            let height = line_height + line.wrap_boundaries.len() as f32 * line_height;
            *y_offset = *y_offset + height;
            p
        } else {
            point(px(0.), px(0.))
        }
    }

    fn on_focus(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.blink_cursor.update(cx, |cursor, cx| {
            cursor.start(cx);
        });
        cx.emit(InputEvent::Focus);
    }

    fn on_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.unselect(window, cx);
        self.blink_cursor.update(cx, |cursor, cx| {
            cursor.stop(cx);
        });
        cx.emit(InputEvent::Blur);
    }

    fn unselect(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let offset = self.next_boundary(self.cursor_offset());
        self.selected_range = offset..offset;
        cx.notify();
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.text.len())
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

    fn line_and_position_for_offset(
        &self,
        offset: usize,
        lines: &[WrappedLine],
        line_height: Pixels,
    ) -> (usize, usize, Option<Point<Pixels>>) {
        let mut prev_lines_offset = 0;
        let mut y_offset = px(0.);
        for (line_index, line) in lines.iter().enumerate() {
            let local_offset = offset.saturating_sub(prev_lines_offset);
            if let Some(pos) = line.position_for_index(local_offset, line_height) {
                let sub_line_index = (pos.y.0 / line_height.0) as usize;
                let adjusted_pos = point(pos.x, pos.y + y_offset);
                return (line_index, sub_line_index, Some(adjusted_pos));
            }

            y_offset += line.size(line_height).height;
            prev_lines_offset += line.len() + 1;
        }
        (0, 0, None)
    }

    fn update_preferred_x_offset(&mut self, _cx: &mut Context<Self>) {
        if let (Some(lines), Some(bounds)) = (&self.last_layout, &self.last_bounds) {
            let offset = self.cursor_offset();
            let line_height = self.last_line_height;

            // Find which line and sub-line the cursor is on and its position
            let (_line_index, _sub_line_index, cursor_pos) =
                self.line_and_position_for_offset(offset, lines, line_height);

            if let Some(pos) = cursor_pos {
                // Adjust by scroll offset
                let scroll_offset = bounds.origin;
                self.preferred_x_offset = Some(pos.x + scroll_offset.x);
            }
        }
    }

    fn select_to(&mut self, offset: usize, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };

        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }

        // Ensure keep word selected range
        if let Some(word_range) = self.selected_word_range.as_ref() {
            if self.selected_range.start > word_range.start {
                self.selected_range.start = word_range.start;
            }
            if self.selected_range.end < word_range.end {
                self.selected_range.end = word_range.end;
            }
        }
        if self.selected_range.is_empty() {
            self.update_preferred_x_offset(cx);
        }
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            //
            self.select_to(self.previous_boundary(self.cursor_offset()), window, cx)
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), window, cx)
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete_beginning_of_line(
        &mut self,
        _: &DeleteToBeginningOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let offset = self.cursor_offset();
        let line_start = self.text[..offset].rfind('\n').map_or(0, |i| i + 1);
        self.replace_text_in_range(Some(line_start..offset), "", window, cx);
    }

    fn delete_end_of_line(
        &mut self,
        _: &DeleteToEndOfLine,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let offset = self.cursor_offset();
        let line_end = self.text[offset..]
            .find('\n')
            .map_or(self.text.len(), |i| offset + i);
        self.replace_text_in_range(Some(offset..line_end), "", window, cx);
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_text_in_range(None, "\n", window, cx);
        cx.emit(InputEvent::PressEnter);
    }

    fn up(&mut self, _: &Up, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_single_line() {
            return;
        }
        self.move_vertical(-1, window, cx);
    }

    fn down(&mut self, _: &Down, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_single_line() {
            return;
        }
        self.move_vertical(1, window, cx)
    }
    fn left(&mut self, _: &Left, window: &mut Window, cx: &mut Context<Self>) {
        self.pause_blink_cursor(cx);
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), window, cx);
        } else {
            self.move_to(self.selected_range.start, window, cx)
        }
    }

    fn right(&mut self, _: &Right, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), window, cx);
        } else {
            self.move_to(self.selected_range.end, window, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, window: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), window, cx);
    }

    fn select_right(&mut self, _: &SelectRight, window: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), window, cx);
    }

    fn select_up(&mut self, _: &SelectUp, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_single_line() {
            return;
        }
        let offset = self.start_of_line(window, cx).saturating_sub(1);
        self.select_to(offset, window, cx);
    }

    fn select_down(&mut self, _: &SelectDown, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_single_line() {
            return;
        }
        let offset = (self.end_of_line(window, cx) + 1).min(self.text.len());
        self.select_to(self.next_boundary(offset), window, cx);
    }

    fn start_of_line(&mut self, window: &mut Window, cx: &mut Context<Self>) -> usize {
        if self.is_single_line() {
            return 0;
        }

        let offset = self.previous_boundary(self.cursor_offset());
        self.text_for_range(self.range_to_utf16(&(0..offset + 1)), &mut None, window, cx)
            .unwrap_or_default()
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0)
    }

    /// Get end of line
    fn end_of_line(&mut self, window: &mut Window, cx: &mut Context<Self>) -> usize {
        if self.is_single_line() {
            return self.text.len();
        }

        let offset = self.next_boundary(self.cursor_offset());
        // ignore if offset is "\n"
        if self
            .text_for_range(
                self.range_to_utf16(&(offset - 1..offset)),
                &mut None,
                window,
                cx,
            )
            .unwrap_or_default()
            .eq("\n")
        {
            return offset;
        }

        self.text_for_range(
            self.range_to_utf16(&(offset..self.text.len())),
            &mut None,
            window,
            cx,
        )
        .unwrap_or_default()
        .find('\n')
        .map(|i| i + offset)
        .unwrap_or(self.text.len())
    }

    fn move_vertical(&mut self, direction: i32, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_single_line() {
            return;
        }

        let (Some(lines), Some(bounds)) = (&self.last_layout, &self.last_bounds) else {
            return;
        };

        let offset = self.cursor_offset();
        let line_height = self.last_line_height;

        let (line_index, sub_line_index, pos) =
            self.line_and_position_for_offset(offset, lines, line_height);

        let Some(pos) = pos else {
            return;
        };

        let current_x = self
            .preferred_x_offset
            .unwrap_or_else(|| pos.x + bounds.origin.x);

        let mut new_line_index = line_index;
        let mut new_sub_line = sub_line_index as i32;

        new_sub_line += direction;

        let is_first_line = new_line_index == 0 && new_sub_line < 0;
        if direction == -1 && is_first_line {
            // Move cursor to the beginning of the text
            self.move_to(0, window, cx);
            return;
        }

        let is_last_line = new_line_index == lines.len() - 1
            && new_sub_line > lines[new_line_index].wrap_boundaries.len() as i32;

        if direction == 1 && is_last_line {
            // Move cursor to the end of the text
            self.move_to(self.text.len(), window, cx);
            return;
        }

        if new_sub_line < 0 {
            if new_line_index > 0 {
                new_line_index -= 1;
                new_sub_line = lines[new_line_index].wrap_boundaries.len() as i32;
            } else {
                new_sub_line = 0;
            }
        } else {
            let max_sub_line = lines[new_line_index].wrap_boundaries.len() as i32;
            if new_sub_line > max_sub_line {
                if new_line_index < lines.len() - 1 {
                    new_line_index += 1;
                    new_sub_line = 0;
                } else {
                    new_sub_line = max_sub_line;
                }
            }
        }

        if new_line_index == line_index && new_sub_line == sub_line_index as i32 {
            return;
        }

        let target_line = &lines[new_line_index];
        let line_x = current_x - bounds.origin.x;
        let target_sub_line = new_sub_line as usize;

        let approx_pos = point(line_x, px(target_sub_line as f32 * line_height.0));
        let index_res = target_line.index_for_position(approx_pos, line_height);

        let new_local_index = match index_res {
            Ok(i) => i + 1,
            Err(i) => i,
        };

        let mut prev_lines_offset = 0;
        for (i, l) in lines.iter().enumerate() {
            if i == new_line_index {
                break;
            }
            prev_lines_offset += l.len() + 1;
        }

        let new_offset = (prev_lines_offset + new_local_index).min(self.text.len());
        self.selected_range = new_offset..new_offset;
        self.pause_blink_cursor(cx);

        cx.notify();
    }

    fn move_to(&mut self, offset: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.pause_blink_cursor(cx);
        self.update_preferred_x_offset(cx);
        cx.notify();
    }

    fn pause_blink_cursor(&mut self, cx: &mut Context<Self>) {
        self.blink_cursor.update(cx, |this, cx| {
            this.pause(cx);
        })
    }

    fn on_key_down_for_blink_cursor(
        &mut self,
        _: &KeyDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pause_blink_cursor(cx);
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
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
        range: Option<std::ops::Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let pending_text: SharedString =
            (self.text[0..range.start].to_owned() + text + &self.text[range.end..]).into();

        //if !self.is_valid_input(&pending_text) {
        //    return;
        //}

        //self.push_history(&range, new_text, window, cx);
        self.text = pending_text;
        self.selected_range = range.start + text.len()..range.start + text.len();
        self.marked_range.take();
        //self.update_preferred_x_offset(cx);

        cx.emit(InputEvent::Change(self.text.clone()));
        cx.notify();
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
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::delete_beginning_of_line))
            .on_action(cx.listener(Self::delete_end_of_line))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_key_down(cx.listener(Self::on_key_down_for_blink_cursor))
            .size_full()
            .items_center()
            .when(self.is_multi_line(), |this| {
                this.h_auto()
                    .when_some(self.height, |this, height| this.h(height))
            })
            .child(
                div()
                    .id("TextElement")
                    .when(self.multi_line, |this| this.h_full())
                    .flex_grow()
                    .overflow_x_hidden()
                    .child(TextElement::new(cx.entity().clone())),
            )
            .when(self.multi_line, |this| {
                this.relative()
                    .child(div().absolute().top_0().left_0().h_full().w_full())
            })
    }
}
