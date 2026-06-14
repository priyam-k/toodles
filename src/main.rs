use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType::Rounded, List, ListItem, ListState, Paragraph, Widget, Wrap},
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

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

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
                match handle_add_new(key, state) {
                    FormAction::Submit => {
                        state.is_add_new = false;
                        state.todo_items.push(TodoItem {
                            completed: false,
                            description: state.input_value.clone(),
                        });
                        state.input_value.clear();
                    }
                    FormAction::Escape => {
                        state.is_add_new = false;
                        state.input_value.clear();
                    }
                    FormAction::None => {}
                }
            } else if handle_key(key, state) {
                break;
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Esc => return FormAction::Escape,
        event::KeyCode::Char(char) => {
            state.input_value.push(char);
        }
        event::KeyCode::Backspace => {
            state.input_value.pop();
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key(key: KeyEvent, state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => return true,
        event::KeyCode::Enter => {
            if let Some(index) = state.list_state.selected() {
                if let Some(item) = state.todo_items.get_mut(index) {
                    item.completed = !item.completed;
                }
            }
        }
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

    let [mut list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(2)
        .areas(border_area);

    Block::bordered()
        .title(" Toodles ".to_span().into_centered_line())
        .title_bottom(
            " [↑/↓] select, [enter] toggle, [A] add new, [D] delete, [esc] quit "
                .to_span()
                .into_centered_line(),
        )
        .border_type(Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(state.todo_items.iter().map(|x| {
        let value = if x.completed {
            x.description.to_span().crossed_out()
        } else {
            x.description.to_span()
        };
        ListItem::from(value)
    }))
    .highlight_symbol("> ")
    .highlight_style(Color::Blue);

    if state.is_add_new {
        let [new_list_area, add_new_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)])
                .margin(2)
                .areas(border_area);
        list_area = new_list_area;

        Paragraph::new(state.input_value.clone() + "█")
            .block(
                Block::bordered()
                    .title(" Add new task ".to_span().into_centered_line())
                    .title_bottom(" [Enter] to submit ".to_span().into_centered_line())
                    // .padding(Padding::uniform(1))
                    .border_type(Rounded)
                    .fg(Color::Green),
            )
            .wrap(Wrap { trim: true })
            .fg(Color::White)
            .render(add_new_area, frame.buffer_mut());

        // frame.set_cursor_position(Position::new(
        //     add_new_area.x + 1 + state.input_value.len() as u16,
        //     add_new_area.y + 1,
        // ));
    }

    frame.render_stateful_widget(list, list_area, &mut state.list_state);
}
