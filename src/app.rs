use std::{fs::File, io, path::PathBuf};

use crossterm::event::{self, KeyEventKind};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::Paragraph, DefaultTerminal}
;

use crate::file;

pub struct App {
    terminal: DefaultTerminal,
    pub input: String,
    pub action_input: String,
    filename: String,
    current_dir: PathBuf,
    modified: bool,
    editor_mode: EditorMode,
    exit: bool,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> App {
        App {
            terminal,
            input: "".to_string(),
            action_input: "".to_string(),
            filename: "".to_string(),
            current_dir: PathBuf::new(),
            modified: false,
            editor_mode: EditorMode::Normal,
            exit: false,
        }
    }

    pub fn run(&mut self, directory: PathBuf, filename: &str) -> io::Result<()> {
        self.filename = filename.to_string();
        self.current_dir = directory.clone();
        
        //get file contents
        if let Ok(content) = file::get_file_contents(directory, filename) {
            self.input = content;
        }

        //main loop
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
            .margin(0)
            .constraints(
                [
                Constraint::Percentage(95),
                Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .split(frame.area());

            let editing_text = Paragraph::new(self.input.clone())
            .style(Style::default().fg(Color::Magenta));

            frame.render_widget(editing_text, chunks[0]);
            
            let editing_action = String::from(":") + &self.action_input;
            let editing_mode = Paragraph::new(match self.editor_mode {
                EditorMode::Normal => if self.modified { "INSERT (modified)" } else { "INSERT" },
                EditorMode::Insert => if self.modified { "INSERT (modified)" } else { "INSERT" },
                EditorMode::Action => editing_action.as_str(),
            }).style(Style::default().fg(Color::White).bg(Color::LightMagenta));

            frame.render_widget(editing_mode, chunks[1]);
        })?;
        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char(':') => {
                        if let EditorMode::Normal = self.editor_mode {
                            self.editor_mode = EditorMode::Action;
                        }
                    },
                    event::KeyCode::Char('i') => {
                        self.editor_mode = EditorMode::Insert;
                    },
                    event::KeyCode::Esc => {
                        if let EditorMode::Action = self.editor_mode {
                            self.editor_mode = EditorMode::Normal;
                        }
                        if let EditorMode::Insert = self.editor_mode {
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
                                self.modified = true;
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
                            EditorMode::Insert => {
                                self.input.push('\n');
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
                if app.modified {
                    let file = File::create(app.current_dir.join(&app.filename)).unwrap();
                    file::write_to_file(file, &app.input).unwrap();
                    app.modified = false;
                }
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
