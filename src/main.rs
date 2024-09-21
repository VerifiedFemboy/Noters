use std::io;
mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    terminal.clear()?;

    let mut app = app::App::new(terminal);

    match app.run() {
        Ok(_) => {
            println!("Successfully exited ratatui");
            ratatui::restore();
        },
        Err(err) => eprintln!("Error: {}", err),
    };

    Ok(())
}
