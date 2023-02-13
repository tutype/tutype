use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, io::Stdout, iter::Iterator, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

// TODO: major refactor

// for now use hardcoded values
const TEXT: &str = "hello world this is test string";

fn main() -> Result<(), io::Error> {
    let mut terminal = start()?;

    terminal.draw(|frame| ui(frame))?;

    thread::sleep(Duration::new(5, 0));

    exit(terminal)?;

    Ok(())
}

fn ui<B: Backend>(frame: &mut Frame<B>) {
    let size = frame.size();

    let wrapper_block = Block::default().borders(Borders::ALL);

    let spans: Vec<Span> = TEXT
        .chars()
        .enumerate()
        .map(|(index, char)| {
            Span::styled(
                char.to_string(),
                Style::default()
                    .add_modifier(Modifier::DIM)
                    .add_modifier(Modifier::BOLD),
            )
        })
        .collect();

    let quote = Paragraph::new(Spans::from(spans))
        .block(Block::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    let help_rect = Rect {
        x: 2,
        y: size.height - 2,
        width: size.width - 2,
        height: 2,
    };
    let help = Paragraph::new("esc to quit; tab to open config; ctrl + r to reload a quote")
        .block(Block::default())
        .alignment(Alignment::Left);

    let paragraph_margin = size.width / 100 * 30;
    let paragraph_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(paragraph_margin)
        .constraints([Constraint::Percentage(50), Constraint::Length(3)].as_ref())
        .split(frame.size());

    frame.render_widget(wrapper_block, size);
    frame.render_widget(quote, paragraph_layout[1]);
    frame.render_widget(help, help_rect);
}

fn exit(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn start() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}
