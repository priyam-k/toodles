use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType::Rounded, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    todo_items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default, Clone)]
struct TodoItem {
    completed: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app 1"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app 2"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app 3"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app 4"),
    });
    state.todo_items.push(TodoItem {
        completed: false,
        description: String::from("finish app 5"),
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
            if state.is_add_new {
                if handle_add_new(key, state) {
                    state.is_add_new = false;
                }
            } else if handle_key(key, state) {
                break;
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            state.todo_items.push(TodoItem {
                completed: false,
                description: state.input_value.clone(),
            });
            state.input_value.clear();
            return true;
        }
        event::KeyCode::Esc => return true,
        event::KeyCode::Char(char) => {
            state.input_value.push(char);
        }
        event::KeyCode::Backspace => {
            state.input_value.pop();
        }
        _ => {}
    }
    false
}

fn handle_key(key: KeyEvent, state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => return true,
        event::KeyCode::Up => {
            state.list_state.select_previous();
        }
        event::KeyCode::Down => {
            state.list_state.select_next();
        }
        event::KeyCode::Char(char) => match char {
            'q' => return true,
            'A' => {
                state.is_add_new = true;
            }

            'D' => {
                if let Some(index) = state.list_state.selected() {
                    state.todo_items.remove(index);
                }
            }
            'j' => {
                state.list_state.select_previous();
            }
            'k' => {
                state.list_state.select_next();
            }
            _ => {}
        },
        _ => {}
    }
    false
}

fn render(frame: &mut Frame, state: &mut AppState) {
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

    let list = List::new(
        state
            .todo_items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .highlight_symbol("> ")
    .highlight_style(Color::Blue);

    frame.render_stateful_widget(list, list_area, &mut state.list_state);

    if state.is_add_new {
        let [_, add_new_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)])
            .margin(1)
            .areas(border_area);

        Paragraph::new(state.input_value.clone())
            .block(
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .border_type(Rounded),
            )
            .fg(Color::Green)
            .render(add_new_area, frame.buffer_mut());
    }

    // Paragraph::new("Hello, world!").render(frame.area(), frame.buffer_mut());
}
