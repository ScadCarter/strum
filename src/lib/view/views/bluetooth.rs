use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub struct Props {
    pub size: Rect,
}

impl Props {
    pub fn new(size: Rect) -> Self {
        Self { size }
    }
}

pub fn render(props: &Props) -> super::Component<List> {
    let container = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(2)
        .horizontal_margin(1)
        .constraints([Constraint::Length(25), Constraint::Max(40)].as_ref())
        .split(props.size);

    let items = [
        ListItem::new("Device 1"),
        ListItem::new("Device 2"),
        ListItem::new("Device 3"),
    ];

    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::White).bg(Color::Black))
        .highlight_symbol(">>");

    super::Component::new(list, container[0])
}
