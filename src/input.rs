use num_traits::WrappingShl;
use glfw::{Action, Context, Key, MouseButton};


// create consts for each key to properly modify the inputstate unsigned integers

// Input States go here
pub struct InputState
{
    // Perhaps instead of keeping bools for each control
    // I could simply modify and access an array for
    // pressed buttons and held buttons, along with modifiers

    // Maybe this is overcomplicated, and I can simply
    // update a bool array. It would be much simpler
    // than bit shifting math. It would also be shorter
    // and prone to less bugs.

    // What other key categories should I keep track of?

    pub standard_keys_press: u64,
    pub standard_keys_held: u64,
    pub modifier_keys_press: u32,
    pub modifier_keys_held: u32,
    pub mouse_button_press: u16,
    pub mouse_button_held: u16,

    pub w_Active: bool,
    pub a_Active: bool,
    pub s_Active: bool,
    pub d_Active: bool,
    pub space_Active: bool,
}

// Kestrel Standard Key (KSK)
// The "shift" of each key
// When key is A, shift 0 times 
// and check if the 0th bit is set
pub enum KSK
{
    A = 0,
    D = 1,
    S = 2,
    W = 3,
}

// Kestrel Modifier Key (KMK)
pub enum KMK
{
    LShift = 0,
    RShift = 1,
    LControl = 2,
    RControl = 3,
}

// Kestrel Mouse Buttons (KMB)
pub enum KMB
{
    M1 = 0,
    M2 = 1,
    M3 = 2,
    M4 = 3,
    M5 = 4,
}

impl InputState
{
    pub fn new() -> InputState
    {
        InputState
        {
            standard_keys_press: 0,
            standard_keys_held: 0,
            modifier_keys_press: 0,
            modifier_keys_held: 0,
            mouse_button_press: 0,
            mouse_button_held: 0,
            w_Active: false,
            a_Active: false,
            s_Active: false,
            d_Active: false,
            space_Active: false,
        }
    }

    pub fn isStandardPressed(&mut self, key: u64) -> bool
    {
        return self.standard_keys_press & (1 << key) != 0;
    }

    pub fn isStandardHeld(&mut self, key: u64) -> bool
    {
        return self.standard_keys_held & (1 << key) != 0;
    }

    pub fn isModifierPressed(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_press & (1 << key) != 0;
    }

    pub fn isModifierHeld(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_held & (1 << key) != 0;
    }

    pub fn isMouseButtonPressed(&mut self, key: u16) -> bool
    {
        return self.mouse_button_press & (1 << key) != 0;
    }
    
    pub fn isMouseButtonHeld(&mut self, key: u16) -> bool
    {
        return self.mouse_button_held & (1 << key) != 0;
    }
}

// $key values will NEVER be higher than 63, but rust insists
// on blocking this due to overflow, therefore I must use this.
// https://docs.rs/num-traits/0.2.12/num_traits/ops/wrapping/trait.WrappingShl.html
macro_rules! key_used {
    (press => $key:expr, $state:expr) => {
        let shift: u64 = WrappingShl::wrapping_shl(&1u64, $key as u32);

        let tempstate = ($state.standard_keys_press | shift);
        $state.standard_keys_press = tempstate;
        
    };
    (hold => $key:expr, $state:expr) => {
        let shift: u64 = WrappingShl::wrapping_shl(&1u64, $key as u32);

        // only increment key ONCE when held
        let tempState = ($state.standard_keys_held | shift);
        if tempState != 1 || $key as u32 == 0
        {
            $state.standard_keys_held = tempState;
        }
        
        println!("press: {}", $state.standard_keys_press);
        println!("held: {}", $state.standard_keys_held);
    };
    (release => $key:expr, $state:expr) => {    // bug here somehow?
        let shift: u64 = WrappingShl::wrapping_shl(&1u64, $key as u32);
        
        let tempstate1 = ($state.standard_keys_press & shift);
        
        $state.standard_keys_press -= tempstate1;

        let tempState2 = ($state.standard_keys_held & shift);

        println!("{}", (tempState2 != 0));
        if (tempState2 != 0)
        {
            $state.standard_keys_held -= $state.standard_keys_held | shift;
        }

        println!("press: {}", $state.standard_keys_press);
        println!("held: {}", $state.standard_keys_held);
    };
}

pub fn handleWindowEvent(window: &mut glfw::Window, event: glfw::WindowEvent, inState: &mut InputState) -> bool
{

    // How to reduce the size of this function?
    // Maybe match event, then store the 4 
    // parts of the Key Event, then convert Key to
    // it's respective kestrel key?

    match event
    {
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) =>
        {
            inState.mouse_button_press += inState.mouse_button_press | (1 << KMB::M1 as u64);
            println!("Mouse press: {}", inState.mouse_button_press);
        }
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Repeat, _) =>
        {
            let shift = (1 << KMB::M1 as u16);
            
            if (inState.mouse_button_held | shift) != 1
            {
                inState.mouse_button_held += inState.mouse_button_held | shift;
            }
            println!("Mouse hold: {}", inState.mouse_button_press);
        }
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) =>
        {
            let shift = (1 << KMB::M1 as u16);

            inState.mouse_button_press -= inState.mouse_button_press | shift;

            // only increment key ONCE when held
            if (inState.mouse_button_held & shift) != 0
            {
                inState.mouse_button_held = inState.mouse_button_held | shift;
            }
            println!("Mouse release: {}", inState.mouse_button_press);
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>
        {
            window.set_should_close(true);
            return window.should_close();
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) =>      {key_used!(press => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) =>     {key_used!(hold => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) =>    {key_used!(release => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) =>      {key_used!(press => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) =>     {key_used!(hold => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) =>    {key_used!(release => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) =>      {key_used!(press => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) =>     {key_used!(hold => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) =>    {key_used!(release => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) =>      {key_used!(press => KSK::D, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) =>     {key_used!(hold => KSK::D, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) =>    {key_used!(release => KSK::D, inState);},
        _ =>                                                        println!("{:?}", event),
    }
    return false;
}