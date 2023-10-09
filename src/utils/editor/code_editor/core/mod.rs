mod color;
mod dashboard;
mod event;
mod history;
mod init;
mod line;
mod state;

use std::{fs, io, path::Path};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    exec::script::{run, run_entry},
    public::run_time::scope::Scope,
    utils::{editor::direction::Direction, number_bit_count, Cursor, Terminal},
};

use self::color::EditorColor;

use super::{
    components::{Component, EditorComponentManager, FileSaver, Finder, Positioner},
    cursor_pos::EditorCursorPos,
};

pub use state::EditorState;

use dashboard::EditorDashboard;
use event::{EditorEvent, EditorOperation};
use history::EditorHistory;
use init::EditorInit;
use line::EditorLine;

pub struct CodeEditor {
    lines: Vec<EditorLine>,
    index: usize, // current editing line index

    overflow_top: usize,
    overflow_bottom: usize,

    components: EditorComponentManager,
    history: EditorHistory,
    dashboard: EditorDashboard,
}

// base value calculating methods
impl CodeEditor {
    #[inline]
    fn label_width(&self) -> usize {
        // returns the longest line label width at left-side
        return number_bit_count(self.lines.len()) + 1;
    }

    #[inline]
    fn label_width_with(&self, value: usize) -> usize {
        // calculate label_width with inputed value
        return number_bit_count(value) + 1;
    }

    #[inline]
    fn visible_area_height(&self) -> usize {
        let term_height = Terminal::height();
        // `2` here means the top and bottom border.
        return term_height - 2;
    }
}

// editing methods
impl CodeEditor {
    fn render_all(&mut self) -> io::Result<()> {
        Cursor::save_pos()?;
        Cursor::move_to_row(1)?;
        Cursor::move_to_col(0)?;

        let label_width = self.label_width();
        let line_count = self.lines.len();

        let line_range = self.overflow_top..(line_count - self.overflow_bottom);
        let lines_to_render = &mut self.lines[line_range.clone()];
        let line_indices = line_range.map(|i| i + 1).collect::<Vec<usize>>();

        let iter = line_indices.into_iter().zip(lines_to_render.iter_mut());
        for (index, line) in iter {
            line.render(index, label_width)?;
            Cursor::down(1)?;
        }

        // initialize the unused lines
        let lines_to_render_count = lines_to_render.len();
        let visible_area_height = self.visible_area_height();
        if lines_to_render_count < visible_area_height {
            let diff = visible_area_height - lines_to_render_count;
            for _ in 0..diff {
                Cursor::move_to_col(0)?;
                Terminal::clear_after_cursor()?;
                print!("{}", " ".repeat(label_width).on_grey());
                Cursor::down(1)?;
            }
        }
        Cursor::restore_pos()?;
        return Ok(());
    }

    fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        let label_width = self.label_width();
        let line_count = self.lines.len();
        let current_line = &mut self.lines[self.index - 1];

        match dir {
            Direction::Left => {
                if current_line.is_at_line_start()? && self.index > 1 {
                    self.move_cursor_vertical(Direction::Up)?;
                    let current_line = self.lines.get_mut(self.index - 1).unwrap();
                    current_line.move_cursor_to_end(label_width)?;
                    return Ok(());
                }
            }
            Direction::Right => {
                if current_line.is_at_line_end()? && self.index < line_count {
                    self.move_cursor_vertical(Direction::Down)?;
                    let current_line = self.lines.get_mut(self.index - 1).unwrap();
                    current_line.move_cursor_to_start(label_width)?;
                    return Ok(());
                }
            }
            _ => unreachable!(),
        }
        current_line.move_cursor_horizontal(dir)?;
        return Ok(());
    }
    fn move_cursor_vertical(&mut self, dir: Direction) -> io::Result<()> {
        let is_at_first_line = self.index == 1;
        let is_at_last_line = self.index == self.lines.len();
        if (is_at_first_line && dir == Direction::Up) || (is_at_last_line && dir == Direction::Down)
        {
            return Ok(());
        }

        self.lines[self.index - 1].disable()?; // disable current line
        let (cursor_pos_row, cursor_pos_col) = (Cursor::pos_row()?, Cursor::pos_col()?);
        let label_width = self.label_width();
        let target_line = match dir {
            Direction::Up => {
                let is_at_top_side = cursor_pos_row == 1;
                self.index -= 1;
                if is_at_top_side {
                    self.overflow_top -= 1;
                    self.overflow_bottom += 1;
                } else {
                    Cursor::up(1)?;
                }
                self.lines.get_mut(self.index - 1).unwrap()
            }
            Direction::Down => {
                let is_at_bottom_side = cursor_pos_row == Terminal::height() - 2;
                self.index += 1;
                if is_at_bottom_side {
                    if self.lines.len() == self.visible_area_height() {
                        self.overflow_bottom += 1;
                    } else {
                        self.overflow_top += 1;
                        self.overflow_bottom -= 1;
                    }
                } else {
                    Cursor::down(1)?;
                }
                self.lines.get_mut(self.index - 1).unwrap()
            }
            _ => unreachable!(),
        };

        // if target_line is shorter than current line
        if cursor_pos_col > target_line.len() + label_width {
            Cursor::left(cursor_pos_col - label_width - target_line.len())?;
        }
        target_line.active()?;
        return Ok(());
    }

    fn insert_line(&mut self) -> io::Result<()> {
        let label_width = self.label_width_with(self.lines.len() + 1);
        let current_line = &mut self.lines[self.index - 1];
        let current_indent = current_line.indent_count();

        let is_at_line_end = current_line.is_at_line_end()?;
        let mut new_line = EditorLine::new(label_width, true);
        new_line.init_indent(current_indent);
        if !is_at_line_end {
            // when input Enter, if cursor is not at line end,
            // truncate current line and push truncated string
            // into the new line.
            let truncated_str = current_line.truncate()?;
            new_line.push_str(&truncated_str);
        }
        current_line.disable()?;
        new_line.move_cursor_after_indent(label_width)?;

        // insert new line
        let insert_pos = Cursor::pos_row()? + self.overflow_top;
        self.lines.insert(insert_pos, new_line);

        self.index += 1;
        // scroll
        if self.lines.len() > self.visible_area_height() {
            self.overflow_top += 1;
        } else {
            Cursor::down(1)?;
        }

        self.render_all()?;
        return Ok(());
    }

    fn insert_char(&mut self, ch: char) -> io::Result<()> {
        let current_line = &mut self.lines[self.index - 1];
        current_line.insert_char(ch)?;
        return Ok(());
    }

    fn delete_line(&mut self) -> io::Result<()> {
        let label_width = self.label_width_with(self.lines.len() - 1);
        let (previous_line, deleted_line) = {
            let remove_pos = Cursor::pos_row()? + self.overflow_top - 1;
            let removed_line = self.lines.remove(remove_pos);
            let previous_line = self.lines.get_mut(remove_pos - 1);
            (previous_line, removed_line)
        };
        if let Some(line) = previous_line {
            // avoid push indent in deleted line into previous line.
            let deleted_line_indent = deleted_line.indent_count();
            let remained_content = &deleted_line.content()[deleted_line_indent..];

            line.push_str(remained_content);
            line.move_cursor_to_end(label_width)?;
            line.active()?;

            for _ in 0..remained_content.len() {
                line.move_cursor_horizontal(Direction::Left)?;
            }
        }
        self.index -= 1;
        // scroll
        let is_overflowed = self.lines.len() >= self.visible_area_height();
        if is_overflowed && self.overflow_top > 0 {
            self.overflow_top -= 1;
        } else {
            Cursor::up(1)?;
        }
        // rerender
        self.render_all()?;
        return Ok(());
    }

    fn delete(&mut self) -> io::Result<()> {
        let cursor_pos = Cursor::pos_col()?;
        let label_width = self.label_width();
        if cursor_pos == label_width && self.index == 1 {
            // when at the start of the first line.
            return Ok(());
        }

        let pos_before = self.cursor_pos()?;
        let current_line = &mut self.lines[self.index - 1];

        if current_line.is_at_line_start()? {
            self.append_event(EditorOperation::DeleteLine, |e| e.delete_line())?;
        } else {
            let deleted_ch = current_line.delete_char()?;
            let pos_after = self.cursor_pos()?;

            let ch = deleted_ch.unwrap();
            self.history.append(EditorEvent {
                op: EditorOperation::DeleteChar(ch),
                pos_before,
                pos_after,
            });
        }
        return Ok(());
    }

    fn append_indent(&mut self) -> io::Result<()> {
        let current_line = &mut self.lines[self.index - 1];
        current_line.append_indent()?;
        return Ok(());
    }
    fn remove_indent(&mut self) -> io::Result<()> {
        let current_line = &mut self.lines[self.index - 1];
        current_line.remove_indent()?;
        return Ok(());
    }

    fn replace(&mut self, count: usize, to: &str) -> io::Result<()> {
        let current_line = &mut self.lines[self.index - 1];
        for _ in 0..count {
            current_line.move_cursor_horizontal(Direction::Right)?;
            current_line.delete_char()?;
        }

        for ch in to.chars().rev() {
            current_line.insert_char(ch)?;
            current_line.move_cursor_horizontal(Direction::Left)?;
        }
        return Ok(());
    }

    // --- --- --- --- --- ---

    fn exec_operation(&mut self, op: EditorOperation) -> io::Result<()> {
        match op {
            EditorOperation::InsertChar(ch) => self.insert_char(ch)?,
            EditorOperation::DeleteChar(_) => {
                let current_line = &mut self.lines[self.index - 1];
                current_line.delete_char()?;
            }
            EditorOperation::InsertLine => self.insert_line()?,
            EditorOperation::DeleteLine => self.delete_line()?,

            EditorOperation::AppendIndent => self.append_indent()?,
            EditorOperation::RemoveIndent => self.remove_indent()?,

            EditorOperation::Replace(from, to) => {
                self.replace(from.len(), to.as_str())?;
            }
        }
        return Ok(());
    }

    fn append_event(
        &mut self,
        op: EditorOperation,
        operation_callback: impl Fn(&mut CodeEditor) -> io::Result<()>,
    ) -> io::Result<()> {
        let pos_before = self.cursor_pos()?;
        operation_callback(self)?;
        let pos_after = self.cursor_pos()?;

        self.history.append(EditorEvent {
            op,
            pos_before,
            pos_after,
        });
        return Ok(());
    }

    fn undo(&mut self) -> io::Result<()> {
        if let Some(ev) = self.history.undo() {
            let target_pos = ev.pos_after;
            let target_op = ev.op.rev();

            self.jump_to(target_pos)?;
            self.exec_operation(target_op)?;
        }
        return Ok(());
    }
    fn redo(&mut self) -> io::Result<()> {
        if let Some(ev) = self.history.redo() {
            let target_pos = ev.pos_before;
            let target_op = ev.op.clone();

            self.jump_to(target_pos)?;
            self.exec_operation(target_op)?;
        }
        return Ok(());
    }
}

// cursor position controller
impl CodeEditor {
    fn cursor_pos(&self) -> io::Result<EditorCursorPos> {
        let current_line = &self.lines[self.index - 1];
        let col = current_line.cursor_pos()? + 1;
        let row = Cursor::pos_row()? + self.overflow_top;
        return Ok(EditorCursorPos { row, col });
    }

    fn check_cursor_pos(&self, pos: EditorCursorPos) -> bool {
        let EditorCursorPos { row, col } = pos;
        let is_row_overflow = row > self.lines.len();
        let is_col_overflow = if is_row_overflow {
            true
        } else {
            let target_line = &self.lines[row - 1];
            col == 0 || col > target_line.len() + 1
        };
        return !is_row_overflow && !is_col_overflow;
    }

    fn jump_to(&mut self, target_pos: EditorCursorPos) -> io::Result<()> {
        if target_pos == self.cursor_pos()? {
            return Ok(());
        }

        // move to target row
        let target_row = target_pos.row;
        let (dir, diff) = if target_row > self.index {
            (Direction::Down, target_row - self.index)
        } else {
            (Direction::Up, self.index - target_row)
        };
        for _ in 0..diff {
            self.move_cursor_vertical(dir)?;
        }
        // is not first and last line
        if self.index != 1 && self.index != self.lines.len() {
            self.move_cursor_vertical(dir)?;
            self.move_cursor_vertical(dir.rev())?;
        }

        // move to target col
        let label_width = self.label_width();
        let target_line = &mut self.lines[target_row - 1];
        target_line.move_cursor_to_start(label_width)?;
        for _ in 0..(target_pos.col - 1) {
            target_line.move_cursor_horizontal(Direction::Right)?;
        }
        self.render_all()?;
        return Ok(());
    }
}

// callback resolver methods
impl CodeEditor {
    // operate editor when using component.
    fn component_exec(
        &mut self,
        component_callback: impl Fn(&mut CodeEditor) -> io::Result<()>,
    ) -> io::Result<()> {
        Cursor::restore_pos()?;
        component_callback(self)?;
        Cursor::save_pos()?;

        // reopen current component
        match self.dashboard.state() {
            EditorState::Saving => self.components.file_saver.open()?,
            EditorState::Positioning => self.components.positioner.open()?,
            EditorState::Finding => self.components.finder.open()?,
            EditorState::Replacing => self.components.replacer.open()?,
            _ => unreachable!(),
        }
        return Ok(());
    }

    #[inline]
    fn dashboard_cursor_pos_refresh(&mut self) -> io::Result<()> {
        let current_cursor_pos = self.cursor_pos()?;
        self.dashboard.set_cursor_pos(current_cursor_pos)?;
        return Ok(());
    }

    fn callbacks_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.dashboard.state() {
            EditorState::Saving if FileSaver::is_save_callback_key(key) => {
                self.dashboard.set_state(EditorState::Saved)?;
            }
            EditorState::Positioning if Positioner::is_positioning_key(key) => {
                self.toggle_state(EditorState::Positioning)?;

                let target_pos = self.components.positioner.get_target();
                if self.check_cursor_pos(target_pos) {
                    self.jump_to(target_pos)?;
                    self.dashboard.set_cursor_pos(target_pos)?;
                }
            }
            EditorState::Finding => {
                let option_target_pos = if Finder::is_finding_key(key) {
                    if self.components.finder.is_empty() {
                        let target_text = self.components.finder.content();
                        if let Some(pos_list) = self.search(target_text) {
                            self.components.finder.set_matches(pos_list);
                        }
                    }
                    self.components.finder.next()
                } else if Finder::is_reverse_finding_key(key) {
                    self.components.finder.previous()
                } else {
                    None
                };

                if let Some(pos) = option_target_pos {
                    let pos = *pos;
                    self.component_exec(|e| e.jump_to(pos))?;
                }
            }
            EditorState::Replacing => {
                fn replace_pos_processor(
                    last_event: &EditorEvent,
                    current_pos: EditorCursorPos,
                    next_pos: &mut EditorCursorPos,
                ) {
                    let EditorEvent {
                        op: EditorOperation::Replace(from, to),
                        pos_before,
                        ..
                    } = last_event
                    else {
                        return;
                    };

                    if pos_before.row == next_pos.row {
                        let text_diff = from.len().abs_diff(to.len());
                        let col_diff = next_pos.col - current_pos.col;
                        next_pos.col = pos_before.col + col_diff;
                        if from.len() > to.len() {
                            next_pos.col -= text_diff;
                        } else {
                            next_pos.col += text_diff;
                        }
                    }
                }

                if self.components.replacer.is_search_key(key) {
                    let target_content = self.components.replacer.search_text();
                    if let Some(pos_list) = self.search(target_content) {
                        let replacer = &mut self.components.replacer;
                        replacer.search_handler(pos_list)?;

                        let first_target_pos = *replacer.first().unwrap();
                        self.component_exec(|e| e.jump_to(first_target_pos))?;
                    }
                } else if self.components.replacer.is_next_key(key) {
                    // jump to next position
                    let next_pos = self.components.replacer.next();
                    if let Some(pos) = next_pos {
                        let pos = *pos;
                        self.component_exec(|e| e.jump_to(pos))?;
                    }
                } else if self.components.replacer.is_replace_one_key(key) {
                    self.component_exec(|e| {
                        let replacer = &mut e.components.replacer;
                        let current_pos = *replacer.current();

                        if let Some(mut next_pos) = replacer.next().cloned() {
                            if let Some(ev) = e.history.previous_event() {
                                replace_pos_processor(ev, current_pos, &mut next_pos);
                            }

                            let replace_op = EditorOperation::Replace(
                                replacer.search_text().to_owned(),
                                replacer.replace_text().to_owned(),
                            );
                            let replace_count = replacer.search_text().len();
                            let replace_text = &replacer.replace_text().to_owned();
                            replacer.replace_handler();

                            e.jump_to(next_pos)?;
                            e.append_event(replace_op, |e| e.replace(replace_count, replace_text))?;
                        }
                        return Ok(());
                    })?;
                } else if self.components.replacer.is_replace_all_key(key) {
                    // close replacer
                    self.toggle_state(EditorState::Replacing)?;

                    let replacer = &mut self.components.replacer;
                    let replace_count = replacer.search_text().len();
                    let replace_text = &replacer.replace_text().to_owned();
                    let replace_op = EditorOperation::Replace(
                        replacer.search_text().to_owned(),
                        replacer.replace_text().to_owned(),
                    );
                    replacer.replace_handler();

                    let mut current_pos = *replacer.current();

                    while let Some(mut next_pos) = self.components.replacer.next().cloned() {
                        if let Some(ev) = self.history.previous_event() {
                            replace_pos_processor(ev, current_pos, &mut next_pos);
                        }

                        self.jump_to(next_pos)?;
                        self.append_event(replace_op.clone(), |e| {
                            e.replace(replace_count, replace_text)
                        })?;

                        current_pos = *self.components.replacer.current();
                    }
                }
            }
            _ => {}
        }
        return Ok(());
    }
}

// Non-editing methods
impl CodeEditor {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            index: 1,

            overflow_top: 0,
            overflow_bottom: 0,

            components: EditorComponentManager::new(),
            history: EditorHistory::new(),
            dashboard: EditorDashboard::new(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Cursor::move_to_row(0)?;
        Cursor::move_to_col(0)?;

        EditorInit::display_title();
        EditorInit::display_border()?;
        self.dashboard.render()?;

        // lines.is_empty() == true -> no file reading
        if self.lines.is_empty() {
            // `2` here is the width of line label ("1 ") in terminal.
            self.lines.push(EditorLine::new(2, true));
        }

        // move cursor to start of first line
        Cursor::move_to_row(1)?;
        let label_width = self.label_width();
        self.lines
            .first_mut()
            .unwrap()
            .move_cursor_to_start(label_width)?;

        self.render_all()?;
        return Ok(());
    }

    pub fn close(&self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        return Ok(());
    }

    #[inline]
    pub fn set_accent_color(color: &str) {
        EditorColor::set_accent_color(color);
    }

    pub fn read_file(&mut self, path: &str) -> io::Result<()> {
        self.components.file_saver.set_path(path);

        if !Path::new(path).exists() {
            return Ok(());
        }

        let file_read_res = fs::read_to_string(path);
        match file_read_res {
            Ok(content) => {
                let file_lines = content.lines();
                let line_count = file_lines.clone().count();
                let visible_area_height = self.visible_area_height();
                let label_width = self.label_width_with(line_count);

                // set `overflow_bottom`
                if line_count > visible_area_height {
                    self.overflow_bottom = line_count - visible_area_height;
                }

                self.lines = file_lines
                    .map(|l| {
                        let mut new_line = EditorLine::new(label_width, false);
                        new_line.push_str(l);
                        new_line
                    })
                    .collect();
            }
            Err(_) => {
                self.close()?;
                panic!("File reading failed!")
            }
        };
        return Ok(());
    }

    fn search(&self, target: &str) -> Option<Vec<EditorCursorPos>> {
        let mut result_pos_list = Vec::<EditorCursorPos>::new();
        for (index, line) in self.lines.iter().enumerate() {
            if let Some(pos_list) = line.find_all(target) {
                for pos in pos_list {
                    result_pos_list.push(EditorCursorPos {
                        row: index + 1,
                        col: pos + 1,
                    })
                }
            }
        }

        if !result_pos_list.is_empty() {
            return Some(result_pos_list);
        } else {
            return None;
        }
    }

    fn content(&self) -> String {
        let mut buf = String::new();
        let mut iter = self.lines.iter();
        while let Some(line) = iter.next() {
            buf += line.content();
            if iter.len() > 0 {
                buf += "\r\n";
            }
        }
        return buf;
    }

    // --- --- --- --- --- ---

    fn toggle_state(&mut self, new_state: EditorState) -> io::Result<()> {
        match self.dashboard.state() {
            // set mode
            EditorState::Saved | EditorState::Modified if !self.components.is_in_component => {
                Cursor::save_pos()?;
                self.components.is_in_component = true;
                self.dashboard.set_state(new_state)?;

                match new_state {
                    EditorState::Saving => {
                        let current_content = self.content();
                        let file_saver = &mut self.components.file_saver;
                        file_saver.set_content(current_content);
                        file_saver.open()?;
                    }
                    EditorState::Positioning => {
                        let current_cursor_pos = self.cursor_pos()?;
                        let positioner = &mut self.components.positioner;
                        positioner.set_cursor_pos(current_cursor_pos);
                        positioner.open()?;
                    }
                    EditorState::Finding => {
                        let finder = &mut self.components.finder;
                        finder.clear();
                        finder.open()?;
                    }
                    EditorState::Replacing => {
                        let replacer = &mut self.components.replacer;
                        replacer.reset();
                        replacer.open()?;
                    }
                    _ => unreachable!(),
                }
            }
            // restore to normal mode
            s if s == new_state && self.components.is_in_component => {
                // restore the covered line
                let label_width = self.label_width();
                let covered_pos = Cursor::pos_row()? + self.overflow_top - 1;
                match self.lines.get_mut(covered_pos) {
                    Some(covered_line) => covered_line.render(covered_pos + 1, label_width)?,
                    None => {
                        let label_width = self.label_width();
                        Cursor::move_to_col(0)?;
                        Terminal::clear_after_cursor()?;
                        print!("{}", " ".repeat(label_width).on_grey());
                    }
                }
                Cursor::restore_pos()?;
                self.dashboard.restore_state()?;
                self.components.is_in_component = false;
            }
            _ => {}
        }
        return Ok(());
    }

    pub fn cycle(&mut self, scope: &mut Scope) -> io::Result<()> {
        loop {
            let Some(key) = Terminal::get_key() else {
                continue;
            };

            // ctrl shotcuts
            if key.modifiers == KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Left | KeyCode::Right => {
                        let current_line = &mut self.lines[self.index - 1];
                        current_line.jump_to_word_edge(Direction::from(key.code))?;
                    }
                    KeyCode::Char(ch) => match ch {
                        'z' => self.undo()?,
                        'y' => self.redo()?,

                        's' => self.toggle_state(EditorState::Saving)?,
                        'g' => self.toggle_state(EditorState::Positioning)?,
                        'f' => self.toggle_state(EditorState::Finding)?,
                        'r' => self.toggle_state(EditorState::Replacing)?,

                        'e' => {
                            self.close()?;
                            let codes = self.content();
                            run_entry(&codes, scope, run);
                            return Ok(());
                        }
                        _ => {}
                    }

                    // ignore other Ctrl shotcuts
                    _ => {}
                }

                if !self.components.is_in_component {
                    // when is using component,
                    continue;
                }
            }

            if self.components.is_in_component {
                let current_state = self.dashboard.state();

                if key.code == KeyCode::Esc {
                    // use key `Esc` to restore to normal mode
                    self.toggle_state(current_state)?;
                    continue;
                }
                self.components.resolve(current_state, key)?;
                self.callbacks_resolve(key)?;
                continue;
            }

            // will enter matches in normal mode
            match key.code {
                // input `Escape` to exit
                KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Down => {
                    self.move_cursor_vertical(Direction::from(key.code))?;
                    self.render_all()?;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.move_cursor_horizontal(Direction::from(key.code))?;
                }

                KeyCode::Backspace
                | KeyCode::Enter
                | KeyCode::Tab
                | KeyCode::BackTab
                | KeyCode::Char(_) => {
                    self.dashboard.set_state(EditorState::Modified)?;
                    match key.code {
                        KeyCode::Backspace => self.delete()?,
                        KeyCode::Enter => {
                            self.append_event(EditorOperation::InsertLine, |e| e.insert_line())?
                        }
                        KeyCode::Tab => {
                            let current_line = &self.lines[self.index - 1];
                            if current_line.is_at_after_indent()? {
                                self.append_event(EditorOperation::AppendIndent, |e| {
                                    e.append_indent()
                                })?
                            }
                        }
                        KeyCode::BackTab => {
                            self.append_event(EditorOperation::RemoveIndent, |e| e.remove_indent())?
                        }

                        KeyCode::Char(ch) => {
                            if !ch.is_ascii() {
                                // avoid Non-ASCII characters
                                continue;
                            }
                            self.append_event(EditorOperation::InsertChar(ch), |e| {
                                e.insert_char(ch)
                            })?;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
            self.dashboard_cursor_pos_refresh()?;
        }
        self.close()?;
        return Ok(());
    }
}
