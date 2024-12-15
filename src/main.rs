
pub mod grid;
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
    crossterm::terminal::enable_raw_mode();
}

fn mainloop() -> io::Result<()>{

    enum Gamestate {Falling(FallingState), Landed(), LineDelete(LineDeleteState)}

    let mut grid: grid::Grid = grid::init_grid();
    let mut gamestate: Gamestate = Gamestate::Falling(FallingState{line:0});

    loop{
        
        // Poll keypresses
        
        match get_input() {
            None => {},
            Some(keycode) => {
                match keycode{
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => debug::print("A Pressed".to_string()),
                    _ => {}
                }
            }
        }
        

        // drop block
        // TODO: Program block dropping down
        
        // Collision
        // TODO: Program collision code
        
        // Line detection
        // TODO: Program line detection
        
        // Render
        // TODO: Program renderer 
    }
    Ok(())
}

fn main() {

    init_term();
    mainloop();

}
