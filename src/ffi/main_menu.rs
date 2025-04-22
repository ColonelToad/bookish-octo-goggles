extern "C" {
    pub fn show_main_menu(selected: i32);
}

pub fn display_main_menu(selected: usize) {
    unsafe {
        show_main_menu(selected as i32);
    }
}
