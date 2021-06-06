use serialport::prelude::*;
use std::time::{Duration, SystemTime};

// const MULTIPLY: f32 = 1.0;
const BIAS: f32 = 500.0;
const THRESHOLD: f32 = 10.0;

// the main reading function
// here we interact with all hardware
pub fn reading() -> (String, String) {
    println!("New reading...");

    // get the first 3 lines
    let line1 = read(2);
    println!("line1 = {}", line1);

    let line2 = read(2);
    println!("line2 = {}", line2);

    let line3 = read(2);
    println!("line3 = {}", line3);

    // get related lines with different time delta
    let related_line1 = read(1);
    let related_line2 = read(1);
    let related_line3 = read(1);

    // get the next 3 lines
    let line4 = read(2);
    println!("line4 = {}", line4);

    let line5 = read(2);
    println!("line5 = {}", line5);

    let line6 = read(2);
    println!("line6 = {}", line6);

    // get next related lines with different time delta
    let related_line4 = read(1);
    let related_line5 = read(1);
    let related_line6 = read(1);

    // build the hexagram
    let hexagram = format!("{}{}{}{}{}{}", line1, line2, line3, line4, line5, line6);
    // build related hexagram
    let related_original = format!(
        "{}{}{}{}{}{}",
        related_line1, related_line2, related_line3, related_line4, related_line5, related_line6
    );
    let related = get_related(&hexagram, &related_original);

    println!("hexagram: {}", hexagram);
    println!("related: {}", related);

    (hexagram, related)
}

// here we read data from pip
// the function accepts time in seconds to read
pub fn read(delta: u64) -> u8 {
    // read the pip data
    let data = read_the_pip(delta);
    println!("data: {:?}", data);

    let mut min = 0;
    if let Some(m) = data.iter().min() {
        min = *m;
    };
    println!("min: {}", min);

    let mut max = 0;
    if let Some(m) = data.iter().max() {
        max = *m;
    };
    println!("max: {}", max);

    let n_data = data.iter().map(|&i| i as f32 - BIAS).collect::<Vec<f32>>();
    println!("n_data = {:?}", n_data);

    let mut mins: Vec<f32> = vec![];
    let mut maxs: Vec<f32> = vec![];
    for i in n_data.windows(3) {
        if i[1] > i[0] && i[1] > i[2] && i[1] > THRESHOLD {
            // println!("local max extremum = {:?}", i[1]);
            maxs.push(i[1]);
        }
        if i[1] < i[0] && i[1] < i[2] && i[1].abs() > THRESHOLD {
            // println!("local min extremum = {:?}", i[1]);
            mins.push(i[1]);
        }
        // println!("windows iter = {:?}", i);
    }

    // println!("mins = {:?}", mins);
    // println!("mins len = {:?}", mins.len());
    // println!("maxs = {:?}", maxs);
    // println!("maxs len = {:?}", maxs.len());

    if maxs.len() > mins.len() {
        1
    } else {
        0
    }
}

// here we read pip data from the serial port
// install arduino ide + teense support to read from serial port on rpi
pub fn read_the_pip(delta: u64) -> Vec<i32> {
    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(1),
    };

    let mut data: Vec<i32> = vec![];
    if let Ok(mut port) = serialport::open_with_settings("/dev/ttyACM0", &s) {
        let mut serial_buf: Vec<u8> = vec![0; 512];
        let start = SystemTime::now();
        loop {
            if let Ok(d) = start.elapsed() {
                if d > Duration::from_secs(delta) {
                    break;
                };
            }
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // println!("Pip val: {}", get_val(&serial_buf[..t]));
                    data.push(get_val(&serial_buf[..t]));
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }

    data
}

// serialize the data from serial port
pub fn get_val(buf: &[u8]) -> i32 {
    let mut output = 0;
    let serial_data = std::str::from_utf8(buf).unwrap();
    if let Some(i) = serial_data.find("PiPVal: ") {
        let s = &serial_data[i + 8..];
        if let Some(j) = s.find("\r") {
            let str_value = &s[..j];
            if let Ok(value) = str_value.parse::<i32>() {
                output = value;
            }
        }
    }

    output
}

// get related lines from hexagram and related hexagram
pub fn get_related(h: &String, r: &String) -> String {
    let mut result = "".to_string();
    for (x, y) in h.chars().zip(r.chars()) {
        if x.eq(&y) {
            if x.eq(&'0') {
                result = format!("{}1", result);
            } else {
                result = format!("{}0", result);
            }
        } else {
            result = format!("{}{}", result, x);
        }
    }

    result
}
