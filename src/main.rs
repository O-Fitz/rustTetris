
pub mod block;
pub mod debug;

use crossterm::event::{self, Event, KeyCode};
use std::io;


struct FallingState {
    line: i32,
    
}

struct LandedState {
    time: u32,
}

struct LineDeleteState {
    lstart: u32,
    lend: u32,
    time: u32,
}

fn get_input() -> Option<KeyCode> {
    match event::poll(std::time::Duration::from_secs(0)) {
        Ok(true) => {
            match event::read(){
                Ok(e) => {
                    match e {
                        Event::Key(key_event) => Some(key_event.code),
                        _ => None
                    }
                },
                Err(err) => {
                    match err {
                        // TODO: Handle more errors here
                        err => {debug::print_error(err); None}
                }}
            }
        },
        Ok(false) => {None},
        Err(err) => {debug::print_error(err); None}
    }
}

fn init_term() {
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen);
    crossterm::terminal::enable_raw_mode();
}

fn clean_term() {
    match crossterm::terminal::disable_raw_mode() {
        Ok(_) => (),
        Err(err) => debug::print_error(err)
    }

    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
}

fn mainloop() {

    const GRID_WIDTH: usize = 10;
    const GRID_HEIGHT: usize = 20;

    enum Gamestate {Falling(FallingState), Landed(), LineDelete(LineDeleteState)}

    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
    let mut gamestate: Gamestate = Gamestate::Falling(FallingState{line:0});

    let mut speed: u32 = 20;
    let mut time: u32 = 0;

    let mut current_block: block::Block = block::make_random_block();
    let mut next_block: block::Block = block::make_random_block();

    loop{
        time += 1;
        if time == speed{
            time = 0;
        }

        // Poll keypresses
        match get_input() {
            None => (),
            Some(keycode) => {
                match keycode{
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => debug::print("A Pressed".to_string()),
                    _ => () 
                }
            }
        }
        

        // Drop block
        // TODO: Program block dropping
        if let Gamestate::Falling(fs) = gamestate {

            

        }

        // Collision
        // TODO: Program collision code
        
        // Line detection
        // TODO: Program line detection
        
        // Render
        // TODO: Program renderer 
    }
    
}

fn main() {

    init_term();

    mainloop();

    clean_term();

}
