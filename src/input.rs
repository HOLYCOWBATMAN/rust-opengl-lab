use sdl::event::{NoEvent, QuitEvent};
// use sdl::event::{KeyDownEvent, KeyUpEvent, NoEvent, QuitEvent};
// use sdl::event::{DownKey, EscapeKey, LeftKey, RShiftKey, ReturnKey, RightKey, UpKey};
// use sdl::event::{XKey, ZKey};
use sdl::event;

pub enum InputResult
{
    Continue,   // Keep playing.
    Quit,       // Quit the application.
}

pub struct GamePadState
{
    left: bool,
    down: bool,
    up: bool,
    right: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool
}

pub struct Input
{
    gamepad_0: GamePadState
}

impl Input
{
    static fn new() -> Input
    {
        Input
        {
            gamepad_0: GamePadState
            {
                left: false,
                down: false,
                up: false,
                right: false,
                a: false,
                b: false,
                select: false,
                start: false
            }
        }
    }

    // fn handle_control_event(&mut self, key_event: &KeyEvent, down: bool)
    // {
    //     match key_event.keycode
    //     {
    //         LeftKey   => self.gamepad_0.left   = down,
    //         DownKey   => self.gamepad_0.down   = down,
    //         UpKey     => self.gamepad_0.up     = down,
    //         RightKey  => self.gamepad_0.right  = down,
    //         ZKey      => self.gamepad_0.a      = down,
    //         XKey      => self.gamepad_0.b      = down,
    //         RShiftKey => self.gamepad_0.select = down,
    //         ReturnKey => self.gamepad_0.start  = down,
    //         _         => {}
    //     }
    // }

    fn check_input(&self) -> InputResult
    {
        loop
        {
            match event::poll_event()
            {
                NoEvent => break,
                // KeyDownEvent(ref key_event) =>
                // {
                //     self.handle_control_event(key_event, true);

                //     if key_event.keycode == EscapeKey
                //     {
                //         return Quit;
                //     }
                // }
                // KeyUpEvent(ref key_event) => self.handle_control_event(key_event, false),
                QuitEvent => return Quit,
                _ => {}
            }
        }
        return Continue;
    }
}