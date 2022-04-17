pub mod bluetooth;
pub mod header;

pub struct Component<W: tui::widgets::Widget> {
    pub widget: W,
    pub rect: tui::layout::Rect,
}

impl<W: tui::widgets::Widget> Component<W> {
    pub fn new(widget: W, rect: tui::layout::Rect) -> Self {
        Self { widget, rect }
    }
}
