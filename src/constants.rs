#[derive(PartialEq)]
pub enum OrderOption {
    DefaultOrder,
    ShuffleOrder,
    AlphabeticalOrder,
    NoExport,
}

pub fn order_option_to_string(option: &OrderOption) -> &'static str {
    match option {
        OrderOption::DefaultOrder => "默认顺序",
        OrderOption::ShuffleOrder => "打乱顺序",
        OrderOption::AlphabeticalOrder => "字典顺序",
        OrderOption::NoExport => "不导出至文件",
    }
}

pub enum FormatOption {
    Csv,
    Pdf,
}

pub fn format_option_to_string(option: &FormatOption) -> &'static str {
    match option {
        FormatOption::Csv => "CSV",
        FormatOption::Pdf => "PDF",
    }
}

pub enum PdfDirection {
    Longitudinal,
    Horizontal,
}

pub fn pdf_direction_to_string(direction: &PdfDirection) -> &'static str {
    match direction {
        PdfDirection::Longitudinal => "纵向",
        PdfDirection::Horizontal => "横向",
    }
}
