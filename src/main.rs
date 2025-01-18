#![no_main]
#![no_std]

mod time;
mod led;
mod button;
mod channel;

use button::{ButtonDirection, ButtonTask};
use channel::Channel;
use led::LedTask;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit;
use rtt_target::{rtt_init_print, rprintln};
//use panic_rtt_target as _;  // replace by #[panic_handler]
use core::panic::PanicInfo;
//use core::cell::Cell;

#[entry]
fn main() -> ! {
    //let mut i = 0;
    let board = microbit::Board::take().unwrap();
    //let mut timer = microbit::hal::Timer::new(board.TIMER0);
    let ticker = time::Ticker::new(board.RTC0);
    let (col, mut row) = board.display_pins.degrade();
    let button_right = board.buttons.button_b.degrade();
    let button_left = board.buttons.button_a.degrade();

    let channel: Channel<ButtonDirection> = Channel::new();
    let mut led_task = LedTask::new(col, &ticker, channel.get_receiver());
    let mut button_l_task = 
        ButtonTask::new(
            button_left, 
            &ticker, 
            button::ButtonDirection::Left, 
            channel.get_sender()
        );
    let mut button_r_task = 
        ButtonTask::new(
            button_right, 
            &ticker, 
            button::ButtonDirection::Right, 
            channel.get_sender()
        );


    row[0].set_high().ok();
    rtt_init_print!();
    rprintln!("Hello, button & led tasks");
    
    // for _ in 0..5{
    //     rprintln!("delay 0.5 sec..");
    //     row1.toggle().ok();
    //     timer.delay_ms(500);
    // }

    loop{
        led_task.poll();
        button_l_task.poll();
        button_r_task.poll();
    }

    //panic!("Done, stop by panic! macro.");
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}