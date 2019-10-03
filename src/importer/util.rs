use std::io;

pub fn to_ascii_string(chars: &[u8]) -> String {

    let mut rstring = String::new();
    for byte in chars {
        if byte.is_ascii() {
            let c = byte.clone() as char; //latin1_to_char(byte.clone());
            rstring.push(c);
        }
    }
    rstring
}

