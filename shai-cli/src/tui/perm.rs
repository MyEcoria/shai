use std::time::Duration;

use ansi_to_tui::IntoText;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListDirection, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget},
    Frame
};
use shai_core::{agent::{events::PermissionRequest, output::PrettyFormatter, PermissionResponse}, tools::{ToolCall, ToolResult}};
// Removed tui_textarea dependency for colored preview

use super::theme::SHAI_YELLOW;

pub enum PermissionModalAction {
    Nope,
    Response {
        request_id: String,
        choice: PermissionResponse
    }
}

#[derive(Clone)]
pub struct PermissionWidget<'a> {
    pub request_id: String,
    pub request: PermissionRequest,
    pub remaining_perms: usize,

    selected_index: usize,
    formatted_request: String,
    preview_text: Text<'a>,
    scroll_offset: usize,
    scroll_state: ScrollbarState
}

impl PermissionWidget<'_> {
    pub fn new(request_id: String, request: PermissionRequest, total: usize) -> Self {
        let formatter = PrettyFormatter::new();
        let formatted_request = formatter.format_toolcall(&request.call, request.preview.as_ref());
        let preview_text = formatted_request.into_text().unwrap();
        let content_length = preview_text.lines.len();

        Self {
            request_id,
            request,
            selected_index: 0,
            remaining_perms: total,
            formatted_request,
            preview_text,
            scroll_offset: 0,
            scroll_state: ScrollbarState::new(content_length)
        }
    }


    pub fn move_up(&mut self) {
        self.selected_index = if self.selected_index == 0 { 2 } else { self.selected_index - 1 };
    }

    pub fn move_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % 3;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
        self.scroll_state = self.scroll_state.position(self.scroll_offset);
    }

    pub fn scroll_down(&mut self) {
        let max_scroll = self.preview_text.lines.len().saturating_sub(1);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
            self.scroll_state = self.scroll_state.position(self.scroll_offset);
        }
    }

    pub fn get_selected(&self) -> PermissionResponse {
        match self.selected_index {
            0 => PermissionResponse::Allow,
            1 => PermissionResponse::AllowAlways,
            2 => PermissionResponse::Deny,
            _ => PermissionResponse::Deny,
        }
    }

    pub async fn handle_mouse_event(&mut self, mouse_event: MouseEvent) ->  PermissionModalAction {
        // Handle mouse scroll in the preview area
        match mouse_event.kind {
            crossterm::event::MouseEventKind::ScrollUp => {
                self.scroll_up();
            }
            crossterm::event::MouseEventKind::ScrollDown => {
                self.scroll_down();
            }
            _ => {}
        }
        PermissionModalAction::Nope
    }

    pub async fn handle_key_event(&mut self, key_event: KeyEvent) ->  PermissionModalAction {
        match key_event.code {
            KeyCode::Up => {
                self.move_up();
                PermissionModalAction::Nope
            }
            KeyCode::Down => {
                self.move_down();
                PermissionModalAction::Nope
            }
            KeyCode::PageUp => {
                // Scroll preview up
                for _ in 0..5 {
                    self.scroll_up();
                }
                PermissionModalAction::Nope
            }
            KeyCode::PageDown => {
                // Scroll preview down
                for _ in 0..5 {
                    self.scroll_down();
                }
                PermissionModalAction::Nope
            }
            KeyCode::Enter => {
                let request_id = self.request_id.clone();
                let choice = self.get_selected();
                PermissionModalAction::Response { request_id, choice }
            }
            KeyCode::Esc => {
                let request_id = self.request_id.clone();
                let choice = PermissionResponse::Deny;
                PermissionModalAction::Response { request_id, choice }
            }
            _ => PermissionModalAction::Nope
        }
    }

    pub fn height(&self) -> u16 {
       4 // outer permission block 2 + 1 top padding
       + 2 // inner tool preview block 2 (0 padding)
       + self.preview_text.lines.len() as u16  // preview content
       + 4 // allow, yolo, deny + 1 top space
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding{left: 1, right: 1, top: 1, bottom: 1})
            .border_style(Style::default().fg(Color::Cyan))
            .title(if self.remaining_perms > 1 {
                format!(" 🔐 Permission Required ({}/{}) ", 1, self.remaining_perms)
            } else {
                format!(" 🔐 Permission Required ")
            });    

        let inner = block.inner(area);
        f.render_widget(block, area);

        let [tool, modal] = Layout::vertical([Constraint::Length(self.preview_text.lines.len() as u16 + 2), Constraint::Length(4)]).areas(inner);

        let call = self.request.call.clone();
        let tool_name = PrettyFormatter::capitalize_first(&call.tool_name);
        let context = PrettyFormatter::extract_primary_param(&call.parameters, &call.tool_name);
        let mut title = Line::from(vec![
            Span::styled("🔧 ", Color::White),
            Span::styled(tool_name,    Style::new().white().bold())
        ]);
        if let Some((_,ctx)) = context {
            title.push_span(Span::styled(format!("({})", ctx), Style::new().white()));
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding{left: 1, right: 1, top: 0, bottom: 0})
            .title(title)
            .title_style(Style::default().fg(Color::White))
            .border_style(Style::default().fg(Color::DarkGray));        
    
        let inner = block.inner(tool);
        f.render_widget(block, tool);

        // Render scrollable paragraph with colors
        let paragraph = Paragraph::new(self.preview_text.clone())
            .scroll((self.scroll_offset as u16, 0));
        f.render_widget(paragraph, inner);

        // Render scrollbar if content is longer than area
        if self.preview_text.lines.len() > inner.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(Color::DarkGray));
            f.render_stateful_widget(scrollbar, inner, &mut self.scroll_state.clone());
        }

        let items = ["Allow", "Allow all tools and don't ask again for this session", "Deny"];
        let mut lines = vec![Line::from("Do you want to run this tool?")];
        for (i,s) in items.into_iter().enumerate() {
            if i == self.selected_index {
                lines.push(Line::from(vec![
                    Span::styled("❯ ", Color::White),
                    Span::styled(s,    Color::White)
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  ", Color::DarkGray),
                    Span::styled(s,    Color::DarkGray)
                ]));
            };
        }
        let text = Text::from(lines);
        let p = Paragraph::new(text);
        f.render_widget(p, modal);
    }
}
