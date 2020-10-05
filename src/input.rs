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

    // pub standard_keys_press: u64,
    // pub standard_keys_held: u64,
    // pub modifier_keys_press: u32,
    // pub modifier_keys_held: u32,
    // pub mouse_button_press: u16,
    // pub mouse_button_held: u16,

    pub standard_keys_press: Vec<bool>,
    pub standard_keys_held: Vec<bool>,
    pub modifier_keys_press: Vec<bool>,
    pub modifier_keys_held: Vec<bool>,
    pub mouse_button_press: Vec<bool>,
    pub mouse_button_held: Vec<bool>,


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
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
    ZERO = 26,
    ONE = 27,
    TWO = 28,
    THREE = 29,
    FOUR = 30,
    FIVE = 31,
    SIX = 32,
    SEVEN = 33,
    EIGHT = 34,
    NINE = 35,
    NUMPAD_ZERO = 36,
    NUMPAD_ONE = 37,
    NUMPAD_TWO = 38,
    NUMPAD_THREE = 39,
    NUMPAD_FOUR = 40,
    NUMPAD_FIVE = 41,
    NUMPAD_SIX = 42,
    NUMPAD_SEVEN = 43,
    NUMPAD_EIGHT = 44,
    NUMPAD_NINE = 45,
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
    M6 = 5,
    M7 = 6,
    M8 = 7,
    M9 = 8,
    M10 = 9,
}

impl InputState
{
    pub fn new() -> InputState
    {
        let mut ret = InputState
        {
            // standard_keys_press: 0,
            // standard_keys_held: 0,
            // modifier_keys_press: 0,
            // modifier_keys_held: 0,
            // mouse_button_press: 0,
            // mouse_button_held: 0,

            standard_keys_press: Vec::with_capacity(64),
            standard_keys_held: Vec::with_capacity(64),
            modifier_keys_press: Vec::with_capacity(32),
            modifier_keys_held: Vec::with_capacity(32),
            mouse_button_press: Vec::with_capacity(32),
            mouse_button_held: Vec::with_capacity(32),

            w_Active: false,
            a_Active: false,
            s_Active: false,
            d_Active: false,
            space_Active: false,
        };

        for _ in 0..32
        {
            ret.standard_keys_press.push(false);
            ret.standard_keys_held.push(false);
            ret.modifier_keys_press.push(false);
            ret.modifier_keys_held.push(false);
            ret.mouse_button_press.push(false);
            ret.mouse_button_held.push(false);
        }
        for _ in 32..64
        {
            ret.standard_keys_press.push(false);
            ret.standard_keys_held.push(false);
        }

        return ret;
    }

    pub fn isStandardPressed(&mut self, key: u64) -> bool
    {
        return self.standard_keys_press[key as usize];
    }

    pub fn isStandardHeld(&mut self, key: u64) -> bool
    {
        return self.standard_keys_held[key as usize];
    }

    pub fn isModifierPressed(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_press[key as usize];
    }

    pub fn isModifierHeld(&mut self, key: u32) -> bool
    {
        return self.modifier_keys_held[key as usize];
    }

    pub fn isMouseButtonPressed(&mut self, key: u16) -> bool
    {
        return self.mouse_button_press[key as usize];
    }
    
    pub fn isMouseButtonHeld(&mut self, key: u16) -> bool
    {
        return self.mouse_button_held[key as usize];
    }
}

macro_rules! key_used {
    (press => $key:expr, $state:expr) => {
        
        $state.standard_keys_press[$key as usize] = true;

    };
    (hold => $key:expr, $state:expr) => {

        $state.standard_keys_held[$key as usize] = true;

    };
    (release => $key:expr, $state:expr) => {

        $state.standard_keys_press[$key as usize] = false;
        $state.standard_keys_held[$key as usize] = false;

    };
}

macro_rules! mouse_button_used {
    (press => $key:expr, $state:expr) => {
        
        $state.mouse_button_press[$key as usize] = true;

    };
    (hold => $key:expr, $state:expr) => {

        $state.mouse_button_held[$key as usize] = true;

    };
    (release => $key:expr, $state:expr) => {

        $state.mouse_button_press[$key as usize] = false;
        $state.mouse_button_held[$key as usize] = false;

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
        /** Mouse Buttons **/
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) =>   {mouse_button_used!(press => KMB::M1, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M1, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {mouse_button_used!(release => KMB::M1, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press, _) =>   {mouse_button_used!(press => KMB::M2, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M2, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Release, _) => {mouse_button_used!(release => KMB::M2, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button3, Action::Press, _) =>   {mouse_button_used!(press => KMB::M3, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button3, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M3, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button3, Action::Release, _) => {mouse_button_used!(release => KMB::M3, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button4, Action::Press, _) =>   {mouse_button_used!(press => KMB::M4, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button4, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M4, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button4, Action::Release, _) => {mouse_button_used!(release => KMB::M4, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button5, Action::Press, _) =>   {mouse_button_used!(press => KMB::M5, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button5, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M5, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button5, Action::Release, _) => {mouse_button_used!(release => KMB::M5, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button6, Action::Press, _) =>   {mouse_button_used!(press => KMB::M6, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button6, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M6, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button6, Action::Release, _) => {mouse_button_used!(release => KMB::M6, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button7, Action::Press, _) =>   {mouse_button_used!(press => KMB::M7, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button7, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M7, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button7, Action::Release, _) => {mouse_button_used!(release => KMB::M7, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button8, Action::Press, _) =>   {mouse_button_used!(press => KMB::M8, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button8, Action::Repeat, _) =>  {mouse_button_used!(hold => KMB::M8, inState);},
        glfw::WindowEvent::MouseButton(MouseButton::Button8, Action::Release, _) => {mouse_button_used!(release => KMB::M8, inState);},
        /** Mouse Events (Scroll) **/

        /** Letter Keys **/
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) =>      {key_used!(press => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) =>     {key_used!(hold => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) =>    {key_used!(release => KSK::A, inState);},
        glfw::WindowEvent::Key(Key::B, _, Action::Press, _) =>      {key_used!(press => KSK::B, inState);},
        glfw::WindowEvent::Key(Key::B, _, Action::Repeat, _) =>     {key_used!(hold => KSK::B, inState);},
        glfw::WindowEvent::Key(Key::B, _, Action::Release, _) =>    {key_used!(release => KSK::B, inState);},
        glfw::WindowEvent::Key(Key::C, _, Action::Press, _) =>      {key_used!(press => KSK::C, inState);},
        glfw::WindowEvent::Key(Key::C, _, Action::Repeat, _) =>     {key_used!(hold => KSK::C, inState);},
        glfw::WindowEvent::Key(Key::C, _, Action::Release, _) =>    {key_used!(release => KSK::C, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) =>      {key_used!(press => KSK::D, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) =>     {key_used!(hold => KSK::D, inState);},
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) =>    {key_used!(release => KSK::D, inState);},
        glfw::WindowEvent::Key(Key::E, _, Action::Press, _) =>      {key_used!(press => KSK::E, inState);},
        glfw::WindowEvent::Key(Key::E, _, Action::Repeat, _) =>     {key_used!(hold => KSK::E, inState);},
        glfw::WindowEvent::Key(Key::E, _, Action::Release, _) =>    {key_used!(release => KSK::E, inState);},
        glfw::WindowEvent::Key(Key::F, _, Action::Press, _) =>      {key_used!(press => KSK::F, inState);},
        glfw::WindowEvent::Key(Key::F, _, Action::Repeat, _) =>     {key_used!(hold => KSK::F, inState);},
        glfw::WindowEvent::Key(Key::F, _, Action::Release, _) =>    {key_used!(release => KSK::F, inState);},
        glfw::WindowEvent::Key(Key::G, _, Action::Press, _) =>      {key_used!(press => KSK::G, inState);},
        glfw::WindowEvent::Key(Key::G, _, Action::Repeat, _) =>     {key_used!(hold => KSK::G, inState);},
        glfw::WindowEvent::Key(Key::G, _, Action::Release, _) =>    {key_used!(release => KSK::G, inState);},
        glfw::WindowEvent::Key(Key::H, _, Action::Press, _) =>      {key_used!(press => KSK::H, inState);},
        glfw::WindowEvent::Key(Key::H, _, Action::Repeat, _) =>     {key_used!(hold => KSK::H, inState);},
        glfw::WindowEvent::Key(Key::H, _, Action::Release, _) =>    {key_used!(release => KSK::H, inState);},
        glfw::WindowEvent::Key(Key::I, _, Action::Press, _) =>      {key_used!(press => KSK::I, inState);},
        glfw::WindowEvent::Key(Key::I, _, Action::Repeat, _) =>     {key_used!(hold => KSK::I, inState);},
        glfw::WindowEvent::Key(Key::I, _, Action::Release, _) =>    {key_used!(release => KSK::I, inState);},
        glfw::WindowEvent::Key(Key::J, _, Action::Press, _) =>      {key_used!(press => KSK::J, inState);},
        glfw::WindowEvent::Key(Key::J, _, Action::Repeat, _) =>     {key_used!(hold => KSK::J, inState);},
        glfw::WindowEvent::Key(Key::J, _, Action::Release, _) =>    {key_used!(release => KSK::J, inState);},
        glfw::WindowEvent::Key(Key::K, _, Action::Press, _) =>      {key_used!(press => KSK::K, inState);},
        glfw::WindowEvent::Key(Key::K, _, Action::Repeat, _) =>     {key_used!(hold => KSK::K, inState);},
        glfw::WindowEvent::Key(Key::K, _, Action::Release, _) =>    {key_used!(release => KSK::K, inState);},
        glfw::WindowEvent::Key(Key::L, _, Action::Press, _) =>      {key_used!(press => KSK::L, inState);},
        glfw::WindowEvent::Key(Key::L, _, Action::Repeat, _) =>     {key_used!(hold => KSK::L, inState);},
        glfw::WindowEvent::Key(Key::L, _, Action::Release, _) =>    {key_used!(release => KSK::L, inState);},
        glfw::WindowEvent::Key(Key::M, _, Action::Press, _) =>      {key_used!(press => KSK::M, inState);},
        glfw::WindowEvent::Key(Key::M, _, Action::Repeat, _) =>     {key_used!(hold => KSK::M, inState);},
        glfw::WindowEvent::Key(Key::M, _, Action::Release, _) =>    {key_used!(release => KSK::M, inState);},
        glfw::WindowEvent::Key(Key::N, _, Action::Press, _) =>      {key_used!(press => KSK::N, inState);},
        glfw::WindowEvent::Key(Key::N, _, Action::Repeat, _) =>     {key_used!(hold => KSK::N, inState);},
        glfw::WindowEvent::Key(Key::N, _, Action::Release, _) =>    {key_used!(release => KSK::N, inState);},
        glfw::WindowEvent::Key(Key::O, _, Action::Press, _) =>      {key_used!(press => KSK::O, inState);},
        glfw::WindowEvent::Key(Key::O, _, Action::Repeat, _) =>     {key_used!(hold => KSK::O, inState);},
        glfw::WindowEvent::Key(Key::O, _, Action::Release, _) =>    {key_used!(release => KSK::O, inState);},
        glfw::WindowEvent::Key(Key::P, _, Action::Press, _) =>      {key_used!(press => KSK::P, inState);},
        glfw::WindowEvent::Key(Key::P, _, Action::Repeat, _) =>     {key_used!(hold => KSK::P, inState);},
        glfw::WindowEvent::Key(Key::P, _, Action::Release, _) =>    {key_used!(release => KSK::P, inState);},
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) =>      {key_used!(press => KSK::Q, inState);},
        glfw::WindowEvent::Key(Key::Q, _, Action::Repeat, _) =>     {key_used!(hold => KSK::Q, inState);},
        glfw::WindowEvent::Key(Key::Q, _, Action::Release, _) =>    {key_used!(release => KSK::Q, inState);},
        glfw::WindowEvent::Key(Key::R, _, Action::Press, _) =>      {key_used!(press => KSK::R, inState);},
        glfw::WindowEvent::Key(Key::R, _, Action::Repeat, _) =>     {key_used!(hold => KSK::R, inState);},
        glfw::WindowEvent::Key(Key::R, _, Action::Release, _) =>    {key_used!(release => KSK::R, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) =>      {key_used!(press => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) =>     {key_used!(hold => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) =>    {key_used!(release => KSK::S, inState);},
        glfw::WindowEvent::Key(Key::T, _, Action::Press, _) =>      {key_used!(press => KSK::T, inState);},
        glfw::WindowEvent::Key(Key::T, _, Action::Repeat, _) =>     {key_used!(hold => KSK::T, inState);},
        glfw::WindowEvent::Key(Key::T, _, Action::Release, _) =>    {key_used!(release => KSK::T, inState);},
        glfw::WindowEvent::Key(Key::U, _, Action::Press, _) =>      {key_used!(press => KSK::U, inState);},
        glfw::WindowEvent::Key(Key::U, _, Action::Repeat, _) =>     {key_used!(hold => KSK::U, inState);},
        glfw::WindowEvent::Key(Key::U, _, Action::Release, _) =>    {key_used!(release => KSK::U, inState);},
        glfw::WindowEvent::Key(Key::V, _, Action::Press, _) =>      {key_used!(press => KSK::V, inState);},
        glfw::WindowEvent::Key(Key::V, _, Action::Repeat, _) =>     {key_used!(hold => KSK::V, inState);},
        glfw::WindowEvent::Key(Key::V, _, Action::Release, _) =>    {key_used!(release => KSK::V, inState);},
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) =>      {key_used!(press => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) =>     {key_used!(hold => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) =>    {key_used!(release => KSK::W, inState);},
        glfw::WindowEvent::Key(Key::X, _, Action::Press, _) =>      {key_used!(press => KSK::X, inState);},
        glfw::WindowEvent::Key(Key::X, _, Action::Repeat, _) =>     {key_used!(hold => KSK::X, inState);},
        glfw::WindowEvent::Key(Key::X, _, Action::Release, _) =>    {key_used!(release => KSK::X, inState);},
        glfw::WindowEvent::Key(Key::Y, _, Action::Press, _) =>      {key_used!(press => KSK::Y, inState);},
        glfw::WindowEvent::Key(Key::Y, _, Action::Repeat, _) =>     {key_used!(hold => KSK::Y, inState);},
        glfw::WindowEvent::Key(Key::Y, _, Action::Release, _) =>    {key_used!(release => KSK::Y, inState);},
        glfw::WindowEvent::Key(Key::Z, _, Action::Press, _) =>      {key_used!(press => KSK::Z, inState);},
        glfw::WindowEvent::Key(Key::Z, _, Action::Repeat, _) =>     {key_used!(hold => KSK::Z, inState);},
        glfw::WindowEvent::Key(Key::Z, _, Action::Release, _) =>    {key_used!(release => KSK::Z, inState);},
        /** Number Keys **/
        glfw::WindowEvent::Key(Key::Num0, _, Action::Press, _) =>      {key_used!(press => KSK::ZERO, inState);},
        glfw::WindowEvent::Key(Key::Num0, _, Action::Repeat, _) =>     {key_used!(hold => KSK::ZERO, inState);},
        glfw::WindowEvent::Key(Key::Num0, _, Action::Release, _) =>    {key_used!(release => KSK::ZERO, inState);},
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) =>      {key_used!(press => KSK::ONE, inState);},
        glfw::WindowEvent::Key(Key::Num1, _, Action::Repeat, _) =>     {key_used!(hold => KSK::ONE, inState);},
        glfw::WindowEvent::Key(Key::Num1, _, Action::Release, _) =>    {key_used!(release => KSK::ONE, inState);},
        glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) =>      {key_used!(press => KSK::TWO, inState);},
        glfw::WindowEvent::Key(Key::Num2, _, Action::Repeat, _) =>     {key_used!(hold => KSK::TWO, inState);},
        glfw::WindowEvent::Key(Key::Num2, _, Action::Release, _) =>    {key_used!(release => KSK::TWO, inState);},
        glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) =>      {key_used!(press => KSK::THREE, inState);},
        glfw::WindowEvent::Key(Key::Num3, _, Action::Repeat, _) =>     {key_used!(hold => KSK::THREE, inState);},
        glfw::WindowEvent::Key(Key::Num3, _, Action::Release, _) =>    {key_used!(release => KSK::THREE, inState);},
        glfw::WindowEvent::Key(Key::Num4, _, Action::Press, _) =>      {key_used!(press => KSK::FOUR, inState);},
        glfw::WindowEvent::Key(Key::Num4, _, Action::Repeat, _) =>     {key_used!(hold => KSK::FOUR, inState);},
        glfw::WindowEvent::Key(Key::Num4, _, Action::Release, _) =>    {key_used!(release => KSK::FOUR, inState);},
        glfw::WindowEvent::Key(Key::Num5, _, Action::Press, _) =>      {key_used!(press => KSK::FIVE, inState);},
        glfw::WindowEvent::Key(Key::Num5, _, Action::Repeat, _) =>     {key_used!(hold => KSK::FIVE, inState);},
        glfw::WindowEvent::Key(Key::Num5, _, Action::Release, _) =>    {key_used!(release => KSK::FIVE, inState);},
        glfw::WindowEvent::Key(Key::Num6, _, Action::Press, _) =>      {key_used!(press => KSK::SIX, inState);},
        glfw::WindowEvent::Key(Key::Num6, _, Action::Repeat, _) =>     {key_used!(hold => KSK::SIX, inState);},
        glfw::WindowEvent::Key(Key::Num6, _, Action::Release, _) =>    {key_used!(release => KSK::SIX, inState);},
        glfw::WindowEvent::Key(Key::Num7, _, Action::Press, _) =>      {key_used!(press => KSK::SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Num7, _, Action::Repeat, _) =>     {key_used!(hold => KSK::SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Num7, _, Action::Release, _) =>    {key_used!(release => KSK::SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Num8, _, Action::Press, _) =>      {key_used!(press => KSK::EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Num8, _, Action::Repeat, _) =>     {key_used!(hold => KSK::EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Num8, _, Action::Release, _) =>    {key_used!(release => KSK::EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Num9, _, Action::Press, _) =>      {key_used!(press => KSK::NINE, inState);},
        glfw::WindowEvent::Key(Key::Num9, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NINE, inState);},
        glfw::WindowEvent::Key(Key::Num9, _, Action::Release, _) =>    {key_used!(release => KSK::NINE, inState);},
        /** Keypad Number Keys **/
        glfw::WindowEvent::Key(Key::Kp0, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_ZERO, inState);},
        glfw::WindowEvent::Key(Key::Kp0, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_ZERO, inState);},
        glfw::WindowEvent::Key(Key::Kp0, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_ZERO, inState);},
        glfw::WindowEvent::Key(Key::Kp1, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_ONE, inState);},
        glfw::WindowEvent::Key(Key::Kp1, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_ONE, inState);},
        glfw::WindowEvent::Key(Key::Kp1, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_ONE, inState);},
        glfw::WindowEvent::Key(Key::Kp2, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_TWO, inState);},
        glfw::WindowEvent::Key(Key::Kp2, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_TWO, inState);},
        glfw::WindowEvent::Key(Key::Kp2, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_TWO, inState);},
        glfw::WindowEvent::Key(Key::Kp3, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_THREE, inState);},
        glfw::WindowEvent::Key(Key::Kp3, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_THREE, inState);},
        glfw::WindowEvent::Key(Key::Kp3, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_THREE, inState);},
        glfw::WindowEvent::Key(Key::Kp4, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_FOUR, inState);},
        glfw::WindowEvent::Key(Key::Kp4, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_FOUR, inState);},
        glfw::WindowEvent::Key(Key::Kp4, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_FOUR, inState);},
        glfw::WindowEvent::Key(Key::Kp5, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_FIVE, inState);},
        glfw::WindowEvent::Key(Key::Kp5, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_FIVE, inState);},
        glfw::WindowEvent::Key(Key::Kp5, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_FIVE, inState);},
        glfw::WindowEvent::Key(Key::Kp6, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_SIX, inState);},
        glfw::WindowEvent::Key(Key::Kp6, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_SIX, inState);},
        glfw::WindowEvent::Key(Key::Kp6, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_SIX, inState);},
        glfw::WindowEvent::Key(Key::Kp7, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Kp7, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Kp7, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_SEVEN, inState);},
        glfw::WindowEvent::Key(Key::Kp8, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Kp8, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Kp8, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_EIGHT, inState);},
        glfw::WindowEvent::Key(Key::Kp9, _, Action::Press, _) =>      {key_used!(press => KSK::NUMPAD_NINE, inState);},
        glfw::WindowEvent::Key(Key::Kp9, _, Action::Repeat, _) =>     {key_used!(hold => KSK::NUMPAD_NINE, inState);},
        glfw::WindowEvent::Key(Key::Kp9, _, Action::Release, _) =>    {key_used!(release => KSK::NUMPAD_NINE, inState);},
        /** Number Pad Keys **/
        
        
        /** Arrow Keys **/
        
        
        /** Misc Keys **/
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>
        {
            window.set_should_close(true);
            return window.should_close();
        },
        _ =>                                                        {/*println!("{:?}", event);*/},
    }
    return false;
}

/*************************************/
// Input tests

#[test]
fn pressTest1()
{
    let mut inpState = InputState::new();

    key_used!(press => KSK::Z, inpState);

    assert_eq!(inpState.isStandardPressed(KSK::Z as u64), true);

}

#[test]
fn pressTest2()
{
    let mut inpState = InputState::new();

    key_used!(press => KSK::Z, inpState);

    assert_eq!(inpState.isStandardPressed(KSK::Z as u64), true);

    key_used!(release => KSK::Z, inpState);

    assert_eq!(inpState.isStandardPressed(KSK::Z as u64), false);

}

#[test]
fn heldTest1()
{
    let mut inpState = InputState::new();

    key_used!(hold => KSK::Z, inpState);

    assert_eq!(inpState.isStandardHeld(KSK::Z as u64), true);

}

#[test]
fn heldTest2()
{
    let mut inpState = InputState::new();

    key_used!(hold => KSK::Z, inpState);
    
    assert_eq!(inpState.isStandardHeld(KSK::Z as u64), true);

    key_used!(release => KSK::Z, inpState);

    assert_eq!(inpState.isStandardHeld(KSK::Z as u64), false);
}

#[test]
fn mousePressTest()
{
    let mut inpState = InputState::new();

    mouse_button_used!(press => KMB::M1, inpState);
    
    assert_eq!(inpState.isMouseButtonPressed(KMB::M1 as u16), true);
    
    mouse_button_used!(release => KMB::M1, inpState);

    assert_eq!(inpState.isMouseButtonPressed(KMB::M1 as u16), false);

}

#[test]
fn mouseHeldTest()
{
    let mut inpState = InputState::new();

    mouse_button_used!(hold => KMB::M1, inpState);

    assert_eq!(inpState.isMouseButtonHeld(KMB::M1 as u16), true);

    mouse_button_used!(release => KMB::M1, inpState);

    assert_eq!(inpState.isMouseButtonHeld(KMB::M1 as u16), false);
}

/*************************************/