use crate::constants::{
    order_option_to_string, pdf_direction_to_string, OrderOption, PdfDirection,
};
use printpdf::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn select_pdf_direction() -> PdfDirection {
    println!("请输出导出PDF时的页面方向（输入数字即可，仅支持单选）：");
    println!("选择横向时，会展示较为完整的单词释义");
    println!(
        "   [0]. {}",
        pdf_direction_to_string(&PdfDirection::Longitudinal)
    );
    println!(
        "   [1]. {}",
        pdf_direction_to_string(&PdfDirection::Horizontal)
    );
    loop {
        print!("您的选择是：");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "0" => return PdfDirection::Longitudinal,
            "1" => return PdfDirection::Horizontal,
            _ => println!("输入错误，请重试。"),
        }
    }
}

fn truncate_text(text: &str, max_width: f64, font: &IndirectFontRef, font_size: f64) -> String {
    let mut width = 0.0;
    let ellipsis = "...";
    let ellipsis_width = font_size * 0.5; // 粗略估计省略号宽度
    let available_width = max_width - ellipsis_width;

    let mut truncated = String::new();
    for c in text.chars() {
        width += font_size * 0.5; // 假设每个字符宽度为字体大小的一半（简化的宽度计算）
        if width <= available_width {
            truncated.push(c);
        } else {
            break;
        }
    }
    if width > available_width {
        truncated.push_str(ellipsis);
    }
    truncated
}

pub fn save_as_pdf(
    all_words: &HashMap<String, String>,
    order_choice: &OrderOption,
) -> Result<(), Box<dyn std::error::Error>> {
    let direction = select_pdf_direction();
    let (page_width, page_height, col_widths) = match direction {
        PdfDirection::Longitudinal => (Mm(210.0), Mm(297.0), vec![Mm(20.0), Mm(50.0), Mm(120.0)]),
        PdfDirection::Horizontal => (Mm(297.0), Mm(210.0), vec![Mm(20.0), Mm(50.0), Mm(200.0)]),
    };

    let (doc, page1, layer1) = PdfDocument::new("Words Export", page_width, page_height, "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    current_layer.set_font(&font, 12.0);

    let line_height = Mm(10.0);
    let table_width = Mm(col_widths.iter().map(|mm| mm.0).sum());
    let start_x = (page_width - table_width) / 2.0;
    let mut y_pos = page_height - Mm(20.0);

    for (idx, (word, interpret)) in all_words.iter().enumerate() {
        if y_pos < Mm(20.0) {
            let (page, layer) = doc.add_page(page_width, page_height, "Layer 1");
            let current_layer = doc.get_page(page).get_layer(layer);
            current_layer.set_font(&font, 12.0);
            y_pos = page_height - Mm(20.0);
        }
        current_layer.add_shape(Line {
            points: vec![
                (Point::new(start_x, y_pos), false),
                (Point::new(start_x + table_width, y_pos), false),
            ],
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,  // 添加缺失的字段
        });

        let mut x_pos = start_x;
        current_layer.use_text((idx + 1).to_string(), 12.0, x_pos, y_pos - Mm(5.0), &font);
        x_pos = x_pos + col_widths[0];
        current_layer.use_text(word, 12.0, x_pos, y_pos - Mm(5.0), &font);
        x_pos = x_pos + col_widths[1];
        let truncated_interpret = truncate_text(interpret, col_widths[2].0, &font, 12.0);
        current_layer.use_text(&truncated_interpret, 12.0, x_pos, y_pos - Mm(5.0), &font);

        y_pos = y_pos - line_height;
    }

    let current_date = chrono::Local::now().format("%Y_%m_%d").to_string();
    let file_name = format!(
        "words-{}-{}-{}.pdf",
        current_date,
        order_option_to_string(order_choice),
        pdf_direction_to_string(&direction)
    );
    doc.save(&mut BufWriter::new(File::create(&file_name)?))?;
    println!("PDF 文件已保存至: {}", file_name);
    Ok(())
}
