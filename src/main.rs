use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Parsing args
    let args: Vec<String> = env::args().collect();

    // If args are parsed
    if args.len() > 1 {
        // Getting argument from command!
        let file_name = &args[1];
        // Playing Song
        play_it(file_name);
    } else {
        // Asking user to provide the filename.
        println!("Please provide the file name with extension:-");
        // Taking input
        let mut file_name = String::new();
        std::io::stdin()
            .read_line(&mut file_name)
            .expect("Input is required");

        // Playing song!
        play_it(&file_name.trim().to_string())
    }
}

fn play_it(file_name: &String) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(file_name).expect("File error"));

    // Decode that sound file into a source
    let source = Decoder::new(file).expect("Can't decode");

    // Printing in terminal!
    println!("Playing {}", file_name);

    // Appending it to the play
    sink.append(source);

    // Since its playing in another thread, we have to wait till the song is over!
    sink.sleep_until_end()
}
