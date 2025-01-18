use fugit::ExtU64;
use microbit::hal::gpio::{Floating, Input, Pin};
use embedded_hal::digital::InputPin;
use crate::time::{Ticker, Timer};
use crate::channel::Sender;

#[derive(Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

enum ButtonState<'a> {
    WaitForPress,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a>{
    pin: Pin<Input<Floating>>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    channel: Sender<'a, ButtonDirection>,
}

impl<'a> ButtonTask<'a>{
    pub fn new(
       pin: Pin<Input<Floating>>, 
       ticker: &'a Ticker, 
       direction: ButtonDirection,
       channel: Sender<'a, ButtonDirection>
    ) -> Self{
        Self{
            pin, 
            ticker, 
            direction, 
            state:ButtonState::WaitForPress,
            channel: channel,
        }
    }

    pub fn poll(&mut self) {
        match &self.state{
            ButtonState::WaitForPress => {
                if self.pin.is_low().unwrap(){
                    let duration_100ms = Timer::new(100.millis(), &self.ticker);
                    self.channel.send(self.direction);
                    self.state = ButtonState::Debounce(duration_100ms);
                }
            }

            ButtonState::Debounce(timer) => {
                if timer.is_ready() && self.pin.is_high().unwrap() {
                    self.state = ButtonState::WaitForPress;
                }
            }
        }

    }
}