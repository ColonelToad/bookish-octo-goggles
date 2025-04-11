use cpal :: traits :: {DeviceTrait, HostTrait, StreamTrait};
use hound;
use rfd :: FileDialog;
use std :: error :: Error;
use std :: fs :: FIle;
use std :: thread;
use std :: time :: Duration;

fn main () -> Result<(), Box <dyn Error>> {

    //Launches a file dialog to let the user pick a file.
    let file_path = FileDialog :: new().pick_file();
    let file_path = match file_path {
        Some(path) => path,
        None => {
            println!("No file selected.");
            return Ok(());
        }
    };

    //-----------------------------------------------------------------

    println!("Selected file: {:?}", file_path);
    
    //File is opened using file::open to return a file handle
    //The question mark operator is used to handle errors.
    //When a file is opened, a new WAV reader instance is made using Hound.
    //The reader parses the WAV file header and preps to read audio data.
    //the spec() is then called to retrieve details about the WAV file.
    //Then its printed for clarity purposes.
    let file = File :: open (&file_path)?;
    let mut reader = hound :: WavReader :: new (file)?;
    let spec = reader.spec();
    println!("WAV Spec: {:?}", spec);


    //Takes samples and converts them to f32 samples
    let samples: Vec<f32> = 
    
    //So if the WAV file is 16 bit, it converts the samples to f32
    if spec.bits_per_sample == 16 {
        reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect()
    } 
    
    //Otherwise, it collects the samples as f32
    else {
        reader.samples :: <f32> ()
            .map(|s| s.unwrap())
            .collect()
    };

    //--------------------------------------------------------------------

    //Audio ouput setup using cpal
    let host = cpal :: default_host ();
    let device = host.default_output_device().ok_or("no output device available")?;
    let config =  device_default_output_config()?;

    println!("Using default output device: {}", device.name()?);
    println!("Output config: {:?}", config);

    match config.sample_format() {
        cpal :: SampleFormat :: F32 => run_stream :: <f32>(&device, config.into(), samples)?,
        _ => return Err("Unsupported sample format".into()),
    }

    


}