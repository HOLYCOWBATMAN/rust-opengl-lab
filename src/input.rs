use sdl::event::{NoEvent, QuitEvent, KeyDownEventType, KeyUpEventType, Key, KeyEvent};
use sdl::event::{Key, DownKey, EscapeKey, LeftKey, RShiftKey, ReturnKey, RightKey, UpKey};
use sdl::event::{XKey, ZKey};
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

    fn handle_control_event(&mut self, key: Key, down: bool)
    {
        match key
        {
            ZKey      => self.gamepad_0.a      = down,      // 122
            XKey      => self.gamepad_0.b      = down,      // 120
            UpKey     => self.gamepad_0.up     = down,      // 273
            DownKey   => self.gamepad_0.down   = down,      // 274
            RightKey  => self.gamepad_0.right  = down,      // 275
            LeftKey   => self.gamepad_0.left   = down,      // 276
            RShiftKey => self.gamepad_0.select = down,      // 303
            ReturnKey => self.gamepad_0.start  = down,      // 13
            _         => {}
        }
    }

    fn check_input(&mut self) -> InputResult
    {
        loop
        {
            match event::poll_event()
            {
                NoEvent => break,
                KeyEvent(key, isDown, _, _) =>
                {
                    self.handle_control_event(key, isDown);

                    if key == EscapeKey
                    {
                        return Quit;
                    }
                }
                QuitEvent => return Quit,
                _ => {}
            }
        }
        return Continue;
    }
}