use std::env;
use std::fs;

// struct Person {
//     name: String,
//     address1: String,
//     city: String,
//     state: String,
//     zip: String,
// }

struct Repeater {
    freq: String,
    duplex: String,
    offset: f32,
    tone: String,
    location: String,
    county: String,
    lat: String,
    long: String,
    call: String,
    usage: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let _prog_name = &args[0];
    let file_path = &args[1];
    
    //println!("\n\n\t*** Working with file {}.", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    //println!("\n\n\t*** File Text ->\n\n{contents}");

    let mut list_of_repeaters: Vec<Repeater> = Vec::new();

    // TODO: Look for a Crate to do this
    for line in contents.lines() {
        //println!("\n\n\t*** Line -> {line}");
        let elements:Vec<_> = line.split(',').collect();
        let _freq = elements[0].get(0..elements[0].len()-1).unwrap();
        let _duplex = elements[0].get(elements[0].len()-1..elements[0].len()).unwrap().to_string();
        let repeater = Repeater {
            freq: _freq.to_string(),
            duplex: _duplex,
            offset: get_offset(_freq),
            tone: elements[1].to_string(),
            location: elements[2].to_string(),
            county: elements[3].to_string(),
            lat: elements[4].to_string(),
            long: elements[5].to_string(),
            call: elements[6].to_string(),
            usage: elements[7].to_string(),
        };
        list_of_repeaters.push(repeater);
    }

    // // TODO:  print CHIRP format
    // //println!("Location,Name,Frequency,Duplex,Offset,Tone,rToneFreq,cToneFreq,DtcsCode,DtcsPolarity,Mode,TStep,Comment");
    // println!("name,frequency,duplex,offset,tone,rtonefreq,ctonefreq,dtcscode,dtcspolarity,mode,tstep");
    // let mut ctr = 70;
    // for rptr in list_of_repeaters {
    //     //println!("{0},{1},{2},,{3},{4},,,,FM,,", rptr.location, rptr.call, rptr.freq, rptr.offset, rptr.tone);
    //     let mut tone_string = "";
    //     let mut tone = "";
    //     if rptr.tone != "" {
    //         tone_string = "TSQL";
    //         tone = &rptr.tone;
    //     }
    //     else {
    //         tone_string = "TSQL";
    //         tone = "141.3";
    //     }
    //     println!("{0},{1},{2},{3},{4},{5},{6},{7},{8},{9},{10},{11},,,,,,", ctr, "", rptr.freq, rptr.duplex, rptr.offset, tone_string, tone, tone, "23", "NN", "FM", 5.000);
    //     ctr = ctr + 1;
    // }

    print_chirp_format(list_of_repeaters);
}

fn is_positive(val: &str) -> String {

    let freq_value = i16::from_str_radix(val, 16)
        .expect("Couldn't parse frequency as a number.");
    
    if freq_value > 0 {
        return "+".to_string();
    } else {
        return "-".to_string();
    }
}

fn get_offset(freq: &str) -> f32 {
    //println!("Frequency value -> {0}", freq.to_string());
    let freq_value: f32 = freq.parse().expect("Couldn't parse frequency as a number.");

    if freq_value > 144.00 && freq_value < 150.00 {
        return 0.6;
    } else {
        return 5.0;
    }
}

fn print_chirp_format(repeaters: Vec<Repeater>) {
    println!("name,frequency,duplex,offset,tone,rtonefreq,ctonefreq,dtcscode,dtcspolarity,mode,tstep");
    let mut ctr = 70;
    for rptr in repeaters {
        //println!("{0},{1},{2},,{3},{4},,,,FM,,", rptr.location, rptr.call, rptr.freq, rptr.offset, rptr.tone);
        let mut tone_string = "";
        let mut tone = "";
        if rptr.tone != "" {
            tone_string = "TSQL";
            tone = &rptr.tone;
        }
        else {
            tone_string = "TSQL";
            tone = "141.3";
        }
        println!("{0},{1},{2},{3},{4},{5},{6},{7},{8},{9},{10},{11},,,,,,", ctr, "", rptr.freq, rptr.duplex, rptr.offset, tone_string, tone, tone, "23", "NN", "FM", 5.000);
        ctr = ctr + 1;
    }
}
