use std::io;
use crossterm::event::{self, Event, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Text, Widget, Frame};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    if self.counter > 0 {
                        self.counter -= 1;
                    }
                }
                KeyCode::Right => {
                    self.counter = self.counter.saturating_add(1);
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.exit = true;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
