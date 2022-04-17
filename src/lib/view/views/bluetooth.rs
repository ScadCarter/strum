use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::lib::state::State;

pub struct Props {
    pub size: Rect,
    pub state: Box<State>,
}

impl Props {
    pub fn new(size: Rect, state: Box<State>) -> Self {
        Self { size, state }
    }
}

pub fn render(props: &Props) -> super::Component<List> {
    let container = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(2)
        .horizontal_margin(2)
        .constraints([Constraint::Length(25), Constraint::Max(40)].as_ref())
        .split(props.size);

    let items: Vec<ListItem> = props
        .state
        .devices
        .iter()
        // TODO: highlight current selected item
        // TODO: state holds the current selected item
        // TODO: previous state as well carries over
        .map(|device| ListItem::new(&*device.name))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::TOP))
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::White).bg(Color::Black))
        .highlight_symbol(">>");

    super::Component::new(list, container[0])
}
