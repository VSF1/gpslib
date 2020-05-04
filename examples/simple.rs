extern crate adafruit_gps;

pub use adafruit_gps::gps::{Gps, open_port, GpsSentence};

fn main() {

    // What info do I need at the start?
    // Baud rate and update rate.

    // Open the port that is connected to the GPS module.
    let port = open_port("/dev/serial0", 9600);
    // Initialise the Gps.
    let mut gps = Gps {port};

    // gps.init() requires the update rate for the gps (1000 miliseconds (1Hz) is default)
    // It returns a hash map to tell you if setting the update rate was successful and if the
    // return type setting was successful.
    // If the return type setting was not successful, the gps.update() method may hang forever and
    // you will need to try gps.init() again until it is successful.
    // If the update_rate is not successful, the gps will run but at whatever the previous setting was.
    // If setting the update_rate consistently fails for faster updates, see exmaples/increase_frequency.rs

    /// Give settings here.
    gps.pmtk_220_set_nmea_updaterate("1000");

    // In a loop, constantly update the gps. The update trait will give you all the data you
    // want from the gps module.
    loop {
        let values = gps.update();

        // Depending on what values you are interested in you can adjust what sentences you
        // wish to get and ignore all other sentences.
        match values {
            GpsSentence::InvalidSentence => println!("Invalid sentence, try again"),
            GpsSentence::InvalidBytes => println!("Invalid bytes given, try again"),
            GpsSentence::NoConnection => println!("No connection with gps"),
            GpsSentence::GGA(sentence) => {
                println!("{}",sentence.utc);
                println!("{}",sentence.long);
                println!("{}",sentence.lat);
            }
            GpsSentence::RMC(sentence) => {
                println!("{}", sentence.speed)
            }
            _ => {
                println!("Dont care about other sentence outputs")
            }
        }
    }
}