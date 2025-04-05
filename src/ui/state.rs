// src/ui/state.rs

#[derive(Debug, Clone, Copy)]
pub enum UIScreen {
    Welcome,
    MainMenu(usize),
}
