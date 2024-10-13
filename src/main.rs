#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{
    adc::{Adc, AdcConfig, Default},
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

#[entry]
fn main() -> ! {
    let board = match Board::take() {
        Some(board) => board,
        None => panic!("failed to find board"),
    };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut adc = Adc::new(board.ADC, AdcConfig::default_10bit());
    let mut anapin = board.edge.e00.into_floating_input();

    let numbers = [
        [
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ],
        [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ],
        [
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ],
        [
            [0, 1, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
        ],
        [
            [0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ],
    ];

    let sign_plus = [
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ];

    let letter_e = [
        [0, 1, 1, 1, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0],
    ];

    loop {
        let analog = match adc.read_channel(&mut anapin) {
            Ok(analog) => analog,
            Err(_) => {
                display.show(&mut timer, letter_e, 10);
                continue;
            }
        };

        let mut count = 0;

        for number in numbers.iter() {
            if count == (analog / 100).unsigned_abs() as usize {
                display.show(&mut timer, *number, 10);
                break;
            }

            count += 1;
        }

        if count == numbers.len() {
            display.show(&mut timer, sign_plus, 10);
        }
    }
}
