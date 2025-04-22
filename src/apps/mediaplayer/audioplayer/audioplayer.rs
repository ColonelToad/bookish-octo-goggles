use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use hound;
use rfd::FileDialog;
use std::error::Error;
use std::fs::File;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::{thread, time::Duration};

fn main () -> Result<(), Box <dyn Error>> {

    //Launches a file dialog to let the user pick a file.
    let file_path = FileDialog::new().pick_file();
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
    let file = File::open(&file_path)?;
    let mut reader = hound::WavReader::new (file)?;
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
        reader.samples::<f32> ()
            .map(|s| s.unwrap())
            .collect()
    };

    //--------------------------------------------------------------------

    //Audio ouput setup using cpal
    let host = cpal::default_host ();
    let device = host.default_output_device().ok_or("no output device available")?;
    let default_config = device.default_output_format()?;
    let sample_format = default_config.sample_format;
    let config: StreamConfig = device.device_default_output_config()?.into();;

    println!("Using default output device: {}", device.name()?);
    println!("Output config: {:?}", config);

   let playback_finished = Arc::new(AtomicBool::new(false));

   match sample_format {
        SampleFormat::F32 => {
            run_stream::<f32>(&device, &config, samples, Arc::clone(&playback_finished))?;
        },
        _=> return Err("Unsupported sample format".into()),
   }
    
   while !playback_finished.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("Playback finished."); 
    
    Ok(()) 

}

//Streams audio until all samples are played
fn run_stream<T> (
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Vec<f32>,
    playback_finished: Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>>
where   
    T: Sample + From<f32>,
    {
        let sample_mut_protector = Arc::new(Mutex::new(samples.into_iter()));

        let err_msg = |err| eprintln!("An error occurred on stream: {}", err);

        let stream = device.build_output_stream (
            config,
            
            move |data: &mut [T], _:&cpal::OutputCallbackInfo| {
                let mut iteration = sample_mut_protector.lock().unwrap();

                for out in data.iter_mut() {
                    if let Some(sample) = iteration.next() {
                        *out = T::from(sample);
                    }

                    else {
                        *out = T::from(0.0);
                        playback_finished.store(true, Ordering::SeqCst);
                    }
                }
            },
                err_msg,     
        )?;

        stream.play()?;
        Ok(())
    }
