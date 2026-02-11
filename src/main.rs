#![no_main]
#![no_std]

use cortex_m::delay;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, i2c::I2c, digital::{OutputPin, InputPin}};
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



struct Level<T: DelayNs> {
    display: Display,
    timer: T,
    delay: u32,
}

impl<T: DelayNs> Level<T> {
    fn new(timer: T, display: Display) -> Self {
        Self {
            timer: timer,
            display: display,
            delay: 100,
        }
    }

    /// Draw unclamped position to mb2 display
    fn draw(&mut self, x: usize, y: usize) {
        let mut display = [[0; 5]; 5];

        // Set led at coordinate: on
        display[x][y] = 1;

        self.display.show(&mut self.timer, display, self.delay);
    }

    pub fn set_delay(&mut self, delay: u32) {
        self.delay = delay;
    }

    /// Draw on mb2 display, coordinate relitive to center
    pub fn set(&mut self, x: i8, y: i8) {
        // Clamp x and y
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

    let mut board = Board::take().unwrap();
    let mut timer0 = Timer::new(board.TIMER0);
    let mut timer1 = Timer::new(board.TIMER1);

    let display = display::blocking::Display::new(board.display_pins);

    let mut led_display = Level::new(timer0, display);
    led_display.set_delay(200);

    // Setup I2C Two wire connection
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

    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    let mut fine_mode = false;

    loop {
        // Sensor status loop: 
        // https://docs.rust-embedded.org/discovery/microbit/10-punch-o-meter/my-solution.html 
        while !sensor.accel_status().unwrap().xyz_new_data() {}

        // Get acceleration data
        let accel = match sensor.acceleration() {
            Ok(v) => v,
            Err(e) => {
                rprintln!("Error getting acceleration: {:?}", e);
                continue;
            }
        };

        // Get button presses from front micro switches
        let button_a_pressed = button_a.is_low().unwrap();
        let button_b_pressed = button_b.is_low().unwrap();

        // Set state based on button pressed
        if !button_a_pressed && button_b_pressed {
            fine_mode = true;
        }
        if button_a_pressed && !button_b_pressed {
            fine_mode = false;
        }

        // Set multiplier based on mode
        let multiplier = if fine_mode { 25.0 } else { 250.0 };

        // Get x and y positions from acceleration data, and split into 5 parts
        let x = ((-accel.x_mg() as f32) / multiplier).round() as i8;
        let y = ((accel.y_mg() as f32) / multiplier).round() as i8;


        rprintln!("{}, {:?}, {:?}", fine_mode, x, y);

        // If the microbit is right side up display the point
        if accel.z_mg() < 0 {
            led_display.set(y, x);
        }
    }
}
