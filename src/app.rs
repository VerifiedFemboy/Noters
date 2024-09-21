use std::{fs::File, io};

use crossterm::event::{self, KeyEventKind};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::Paragraph, DefaultTerminal}
;

pub struct App {
    terminal: DefaultTerminal,
    pub input: String,
    pub action_input: String,
    selected_file: Option<File>,
    editor_mode: EditorMode,
    exit: bool,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> App {
        App {
            terminal,
            input: "".to_string(),
            action_input: "".to_string(),
            selected_file: None,
            editor_mode: EditorMode::Normal,
            exit: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if self.exit {
                return Ok(());
            }

            self.render()?;
            self.handle_input()?;
        }
    }

    fn render(&mut self) -> io::Result<()> {
        let terminal = &mut self.terminal;
        let _ = terminal.draw(|frame| {
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(frame.area());

            let editing_text = Paragraph::new(self.input.clone())
            .style(Style::default().fg(Color::LightMagenta).bg(Color::Black));

            frame.render_widget(editing_text, chunks[0]);
            
            let editing_action = String::from(":") + &self.action_input;
            let editing_mode = Paragraph::new(match self.editor_mode {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Action => editing_action.as_str(),
            });

            frame.render_widget(editing_mode, chunks[1]);
        })?;
        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char(':') => {
                        self.action_input.clear();
                        self.editor_mode = EditorMode::Action;
                    },
                    event::KeyCode::Esc => {
                        if let EditorMode::Action = self.editor_mode {
                            self.editor_mode = EditorMode::Normal;
                        }
                    },
                    event::KeyCode::Char(c) => {
                        match self.editor_mode {
                            EditorMode::Action => {
                                self.action_input.push(c);
                            },
                            EditorMode::Insert => {
                                self.input.push(c);
                            },
                            _ => {}
                        }
                    },
                    event::KeyCode::Backspace => {
                        match self.editor_mode {
                            EditorMode::Action => {
                                self.action_input.pop();
                            },
                            EditorMode::Insert => {
                                self.input.pop();
                            },
                            _ => {}
                        }
                    },

                    event::KeyCode::Enter => {
                        match self.editor_mode {
                            EditorMode::Action => {
                                let action_input = self.action_input.clone();
                                action(&action_input, self);

                                self.action_input.clear();
                                self.editor_mode = EditorMode::Normal;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
                return Ok(());
            }
        }
        Ok(())
    }
}

fn action(input: &str, app: &mut App) {
    for c in input.chars() {
        match c {
            'w' => {
                
            },
            'q' => {
                app.exit = true;
            },
            _ => {}
            
        }
    }
}

enum EditorMode {
    Normal,
    Action,
    Insert,
}
