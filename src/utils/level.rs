use crate::Level;

extern crate quick_xml;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

extern crate flate2;

use std::{collections::HashMap, io::prelude::*};
use flate2::read::GzDecoder;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::io::Cursor;

pub fn read_gmd() -> String {
    let mut reader = Reader::from_file("./Unnamed 4.gmd").expect("No file!");
    reader.config_mut().trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }

    let mut idx = 0;
    for i in 0..txt.len() {
        if txt[i] == "k4" { idx = i + 1; }
    }
    txt[idx].clone()
}

pub fn decode_level() -> String {
    let level = read_gmd();
    let decoded_bytes = URL_SAFE.decode(level).expect("Failed to decode level");
    let mut gz = GzDecoder::new(Cursor::new(decoded_bytes));
    let mut decompressed_string = String::new();
    gz.read_to_string(&mut decompressed_string).expect("Failed to decompress gzip");

    decompressed_string
}

pub fn parse_level(level: String) -> Level {
    let a = level.split(';').collect::<Vec<&str>>();
    let mut objects = Vec::with_capacity(a.len() - 2);
    for i in 1..(a.len() - 1) {
        let obj = a[i].split(',').collect::<Vec<&str>>();
        let mut object = HashMap::with_capacity(obj.len() / 2);
        for j in 0..(obj.len() / 2) {
            object.insert(obj[j * 2].to_string(), obj[j * 2 + 1].to_string());
        }
        objects.push(object);
    }

    let b = a[0].split(',').collect::<Vec<&str>>();
    let mut data = HashMap::with_capacity(b.len() / 2 - 1);
    let mut color_string = String::new();
    for i in 0..(b.len() / 2) {
        if b[i * 2] == "kS38" {
            color_string = b[i * 2 + 1].to_string();
            continue;
        }
        data.insert(b[i * 2].to_string(), b[i * 2 + 1].to_string());
    }

    let c = color_string.split('|').collect::<Vec<&str>>();
    let mut colors = Vec::with_capacity(c.len() - 1);
    for i in 0..(c.len() - 1) {
        let col = c[i].split('_').collect::<Vec<&str>>();
        let mut color = HashMap::with_capacity(col.len() / 2);
        for j in 0..(col.len() / 2) {
            color.insert(col[j * 2].to_string(), col[j * 2 + 1].to_string());
        }
        colors.push(color);
    }

    Level { data, colors, objects }
}
