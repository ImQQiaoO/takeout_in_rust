mod constants;
mod csv_formatter;
mod insertion_order_map;
mod pdf_formatter;

use crate::constants::*;
use crate::csv_formatter::save_as_csv;
use crate::insertion_order_map::InsertionOrderMap;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

fn deserialize_u32_from_f64<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let f = f64::deserialize(deserializer)?; // 解析为 f64
    Ok(f as u32) // 转换为 u32，丢弃小数部分
}

#[derive(Deserialize)]
struct PageInfo {
    #[serde(rename = "totalPage")]
    #[serde(deserialize_with = "deserialize_u32_from_f64")]
    total_page: u32,
}

#[derive(Deserialize)]
struct WordInfo {
    word: String,
    interpret: String,
}

#[derive(Deserialize)]
struct DataBody {
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    #[serde(rename = "wordList")]
    word_list: Vec<WordInfo>,
}

#[derive(Deserialize)]
struct ApiResponse {
    data_body: DataBody,
}

fn fetch_page_data(
    url: &str,
    headers: &HashMap<&str, String>,
    retries: u32,
) -> Option<ApiResponse> {
    let client = Client::new();
    let header_map: HeaderMap = headers
        .iter()
        .map(|(k, v)| {
            let header_name = HeaderName::try_from(*k).expect("Invalid header name");
            let header_value = HeaderValue::from_str(v).expect("Invalid header value");
            (header_name, header_value)
        })
        .collect();

    for _ in 0..retries {
        match client
            .get(url)
            .headers(header_map.clone())
            .timeout(Duration::from_secs(10))
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<ApiResponse>() {
                        Ok(data) => return Some(data),
                        Err(e) => println!("JSON 解析失败: {}", e),
                    }
                } else {
                    println!("请求失败: {}", response.status());
                }
            }
            Err(e) => println!("请求失败: {}", e),
        }
        std::thread::sleep(Duration::from_secs_f32(rand::random::<f32>() * 3.0 + 2.0));
    }
    None
}

fn fetch_all_words(headers: &HashMap<&str, String>) -> InsertionOrderMap<String, String> {
    let mut all_words: InsertionOrderMap<String, String> = InsertionOrderMap::new();
    let base_url = "https://www.bbdc.cn/api/user-new-word?page={}";
    let first_page_url = base_url.replace("{}", "0");
    if let Some(data) = fetch_page_data(&first_page_url, headers, 3) {
        let total_pages = data.data_body.page_info.total_page;
        for word_info in data.data_body.word_list {
            let interpret = word_info.interpret.replace('\n', " ");
            all_words.insert(word_info.word, interpret);
        }
        for i in 1..total_pages {
            let progress_chars = ((i + 1) * 50) / total_pages;
            let percentage = ((i + 1) * 100) / total_pages;
            print!(
                "\r进度：|{:50}| {}%",
                "#".repeat(progress_chars as usize),
                percentage
            );
            io::stdout().flush().unwrap();
            let page_url = base_url.replace("{}", &i.to_string());
            if let Some(page_data) = fetch_page_data(&page_url, headers, 3) {
                for word_info in page_data.data_body.word_list {
                    let interpret = word_info.interpret.replace('\n', " ");
                    all_words.insert(word_info.word, interpret);
                }
            } else {
                break;
            }
        }
        println!();
    }
    all_words
}

fn select_output_word_order(all_words: &mut InsertionOrderMap<String, String>) -> OrderOption {
    println!("请输出导出至文件时的单词顺序（输入数字即可，仅支持单选）：");
    for i in 0..4 {
        println!(
            "   [{}]. {}",
            i,
            order_option_to_string(&match i {
                0 => OrderOption::DefaultOrder,
                1 => OrderOption::ShuffleOrder,
                2 => OrderOption::AlphabeticalOrder,
                3 => OrderOption::NoExport,
                _ => unreachable!(),
            })
        );
    }
    loop {
        print!("您的选择是：");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "0" => return OrderOption::DefaultOrder,
            "1" => {
                all_words.shuffle();
                return OrderOption::ShuffleOrder;
            }
            "2" => {
                all_words.sort_by_key();
                return OrderOption::AlphabeticalOrder;
            }
            "3" => return OrderOption::NoExport,
            _ => println!("输入错误，请重试。"),
        }
    }
}

fn select_format() -> FormatOption {
    println!("请输出导出至文件时的文件形式（输入数字即可，仅支持单选）：");
    println!("   [0]. CSV");
    println!("   [1]. PDF");
    loop {
        print!("您的选择是：");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "0" => return FormatOption::Csv,
            "1" => return FormatOption::Pdf,
            _ => println!("输入错误，请重试。"),
        }
    }
}

fn main() {
    println!("欢迎使用不背单词导出工具！");
    print!("请输入您的不背单词的cookie，然后按回车键继续...\n");
    io::stdout().flush().unwrap();
    let mut cookie: String = String::new();
    io::stdin().read_line(&mut cookie).unwrap();
    let headers: HashMap<&str, String> = HashMap::from([("cookie", cookie.trim().to_string())]);

    let mut all_words = fetch_all_words(&headers);
    println!("单词获取成功，共 {} 个单词。", all_words.len());

    loop {
        let order_choice = select_output_word_order(&mut all_words);
        for (word, interpret) in all_words.iter() {
            println!("{} {}", word, interpret);
        }
        if order_choice != OrderOption::NoExport {
            let format_choice = select_format();
            match format_choice {
                FormatOption::Csv => {
                    if let Err(e) = save_as_csv(&all_words, &order_choice) {
                        println!("保存 CSV 失败: {}", e);
                    } else {
                        println!("此次保存成功！");
                    }
                }
                FormatOption::Pdf => {
                    if let Err(e) = pdf_formatter::save_as_pdf(&all_words, &order_choice) {
                        println!("保存 PDF 失败: {}", e);
                    } else {
                        println!("此次保存成功！");
                    }
                }
            }
        }
        if input("输入[q]退出程序，输入其他任意内容按回车键继续保存：").to_lowercase() == "q"
        {
            break;
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
