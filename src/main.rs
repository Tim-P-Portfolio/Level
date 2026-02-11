#![no_main]
#![no_std]

use cortex_m::delay;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin, i2c::I2c};
use microbit::{
    board::{Board, I2CInternalPins},
    display::{self, blocking::Display},
    hal::{
        gpio::{Output, PushPull},
        timer::{self, Timer},
        twim, Delay, Twim,
    },
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use lsm303agr;

use num_traits::float::Float;

struct Position {
    x: u8,
    y: u8,
}

struct Level<T: DelayNs> {
    display: Display,
    position: Position,
    timer: T,
    delay: u32,
}

impl<T: DelayNs> Level<T> {
    fn new(timer: T, display: Display) -> Self {
        Self {
            timer: timer,
            display: display,
            position: Position { x: 0, y: 0 },
            delay: 100,
        }
    }

    fn draw(&mut self, x: usize, y: usize) {
        let mut display = [[0; 5]; 5];

        display[x][y] = 1;

        self.display.show(&mut self.timer, display, self.delay);
    }

    pub fn set_delay(&mut self, delay: u32) {
        self.delay = delay;
    }

    pub fn set(&mut self, x: i8, y: i8) {
        let x = match x {
            2.. => 2,
            ..-2 => -2,
            _ => x,
        };
        let y = match y {
            2.. => 2,
            ..-2 => -2,
            _ => y,
        };

        self.draw((x + 2) as usize, (y + 2) as usize);
    }
}

#[entry]
fn init() -> ! {
    rtt_init_print!();

    rprintln!("Hello?");
    let mut board = Board::take().unwrap();
    let mut timer0 = Timer::new(board.TIMER0);
    let mut timer1 = Timer::new(board.TIMER1);

    let pos = Position { x: 0, y: 0 };

    let display = display::blocking::Display::new(board.display_pins);

    let mut dis = Level::new(timer0, display);
    dis.set_delay(200);

    // P0.08 	I2C_INT_SCL
    // P0.16 	I2C_INT_SDA
    let i2c = Twim::new(
        board.TWIM0,
        twim::Pins {
            scl: board.i2c_internal.scl.degrade(),
            sda: board.i2c_internal.sda.degrade(),
        },
        twim::Frequency::K100,
    );

    let mut sensor = lsm303agr::Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer1,
            lsm303agr::AccelMode::Normal,
            lsm303agr::AccelOutputDataRate::Hz100,
        )
        .unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut timer1,
            lsm303agr::MagMode::LowPower,
            lsm303agr::MagOutputDataRate::Hz10,
        )
        .unwrap();

    loop {
        // let status = sensor.mag_status();

        let accel = match sensor.acceleration() {
            Ok(v) => v,
            Err(e) => {
                rprintln!("Error getting acceleration: {:?}", e);
                continue;
            }
        };


        

        let x = ((-accel.x_mg() as f32) / 250.0).round() as i8;
        let y = ((accel.y_mg() as f32) / 250.0).round() as i8;



        rprintln!("{:?}, {:?}", x, y);


        if accel.z_mg() < 0 {
            dis.set(y, x);
        }

        // for x in -3..=3 {
        //     for y in -3..=3 {
        //         dis.set(x, y);
        //     }
        // }
        // timer1.delay(100);
    }
}
