use std::process::Command;
use std::error::Error;
use rfd::FileDialog;


fn main() -> Result<(), Box<dyn Error>> {

    //launches a file dialog to let the user select a video file
    let file_path = FileDialog::new() 
        .add_filter("Videos", &["mp4", "mkv", "avi", "mov"])
        .pick_file();

    let file_path = match file_path {
        Some(file) => file,
        None => {
            println!("No file selected");
            return Ok(());
        }
    };

    //Convert the selected file path to a string
    let video_path = file_path.to_str().ok_or("Invalid file path")?;

    //Call VLC w/ the selected video file and fullscreen flag
    let status = Command::new("vlc")
        .arg(video_path)
        .arg("--fullscreen")
        .status()?;

    if status.success() {
        println!("Video player launched successfully");
    } 
    
    else {
        println!("Failed to launch video player");
    }

    Ok(())
} 