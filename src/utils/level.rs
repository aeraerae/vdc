use crate::Level;

extern crate quick_xml;
extern crate flate2;

use std::{collections::HashMap, io::{prelude::*, Cursor}};
use quick_xml::{events::Event, reader::Reader};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

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

pub fn parse_level(level_string: String) -> Level {
    let a = level_string.split(';').collect::<Vec<&str>>();
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

pub fn stringify_level(level: Level) -> String {
    let mut level_string = String::from("KS38,");

    for i in level.colors.iter() {
        let k = i.keys().collect::<Vec<&String>>();
        let v = i.values().collect::<Vec<&String>>();

        let mut is_first = true;
        for j in 0..k.len() {
            if !is_first { level_string.push('_'); } else { is_first = false; }
            level_string.push_str(k[j]);
            level_string.push('_');
            level_string.push_str(v[j]);
        }
        level_string.push('|');
    }

    for i in level.data.iter() {
        level_string.push(',');
        level_string.push_str(i.0);
        level_string.push(',');
        level_string.push_str(i.1);
    }
    level_string.push(';');

    for i in level.objects.iter() {
        let k = i.keys().collect::<Vec<&String>>();
        let v = i.values().collect::<Vec<&String>>();

        let mut is_first = true;
        for j in 0..k.len() {
            if !is_first { level_string.push(','); } else { is_first = false; }
            level_string.push_str(k[j]);
            level_string.push(',');
            level_string.push_str(v[j]);
        }
        level_string.push(';');
    }

    level_string
}

pub fn encode_level(level_string: String) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(level_string.as_bytes()).expect("Failed to write data");
    let compressed = encoder.finish().expect("Failed to compress data");
    let encoded_bytes = URL_SAFE.encode(compressed);

    encoded_bytes
}
