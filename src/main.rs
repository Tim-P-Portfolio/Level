#![no_main] 
#![no_std] 

use cortex_m_rt::entry; 
use embedded_hal::{digital::OutputPin, delay::DelayNs}; 
use microbit::{ 
    board::Board, display::{self, blocking::Display}, hal::{ 
        Delay, gpio::{Output, PushPull}, timer::{self, Timer} 
    } 
}; 
use rtt_target::{rtt_init_print, rprintln}; 
use panic_rtt_target as _; 


struct Position {
    x: u8,
    y: u8
}

struct Level <T: DelayNs> {
    display: Display,
    position: Position,
    timer: T
}

impl<T: DelayNs> Level <T> {
    fn new(timer: T, display: Display) -> Self {
        Self { timer: timer, display: display, position: Position {x:0, y:0} }
    }

    fn draw(&mut self) {
        let x = 0;
        let y = 2;
        let mut display = [[0; 5]; 5];

        display[x][y] = 1;

        self.display.show(&mut self.timer, display, 100);
    }

    fn set(&mut self, x: u8, y: u8) {
        
        self.draw();
    }

}


#[entry] 
fn init() -> ! { 
    rtt_init_print!(); 
    let mut board = Board::take().unwrap(); 
    let mut timer = Timer::new(board.TIMER0); 

    let pos = Position {x: 0, y: 0};
    
    let display = display::blocking::Display::new(board.display_pins);

    let mut dis = Level::new(timer, display);
    
    loop {
        dis.set();
        
    }
} 