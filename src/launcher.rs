use std::process::Command;

pub struct Launcher;

impl Launcher {
    pub fn open_file_explorer() {
        Self::spawn("pcmanfm");
    }

    pub fn open_terminal() {
        Self::spawn("lxterminal");
    }

    pub fn open_ide() {
        Self::spawn("thonny");
    }

    pub fn open_text_editor() {
        Self::spawn("mousepad");
    }

    pub fn open_calendar() {
        Self::spawn_flatpak("org.gnome.Calendar");
    }

    pub fn open_maps() {
        Self::spawn_flatpak("org.gnome.Maps");
    }

    pub fn open_notes() {
        Self::spawn_flatpak("com.github.flxzt.rnote");
    }

    pub fn open_todo_list() {
        Self::spawn_flatpak("io.github.mrvladus.List");
    }

    pub fn open_writer(file: Option<&str>) {
        Self::spawn_with_args("libreoffice", &["--writer"], file);
    }

    pub fn open_calc(file: Option<&str>) {
        Self::spawn_with_args("libreoffice", &["--calc"], file);
    }

    pub fn open_impress(file: Option<&str>) {
        Self::spawn_with_args("libreoffice", &["--impress"], file);
    }

    fn spawn(cmd: &str) {
        if let Err(e) = Command::new(cmd).spawn() {
            eprintln!("Failed to launch `{}`: {}", cmd, e);
        }
    }

    fn spawn_flatpak(app_id: &str) {
        if let Err(e) = Command::new("flatpak")
            .args(["run", app_id])
            .spawn()
        {
            eprintln!("Failed to launch flatpak `{}`: {}", app_id, e);
        }
    }

    fn spawn_with_args(base: &str, args: &[&str], file: Option<&str>) {
        let mut command = Command::new(base);
        command.args(args);
        if let Some(f) = file {
            command.arg(f);
        }

        if let Err(e) = command.spawn() {
            eprintln!("Failed to launch `{}` with args {:?}: {}", base, args, e);
        }
    }
}

/// Optional enum to map UI IDs
pub enum LaunchableApp {
    Weather,
    AudioPlayer,
    VideoPlayer,
    FileExplorer,
    Terminal,
    IDE,
    TextEditor,
    Calendar,
    Maps,
    Notes,
    TodoList,
    Writer(Option<String>),
    Calc(Option<String>),
    Impress(Option<String>),
}

pub fn launch_app(app: LaunchableApp) {
    match app {
        LaunchableApp::Weather => {
            let _ = Command::new("cargo")
                .args(&["run", "--bin", "weather"])
                .spawn();
        }
        LaunchableApp::AudioPlayer => {
            let _ = Command::new("cargo")
                .args(&["run", "--bin", "audio"])
                .spawn();
        }
        LaunchableApp::VideoPlayer => {
            let _ = Command::new("cargo")
                .args(&["run", "--bin", "video"])
                .spawn();
        }
        LaunchableApp::FileExplorer => Launcher::open_file_explorer(),
        LaunchableApp::Terminal => Launcher::open_terminal(),
        LaunchableApp::IDE => Launcher::open_ide(),
        LaunchableApp::TextEditor => Launcher::open_text_editor(),
        LaunchableApp::Calendar => Launcher::open_calendar(),
        LaunchableApp::Maps => Launcher::open_maps(),
        LaunchableApp::Notes => Launcher::open_notes(),
        LaunchableApp::TodoList => Launcher::open_todo_list(),
        LaunchableApp::Writer(file) => Launcher::open_writer(file.as_deref()),
        LaunchableApp::Calc(file) => Launcher::open_calc(file.as_deref()),
        LaunchableApp::Impress(file) => Launcher::open_impress(file.as_deref()),
    }
}