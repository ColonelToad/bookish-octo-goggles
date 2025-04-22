#[derive(Debug, Clone)]
pub enum UIScreen {
    Welcome,
    MainMenu(u8), // 0 = APPS, 1 = PROFILE, 2 = SETTINGS
}