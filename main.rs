use crossterm::terminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

fn main() {
    terminal::enable_raw_mode().unwrap();

    let mut buffer = String::new(); // stores what you type

    print!(">> "); // prompt
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                // quit
                KeyCode::Char('q') => break,

                // add character to buffer
                KeyCode::Char(c) => {
                    buffer.push(c);
                    print!("{}", c); // show it as you type
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                },

                // on enter, process the full line
                KeyCode::Enter => {
                    println!("\r\nYou typed: {}\r", buffer);
                    buffer.clear(); // clear for next line
                    print!(">> \r");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                },

                // delete last character
                KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        buffer.pop();
                        print!("\r>> {}", buffer); // redraw line
                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    }
                },

                _ => {}
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
    println!("\r\nGoodbye!");
}