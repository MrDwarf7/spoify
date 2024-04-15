use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

use super::util::help_table_ui;

pub fn render_default_help(f: &mut Frame, header_chunk: &[Rect], app: &mut App) {
    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Help"))
        .border_style(Style::default())
        .style(Style::default().bg(app.background_color));

    let mut help_panel_vec = Vec::new();
    let _var = help_panel_vec;

    help_panel_vec = vec![Line::from(vec![Span::raw("Type ?")])];

    let help_panel = Paragraph::new(help_panel_vec)
        .wrap(Wrap { trim: true })
        .block(help_block);

    f.render_widget(help_panel, header_chunk[1]);
}

pub fn render_help(f: &mut Frame, app: &mut App) {
    f.render_widget(Clear, f.size());

    let help_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Help"))
        .border_style(Style::new().fg(app.border_color))
        .style(Style::default().bg(app.background_color));

    let help_table = help_table_ui(
        app.tasks.clone(),
        app.first_keys.clone(),
        help_block,
        app.highlight_color.clone(),
        app.background_color.clone(),
    );

    f.render_widget(help_table, f.size());
}
