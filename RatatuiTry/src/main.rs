use ratatui::widgets::{Block, Paragraph};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
//use ratatui::symbols::Marker::Block;
use chrono::Local;
use ratatui::widgets::Borders;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut pressed = false;
    let mut message:Option<String> = None;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Ratatui")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            if let Some(msg) = &message{
               let paragraph = Paragraph::new(msg.clone())
                   .block(Block::default().title("Message").borders(Borders::ALL));
                f.render_widget(paragraph, size);
            }
        })?;
        if let Event::Key(key) = event::read()? {
            let now = Local::now();
            message = Some(format!(
                "You pressed {:?} at {}", key.code, now.format("%Y-%m-%d %H:%M:%S")
            ));
            if key.code == KeyCode::Char('q') {
                break Ok(())
            }
        }
    }
}

fn render(frame: &mut Frame){
    frame.render_widget("Hello world my darling", frame.area());
}

