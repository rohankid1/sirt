use color_eyre::Result;
use libsirt::Block as SirtBlock;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::sirt::BlockItem;

#[derive(Debug)]
pub struct App {
    block_list: BlockList,
    exit: bool,
}

#[derive(Debug)]
struct BlockList {
    blocks: Vec<BlockItem>,
    state: ListState,
}

impl App {
    pub fn new(blocks: Vec<SirtBlock>) -> Self {
        let block_items = blocks
            .into_iter()
            .map(|block| BlockItem {
                block,
                description: None,
                syntax: None,
            })
            .collect();

        Self {
            block_list: BlockList {
                blocks: block_items,
                state: ListState::default(),
            },
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn select_none(&mut self) {
        self.block_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.block_list.state.select_next();
    }

    fn select_prev(&mut self) {
        self.block_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.block_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.block_list.state.select_last();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&mut self, f: &mut Frame) {
        f.render_widget(self, f.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(ke) if ke.kind == KeyEventKind::Press => return self.handle_key_events(ke),
            _ => {}
        };

        Ok(())
    }

    fn handle_key_events(
        &mut self,
        ke: event::KeyEvent,
    ) -> std::result::Result<(), color_eyre::eyre::Error> {
        match ke.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_prev(),
            KeyCode::Char('h') | KeyCode::Left => self.select_first(),
            KeyCode::Char('l') | KeyCode::Right => self.select_last(),
            KeyCode::Esc => self.select_none(),
            _ => {}
        }

        Ok(())
    }

    fn render_block_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::raw("Blocks").centered())
            .border_set(border::ROUNDED);

        let items: Vec<ListItem> = self
            .block_list
            .blocks
            .iter()
            .map(|block_item| ListItem::new(block_item.block.get_name()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.block_list.state);
    }

    fn render_block_description(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::raw("Description").centered())
            .title_bottom(Line::from(vec![
                "Sel None ".blue(),
                "<Esc> ".blue().bold(),
                "Sel Down ".yellow(),
                "<J>/<Down> ".yellow().bold(),
                "Sel Up ".green(),
                "<K>/<Up> ".green().bold(),
                "Sel First ".gray(),
                "<H>/<Left> ".gray().bold(),
                "Sel Last ".gray(),
                "<L>/<Right> ".gray().bold(),
                "Quit ".red(),
                "<Q>".red().bold(),
            ]))
            .border_set(border::ROUNDED)
            .padding(Padding::horizontal(2));

        let info = if let Some(block_idx) = self.block_list.state.selected() {
            let block_item = &mut self.block_list.blocks[block_idx];
            let item_desc = block_item.item_description().to_owned();
            &format!("{item_desc}\n\n\n{}", block_item.item_syntax())
        } else {
            "Select a Block for analysis"
        };

        Paragraph::new(info)
            .wrap(Wrap { trim: true })
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [a, b] = Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(area);

        self.render_block_list(a, buf);
        self.render_block_description(b, buf);
    }
}
