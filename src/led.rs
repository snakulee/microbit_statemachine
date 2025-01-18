use fugit::ExtU64;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use microbit::gpio::NUM_COLS;
use microbit::hal::gpio::{Pin, Output, PushPull};
//use microbit::pac::p0::dir;
use rtt_target::rprintln;
use crate::time::{Timer, Ticker};
use crate::channel::Receiver;
use crate::button::ButtonDirection;

pub struct LedTask<'a>{
    col: [Pin<Output<PushPull>>; NUM_COLS],
    active_col: usize,
    ticker: &'a Ticker,
    state: LedState<'a>,
    channel: Receiver<'a, ButtonDirection>,
}

pub enum LedState<'a>{
    Toggle,
    Wait(Timer<'a>)
}

impl<'a> LedTask<'a>{
    pub fn new(col:[Pin<Output<PushPull>>; NUM_COLS], 
        ticker: &'a Ticker, 
        channel: Receiver<'a, ButtonDirection>) -> Self{
        Self{
            col: col,
            active_col: 0,
            ticker: ticker,
            state: LedState::Toggle,
            channel: channel,
        }
    }

    fn shift(&mut self, direction: ButtonDirection){
        self.col[self.active_col].set_high().ok();
        match direction {
            ButtonDirection::Left => {
                rprintln!("button A (left) pressed");
                
                if self.active_col == 0{
                    self.active_col = NUM_COLS-1;
                }
                else {
                    self.active_col -= 1;
                }
            },
            ButtonDirection::Right => {
                rprintln!("button B (right) pressed");
                if (self.active_col + 1) >= NUM_COLS{
                    self.active_col = 0;
                }
                else {
                    self.active_col += 1;
                }
            },
        }
        self.col[self.active_col].set_high().ok();
    }

    pub fn poll(&mut self){
        match &self.state {
            LedState::Toggle => {
                rprintln!("Blink LED {}", self.active_col);
                self.col[self.active_col].toggle().ok();
                let duration_500ms = Timer::new(500.millis(), &self.ticker);
                self.state = LedState::Wait(duration_500ms);
            }

            LedState::Wait(timer)=>{
                if timer.is_ready() {
                    self.state = LedState::Toggle;
                }

                if let Some(direction) = self.channel.receive(){
                    self.shift(direction);
                    self.state = LedState::Toggle;
                }
            }
        }
    }
}

