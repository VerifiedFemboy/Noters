use std::io;
mod app;
mod file;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let current_dir = std::env::current_dir()?;
    
    let mut terminal = ratatui::init();

    terminal.clear()?;

    let mut app = app::App::new(terminal);

    match app.run(current_dir, &filename) {
        Ok(_) => {
            println!("Successfully exited ratatui");
            ratatui::restore();
        },
        Err(err) => eprintln!("Error: {}", err),
    };

    Ok(())
}
