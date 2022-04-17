use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Tabs},
};

use crate::lib::view::Tab;

pub struct Props {
    pub tab: Tab,
    pub size: Rect,
}

impl Props {
    pub fn new(tab: Tab, size: Rect) -> Self {
        Self { tab, size }
    }
}

pub fn render(props: &Props) -> super::Component<Tabs> {
    let container = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(1)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Max(40)].as_ref())
        .split(props.size);

    let (selected_index, tabs): (usize, Vec<Spans>) =
        Tab::iter()
            .enumerate()
            .fold((0, vec![]), |mut current, (index, tab)| {
                if std::mem::discriminant(tab) == std::mem::discriminant(&props.tab) {
                    current.0 = index;
                }

                current
                    .1
                    .push(Spans::from(Span::styled(tab.to_string(), Style::default())));

                current
            });

    let tabs = Tabs::new(tabs)
        .block(Block::default())
        .select(selected_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::White).bg(Color::Black));

    super::Component::new(tabs, container[0])
}
