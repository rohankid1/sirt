use crate::sirt::BlockItem;
use iced::{
    Element, Font,
    Length::{self, Fill, FillPortion},
    Padding, Task, Theme,
    font::Weight,
    highlighter::Theme as HighlighterTheme,
    widget::{
        self, Column, button, column, container, pick_list, row, scrollable, text,
        text_editor::{self, Action, Edit},
    },
};
use libsirt::Block as SirtBlock;

#[derive(Debug, Clone)]
enum Message {
    Exit,
    CopyCode,
    CopyDetails,
    Edit(Action),
    EditSyn(Action),
    SetTheme(Theme),
    SetSyntaxTheme(HighlighterTheme),
    ChooseBlock(usize),
}

#[derive(Debug, Clone)]
struct App {
    theme: Theme,
    highlighter: HighlighterTheme,
    blocks: Vec<BlockItem>,
    repr_editor: text_editor::Content,
    syntax_editor: text_editor::Content,
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::CatppuccinMacchiato,
            highlighter: HighlighterTheme::SolarizedDark,
            blocks: vec![],
            repr_editor: text_editor::Content::new(),
            syntax_editor: text_editor::Content::new(),
        }
    }
}

impl App {
    fn new(blocks: Vec<SirtBlock>) -> Self {
        let blocks = blocks
            .into_iter()
            .map(|block| BlockItem {
                block,
                description: None,
                syntax: None,
            })
            .collect();

        Self {
            blocks,
            ..Default::default()
        }
    }

    fn clear_repr_editor(&mut self) {
        self.repr_editor = text_editor::Content::new();
    }

    fn clear_syn_editor(&mut self) {
        self.syntax_editor = text_editor::Content::new()
    }

    fn clear_all(&mut self) {
        self.clear_repr_editor();
        self.clear_syn_editor();
    }

    fn title(&self) -> String {
        "Sirt Analyzer".into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTheme(theme) => {
                self.theme = theme;
            }
            Message::SetSyntaxTheme(theme) => {
                self.highlighter = theme;
            }
            Message::Exit => return iced::exit(),
            Message::CopyCode => {
                return iced::clipboard::write(self.repr_editor.text().trim().to_string());
            }
            Message::CopyDetails => {
                return iced::clipboard::write(self.syntax_editor.text().trim().to_string());
            }
            Message::Edit(act) => match act {
                Action::Click(_)
                | Action::Drag(_)
                | Action::Move(_)
                | Action::Select(_)
                | Action::Scroll { lines: _ } => self.repr_editor.perform(act),
                _ => {}
            },
            Message::EditSyn(act) => match act {
                Action::Click(_)
                | Action::Drag(_)
                | Action::Select(_)
                | Action::Scroll { lines: _ } => self.syntax_editor.perform(act),
                _ => {}
            },
            Message::ChooseBlock(idx) => {
                self.clear_all();
                let block_item = &mut self.blocks[idx];
                self.repr_editor
                    .perform(Action::Edit(Edit::Paste(std::sync::Arc::new(
                        block_item.item_description().to_string(),
                    ))));
                self.syntax_editor
                    .perform(Action::Edit(Edit::Paste(std::sync::Arc::new(
                        block_item.item_syntax().to_string(),
                    ))));
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let nav = column![
            row![
                button("Exit").on_press(Message::Exit),
                widget::space().width(Length::Fill),
                container(
                    row![
                        column![
                            text("Theme"),
                            pick_list(Theme::ALL, Some(&self.theme), Message::SetTheme)
                                .placeholder(self.theme.to_string()),
                        ]
                        .spacing(2.5),
                        column![
                            text("Highligher"),
                            pick_list(
                                HighlighterTheme::ALL,
                                Some(&self.highlighter),
                                Message::SetSyntaxTheme
                            ),
                        ]
                        .spacing(2.5)
                    ]
                    .spacing(10)
                ),
            ]
            .padding(10),
            colored_border()
        ]
        .padding(Padding::default().bottom(5));

        let elements: Vec<Element<'_, Message>> = self
            .blocks
            .iter()
            .enumerate()
            .map(|(idx, element)| {
                button(text(element.block.get_name()).wrapping(text::Wrapping::Glyph))
                    .padding(5)
                    .on_press(Message::ChooseBlock(idx))
                    .into()
            })
            .collect();

        let program_version = env!("CARGO_PKG_VERSION");
        let program_name = "sirt analyzer";

        let side_bar = container(column![
            text("Blocks").size(30.0).font(Font {
                weight: Weight::Bold,
                ..Default::default()
            }),
            scrollable(Column::from_vec(elements).spacing(12))
                .width(Fill)
                .height(Fill),
            container(
                text(format!("{program_name} - {program_version}"))
                    .size(15.0)
                    .align_x(widget::text::Alignment::Center),
            )
            .center_x(Fill)
            .padding(Padding::default().bottom(10).top(10))
        ])
        .padding(Padding::default().left(10))
        .width(FillPortion(1))
        .height(Fill)
        .style(container::rounded_box);

        let editors = container(
            column![
                row![
                    button("Copy Rust").on_press(Message::CopyCode),
                    button("Copy Details").on_press(Message::CopyDetails)
                ]
                .spacing(5),
                widget::text_editor(&self.repr_editor)
                    .font(iced::Font::MONOSPACE)
                    .on_action(Message::Edit)
                    .height(Length::Fill)
                    .highlight("rust", self.highlighter),
                widget::text_editor(&self.syntax_editor)
                    .font(iced::Font::MONOSPACE)
                    .on_action(Message::EditSyn)
                    .height(Length::Fill)
                    .highlight("rust", self.highlighter)
            ]
            .spacing(5),
        )
        .width(Length::FillPortion(3))
        .height(Length::Fill);

        container(column![nav, row![side_bar, editors]]).into()
    }
}

fn colored_border<'a>() -> widget::Container<'a, Message> {
    container(widget::space::horizontal())
        .height(1.0)
        .style(move |theme: &Theme| {
            widget::container::background(iced::Background::from(
                theme.extended_palette().primary.base.color,
            ))
        })
}

pub fn run(blocks: Vec<crate::Block>) -> iced::Result {
    let app = App::new(blocks);
    iced::application(move || app.clone(), App::update, App::view)
        .theme(App::theme)
        .title(App::title)
        .run()
}
