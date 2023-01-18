// Hello! Sorry to hear that I caused trouble. Please see the next line down.

// Please visit this website https://play.rust-lang.org/ and paste this code there!



use std::f64;
const EULER: f64 = std::f64::consts::E; // Euler's number, "e"

fn main() {

    let capacitence: f64 = 0.001; // Maximum capacitence on the capacitor. It's denoted on the side, by the "1000uF".
    let resistence: f64 = 100000.0; // Luv me resistors, simple as. (this is in Ohms.)


    let voltage: f64 = 5.0;// Voltage applied by power source during lab. (it was 5 during the practical test)
    let step: f64 = 50.0;

    println!("Output will be given in Amperes.");

    for i in (0..=(step*(10 as f64)) as i64).step_by(step as usize) {
        let ic: f64 = (voltage)/resistence * EULER.powf(-i as f64/(resistence*capacitence));
        println!("Discharge capacitance at point {} is {}",i,ic*10.0_f64.powf(5 as f64).abs()); // Multiplied by 10^5 so I don't get some teensy tiny number, my eyes are fragile!
        //Should also add that because I'm multiplying by 10^5, actual results are 10^-5
    }

    for i in (0..=(step*(10 as f64)) as i64).step_by(step as usize) {
        let vc: f64 = voltage*(1.0 - EULER.powf(-i as f64/(resistence*capacitence)));
        println!("Charge capacitance at point {} is {}",i,vc.abs());
    }
    //Decided to do this in code instead of by hand because of my extensive background in programming, and because it was significantly easier than doing the same operation 10 times ;)
    
    // All outputed data is in Ampere.

}
