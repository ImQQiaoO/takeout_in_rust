use crate::constants::{order_option_to_string, OrderOption};
use crate::insertion_order_map::InsertionOrderMap;
use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use winapi::um::winnls::CP_ACP;
#[cfg(windows)]
use winapi::um::stringapiset::WideCharToMultiByte;

fn encode_string(s: &str) -> Vec<u8> { 
    if cfg!(target_os = "windows") {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let mut buffer = vec![0u8; utf16.len() * 2];
        let len = unsafe {
            WideCharToMultiByte(
                CP_ACP,
                0,
                utf16.as_ptr(),
                utf16.len() as i32,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i32,
                std::ptr::null(),
                std::ptr::null_mut(),
            )
        };
        
        if len > 0 {
            buffer.truncate(len as usize);
            buffer
        } else {
            panic!("Failed to encode string to ANSI");
        }
    } else {
        s.as_bytes().to_vec()
    }
}

pub fn save_as_csv(
    all_words: &InsertionOrderMap<String, String>,
    order_choice: &OrderOption,
) -> io::Result<()> {
    let current_date = chrono::Local::now().format("%Y_%m_%d").to_string();
    let file_name = format!(
        "words-{}-{}.csv",
        current_date,
        order_option_to_string(order_choice)
    );
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);

    if all_words.len() == 0 {
        return Ok(());
    }

    let mut index = 0;
    for (word, interpret) in all_words.iter() {
        index += 1;
        let record = [index.to_string(), word.clone(), interpret.clone()];
        wtr.write_record(&record)?;
    }

    let csv_utf8 = wtr.into_inner().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("CSV writing error: {}", e))
    })?;

    let csv_string = String::from_utf8(csv_utf8)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("UTF-8 conversion error: {}", e)))?;
    let csv_ansi = encode_string(&csv_string);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_name)?;
    file.write_all(&csv_ansi)?;
    println!("CSV 文件已保存至: {}", file_name);
    Ok(())
}
