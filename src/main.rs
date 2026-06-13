use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType::Rounded, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    todo_items: Vec<TodoItem>,
}

#[derive(Debug, Default, Clone)]
struct TodoItem {
    completed: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();

    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app"),
    });



    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    List::new(
        state
            .todo_items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .render(list_area, frame.buffer_mut());

    // Paragraph::new("Hello, world!").render(frame.area(), frame.buffer_mut());
}
