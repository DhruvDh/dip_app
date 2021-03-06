#[macro_use]
extern crate serde_derive;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use console_error_panic_hook::set_once as set_panic_hook;
use pest::Parser;
use wasm_bindgen::prelude::*;

// TODO: Try turning this to inspectable to get rid of JsParsing
#[derive(Serialize)]
pub struct FileHeaderParseResponse {
    file_type: String,
    height: usize,
    width: usize,
}

#[derive(Parser)]
#[grammar = "parse.pest"]
pub struct FileParser;

macro_rules! unwrap {
    ($x:expr, $e:ident, $line_no:expr) => {
        match $x {
            Ok(val) => val,
            Err(e) => {
                match e {
                    std::num::ParseIntError { .. } => {
                        $e.push(format!("At line {}: Cannot parse as a number.", $line_no));
                    }
                    _ => {
                        $e.push(format!("At line {}: {}", $line_no, e));
                    }
                };
                0
            }
        }
    };
    ($x:expr, $e:ident, $line_no:expr, $msg:expr) => {
        match $x {
            Ok(val) => val,
            Err(e) => {
                match e {
                    std::num::ParseIntError { .. } => {
                        $e.push(format!($msg, $line_no));
                    }
                    _ => {
                        $e.push(format!("At line {}: {}", $line_no, e));
                    }
                };
                0
            }
        }
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}

pub fn ass1_parse_file(file_text: &str) -> (Vec<f64>, String) {
    let mut array: Vec<f64> = Vec::with_capacity(256 * 256);
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");

    let mut file = match FileParser::parse(Rule::ASSIGNMENT_1_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (Vec::new(), errors);
        }
    };
    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::FLOATING_POINT_NUMBER => array.push(val.as_str().trim().parse::<f64>().unwrap()),
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    (array, errors)
}

pub fn ass1_convert_to_csv(file_text: &str, img_width: usize) -> (String, String) {
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");
    let mut csv = String::new();
    let mut comma_count = 0;

    let mut file = match FileParser::parse(Rule::ASSIGNMENT_1_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (String::new(), errors);
        }
    };
    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::FLOATING_POINT_NUMBER => csv.push_str(val.as_str().trim()),
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => {
                comma_count = comma_count + 1;
                if comma_count == img_width {
                    comma_count = 0;
                    csv.push_str(",\n");
                } else {
                    csv.push(',');
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    (csv, errors)
}

pub fn ass1_convert_to_ascii_art(
    file_text: &str,
    light_char: char,
    dark_char: char,
    threshold: f64,
    img_width: usize,
) -> (String, String) {
    let mut array: String = String::from("");
    let mut counter = 0;
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");

    let mut file = match FileParser::parse(Rule::ASSIGNMENT_1_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (String::new(), errors);
        }
    };
    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::FLOATING_POINT_NUMBER => {
                let num = val.as_str().trim().parse::<f64>().unwrap();
                if num <= threshold {
                    array.push(light_char);
                } else {
                    array.push(dark_char);
                }

                counter = counter + 1;
                if counter == img_width {
                    counter = 0;
                    array.push('\n');
                }
            }
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    (array, errors)
}

pub fn ass1_convert_to_grayscale_img(file_text: &str) -> (Vec<u8>, String) {
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");
    let mut img = Vec::new();

    let mut file = match FileParser::parse(Rule::ASSIGNMENT_1_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (Vec::new(), errors);
        }
    };

    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut max = std::f64::MIN;
    let mut min = std::f64::MAX;

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::FLOATING_POINT_NUMBER => {
                let num = val.as_str().trim().parse::<f64>().unwrap();
                img.push(num);
                if num > max {
                    max = num;
                }

                if num < min {
                    min = num;
                }
            }
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => {}
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let (new_max, new_min) = (255u8, 0u8);

    let m = (new_max - new_min) as f64 / (max - min);
    let b = (new_min as f64 - m) * min;

    let mut new_img: Vec<u8> = vec![];

    for x in img {
        new_img.push(255 - (m * (x as f64) + b) as u8);
        new_img.push(255 - (m * (x as f64) + b) as u8);
        new_img.push(255 - (m * (x as f64) + b) as u8);
        new_img.push(255);
    }

    (new_img, errors)
}

#[allow(unreachable_patterns)]
pub fn viewer_parse_header(file_text: &str) -> (FileHeaderParseResponse, Vec<String>) {
    let lines: Vec<&str> = file_text.lines().take(3).collect();
    let mut errors: Vec<String> = vec![];

    if lines.len() == 0 {
        errors.push(String::from(
            "I think the file is empty, splitting at newline gives 0 lines.",
        ));
        return (
            FileHeaderParseResponse {
                file_type: String::new(),
                height: 0,
                width: 0,
            },
            errors,
        );
    } else if lines.len() == 1 {
        errors.push(String::from("There is only a single line in the file, there need to be at least 3 for file_type, height, and width."));
        return (
            FileHeaderParseResponse {
                file_type: String::new(),
                height: 0,
                width: 0,
            },
            errors,
        );
    } else if lines.len() == 2 {
        errors.push(String::from("There is only 2 lines in the file, there need to be at least 3 for file_type, height, and width."));
        return (
            FileHeaderParseResponse {
                file_type: String::new(),
                height: 0,
                width: 0,
            },
            errors,
        );
    }

    let file_type = lines[0];

    match file_type {
        "RGB" => {}
        "G" => {}
        _ => {
            errors.push(String::from(
                "At line 1: Not a valid file type. Expected \"RGB\" or \"G\"",
            ));
        }
    };

    let height: usize = unwrap!(lines[1].parse(), errors, 2);
    let width: usize = unwrap!(lines[2].parse(), errors, 3);

    (
        FileHeaderParseResponse {
            file_type: String::from(file_type),
            height,
            width,
        },
        errors,
    )
}

#[wasm_bindgen(js_name = viewerParseHeader)]
pub fn viewer_parse_header_json(file_text: &str) -> Result<JsValue, JsValue> {
    let (res, errors) = viewer_parse_header(file_text);

    if errors.len() != 0 {
        let errors = errors.join("#!@");
        Err(JsValue::from(errors))
    } else {
        Ok(JsValue::from_serde(&res).unwrap())
    }
}

#[allow(unreachable_patterns)]
pub fn viewer_parse_pixels(file_text: &str) -> (Vec<u8>, Vec<String>) {
    let lines: Vec<&str> = file_text.lines().collect();

    let (header, errs) = viewer_parse_header(&file_text);
    let file_type = header.file_type;
    let height = header.height;
    let width = header.width;
    let mut errors: Vec<String> = vec![];
    for e in errs {
        errors.push(e);
    }

    if lines.len() - 3 != height * width {
        errors.push(format!(
            "Expected height ({}) * width ({}) pixels (= {}), but file has {} pixels",
            height,
            width,
            height * width,
            lines.len() - 3
        ));
    }

    let mut pixels: Vec<u8> = Vec::with_capacity(height * width);

    if file_type == "RGB" {
        for (no, line) in lines.iter().enumerate().skip(3) {
            let no = no + 1;

            let line = String::from(*line);
            let colors: Vec<&str> = line.split_whitespace().collect();

            if colors.len() != 3 {
                errors.push(format!(
                    "At line {}: Expected 3 color components, found {}.",
                    no,
                    colors.len(),
                ));

                continue;
            }

            let red: u8 = unwrap!(
                colors[0].parse(),
                errors,
                no,
                "At line {}: Cannot parse the red component as a number."
            );
            let green: u8 = unwrap!(
                colors[1].parse(),
                errors,
                no,
                "At line {}: Cannot parse the green component as a number."
            );
            let blue: u8 = unwrap!(
                colors[2].parse(),
                errors,
                no,
                "At line {}: Cannot parse the blue component as a number."
            );

            pixels.push(red);
            pixels.push(green);
            pixels.push(blue);
            pixels.push(255);
        }
    } else if file_type == "G" {
        for (no, line) in lines.iter().enumerate().skip(3) {
            let line = String::from(*line);
            let color = line.trim();

            let color: u8 = unwrap!(color.parse(), errors, no);

            pixels.push(color);
            pixels.push(color);
            pixels.push(color);
            pixels.push(255);
        }
    }

    (pixels, errors)
}

#[wasm_bindgen(js_name = viewerParsePixels)]
pub fn viewer_parse_pixels_json(file_text: &str) -> Result<Vec<u8>, JsValue> {
    let (pixels, errors) = viewer_parse_pixels(file_text);

    if errors.len() != 0 {
        let errors = errors.join("#!@");
        Err(JsValue::from(errors))
    } else {
        Ok(pixels)
    }
}

#[wasm_bindgen(js_name = ass1ParseFile)]
pub fn ass1_parse_file_json(file_text: &str) -> Result<Vec<f64>, JsValue> {
    let (res, errors) = ass1_parse_file(file_text);

    let err: String = errors.lines().collect::<Vec<&str>>().join("\n");

    if errors != "" {
        Err(JsValue::from(err))
    } else {
        Ok(res)
    }
}

#[wasm_bindgen(js_name = ass1ConvertToCsv)]
pub fn ass1_convert_to_csv_json(file_text: &str, img_width: usize) -> Result<String, JsValue> {
    let (res, errors) = ass1_convert_to_csv(file_text, img_width);

    let err: String = errors.lines().collect::<Vec<&str>>().join("#!@");
    if errors != "" {
        Err(JsValue::from(err))
    } else {
        Ok(res)
    }
}

#[wasm_bindgen(js_name = ass1ConvertToAsciiArt)]
pub fn ass1_convert_to_ascii_art_json(
    file_text: &str,
    light_char: char,
    dark_char: char,
    threshold: f64,
    img_width: usize,
) -> Result<String, JsValue> {
    let (res, errors) =
        ass1_convert_to_ascii_art(file_text, light_char, dark_char, threshold, img_width);

    let err: String = errors.lines().collect::<Vec<&str>>().join("\n");
    if errors != "" {
        Err(JsValue::from(err))
    } else {
        Ok(res)
    }
}

#[wasm_bindgen(js_name = ass1ConvertToGrayscaleImg)]
pub fn ass1_convert_to_grayscale_img_json(file_text: &str) -> Result<Vec<u8>, JsValue> {
    let (res, errors) = ass1_convert_to_grayscale_img(file_text);

    if errors != "" {
        Err(JsValue::from(errors))
    } else {
        Ok(res)
    }
}

pub fn ass2_parse_file(file_text: &str) -> (Vec<u8>, String) {
    let mut array: Vec<u8> = Vec::new();
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");

    let mut file = match FileParser::parse(Rule::ASSIGNMENT_2_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (Vec::new(), errors);
        }
    };
    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::WHOLE_NUMBER => {
                let num = val.as_str().trim().parse::<usize>().unwrap();
                let num = if num > 255 { 255u8 } else { num as u8 };
                array.push(num)
            }
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    array.shrink_to_fit();

    (array, errors)
}

#[wasm_bindgen(js_name = ass2ParseFile)]
pub fn ass2_parse_file_json(file_text: &str) -> Result<Vec<u8>, JsValue> {
    let (res, errors) = ass2_parse_file(file_text);

    if errors != "" {
        Err(JsValue::from(errors))
    } else {
        Ok(res)
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

#[wasm_bindgen(js_name = ass2DoPartA)]
pub fn ass2_do_part_a(file_text: &str, img_width: usize) -> Result<JsValue, JsValue> {
    let array = ass2_parse_file_json(file_text)?;

    let mut text = String::new();
    text.push_str("RGB\n");
    text.push_str(format!("{}\n", array.len() / img_width).as_str());
    text.push_str(format!("{}\n", img_width).as_str());

    enum Row {
        BlueGreen,
        GreenRed,
    };

    let mut row = Row::BlueGreen;

    for line in array.chunks_exact(img_width) {
        match row {
            Row::BlueGreen => {
                let mut color = Color::Blue;
                for elem in line {
                    match color {
                        Color::Blue => {
                            text.push_str(format!("0 0 {}\n", *elem).as_str());
                            color = Color::Green;
                        }
                        Color::Green => {
                            text.push_str(format!("0 {} 0\n", *elem).as_str());
                            color = Color::Blue;
                        }
                        _ => unreachable!(),
                    }
                }
                row = Row::GreenRed;
            }
            Row::GreenRed => {
                let mut color = Color::Green;
                for elem in line {
                    match color {
                        Color::Green => {
                            text.push_str(format!("0 {} 0\n", *elem).as_str());
                            color = Color::Red;
                        }
                        Color::Red => {
                            text.push_str(format!("{} 0 0\n", *elem).as_str());
                            color = Color::Green;
                        }
                        _ => unreachable!(),
                    }
                }
                row = Row::BlueGreen;
            }
        }
    }

    Ok(JsValue::from(text))
}

#[wasm_bindgen(js_name = ass2DoPartB)]
pub fn ass2_do_part_b(file_text: &str, img_width: usize) -> Result<JsValue, JsValue> {
    let array = ass2_parse_file_json(file_text)?;
    let img_height = array.len() / img_width;

    let mut text = String::new();
    text.push_str("RGB\n");
    text.push_str(format!("{}\n", img_height / 2).as_str());
    text.push_str(format!("{}\n", img_width / 2).as_str());

    let mut stacked_array: Vec<Vec<u8>> = Vec::with_capacity(img_width);

    for i in 0..img_height {
        let mut current_row: Vec<u8> = Vec::with_capacity(img_width);
        for j in 0..img_width {
            current_row.push(array[i * img_height + j]);
        }
        stacked_array.push(current_row);
    }

    let stacked_array = stacked_array;

    for i in (0..img_height).step_by(2) {
        let row_1 = &stacked_array[i];
        let row_2 = &stacked_array[i + 1];

        for j in (0..img_width).step_by(2) {
            let blue = row_1[j];
            let green_1 = row_1[j + 1];
            let green_2 = row_2[j];
            let red = row_2[j + 1];

            let green: usize = (green_1 as usize + green_2 as usize) / 2;
            let green = green as u8;

            text.push_str(format!("{} {} {}\n", red, green, blue).as_str());
        }
    }

    Ok(JsValue::from(text))
}

#[wasm_bindgen(js_name = ass2DoPartC)]
pub fn ass2_do_part_c(file_text: &str, img_width: usize) -> Result<JsValue, JsValue> {
    let array = ass2_parse_file_json(file_text)?;
    let img_height = array.len() / img_width;

    let mut text = String::new();
    text.push_str("RGB\n");
    text.push_str(format!("{}\n", img_height - 1).as_str());
    text.push_str(format!("{}\n", img_width - 1).as_str());

    enum RowType {
        A,
        B,
    }

    enum GridType {
        BGGR,
        GBRG,
        GRBG,
        RGGB,
    };

    let mut row = RowType::A;

    macro_rules! index {
        ($x:expr, $y:expr) => {
            (($x * img_width) + $y) as usize
        };
    };

    for i in 0..img_height - 1 {
        match row {
            RowType::A => {
                let mut grid = GridType::BGGR;

                for j in 0..img_width - 1 {
                    match grid {
                        GridType::BGGR => {
                            let blue = array[index!(i, j)];
                            let green1 = array[index!(i, j + 1)] as usize;
                            let green2 = array[index!(i + 1, j)] as usize;
                            let green = (green1 + green2) / 2;
                            let green = green as u8;
                            let red = array[index!(i + 1, j + 1)];

                            text.push_str(format!("{} {} {}\n", red, green, blue).as_str());
                            grid = GridType::GBRG;
                        }
                        GridType::GBRG => {
                            let blue = array[index!(i, j + 1)];
                            let green1 = array[index!(i, j)] as usize;
                            let green2 = array[index!(i + 1, j + 1)] as usize;
                            let green = (green1 + green2) / 2;
                            let green = green as u8;
                            let red = array[index!(i + 1, j)];

                            text.push_str(format!("{} {} {}\n", red, green, blue).as_str());
                            grid = GridType::BGGR;
                        }
                        GridType::GRBG => panic!("I should not be at this RowType (GRBG)"),
                        GridType::RGGB => panic!("I should not be at this RowType (RGGB)"),
                    };
                }

                row = RowType::B;
            }
            RowType::B => {
                let mut grid = GridType::GRBG;

                for j in 0..img_width - 1 {
                    match grid {
                        GridType::BGGR => panic!("I should not bee at this RowType (BGGR)"),
                        GridType::GBRG => panic!("I should not be at this RowType (GBRG)"),
                        GridType::GRBG => {
                            let blue = array[index!(i + 1, j)];
                            let green1 = array[index!(i, j)] as usize;
                            let green2 = array[index!(i + 1, j + 1)] as usize;
                            let green = (green1 + green2) / 2;
                            let green = green as u8;
                            let red = array[index!(i, j + 1)];

                            text.push_str(format!("{} {} {}\n", red, green, blue).as_str());
                            grid = GridType::RGGB;
                        }
                        GridType::RGGB => {
                            let blue = array[index!(i + 1, j + 1)];
                            let green1 = array[index!(i, j + 1)] as usize;
                            let green2 = array[index!(i + 1, j)] as usize;
                            let green = (green1 + green2) / 2;
                            let green = green as u8;
                            let red = array[index!(i, j)];

                            text.push_str(format!("{} {} {}\n", red, green, blue).as_str());
                            grid = GridType::GRBG;
                        }
                    };
                }

                row = RowType::A;
            }
        };
    }

    Ok(JsValue::from(text))
}

pub fn ass3_compute_kernel(
    pixels: Vec<u8>,
    kernel_height: u32,
    kernel_width: u32,
    kernel: Vec<f32>,
    img_height: u32,
    img_width: u32,
) -> Vec<u8> {
    let scale_factor: f32 = kernel.iter().sum();

    let scale_factor = if scale_factor < 1.0 && scale_factor > -1.0 {
        1.0
    } else {
        scale_factor
    };

    let new_height = img_height - kernel_height + 1;
    let new_width = img_width - kernel_width + 1;

    let mut new_pixels: Vec<u8> = Vec::with_capacity((new_height * new_width) as usize);

    let kernel_vertical_offset: i32 = kernel_height as i32 / 2;
    let kernel_horizontal_offset: i32 = kernel_width as i32 / 2;

    let kernel_vertical_offset = if kernel_height % 2 == 0 {
        (-kernel_vertical_offset..kernel_vertical_offset).collect::<Vec<i32>>()
    } else {
        (-kernel_vertical_offset..=kernel_vertical_offset).collect::<Vec<i32>>()
    };

    let kernel_horizontal_offset = if kernel_width % 2 == 0 {
        (-kernel_horizontal_offset..kernel_horizontal_offset).collect::<Vec<i32>>()
    } else {
        (-kernel_horizontal_offset..=kernel_horizontal_offset).collect::<Vec<i32>>()
    };

    let mut img = vec![];
    for i in 0..img_height {
        let mut row = vec![];
        for j in (0..img_width * 4).step_by(4) {
            row.push((
                pixels[(i * (img_width * 4) + j + 0) as usize],
                pixels[(i * (img_width * 4) + j + 1) as usize],
                pixels[(i * (img_width * 4) + j + 2) as usize],
            ))
        }
        img.push(row);
    }

    let mut stacked_kernel = vec![];
    for i in 0..kernel_height {
        let mut row = vec![];
        for j in 0..kernel_width {
            row.push(kernel[(i * kernel_width + j) as usize] / scale_factor);
        }
        stacked_kernel.push(row);
    }

    let kernel = stacked_kernel;

    for i in kernel_height / 2..new_height + (kernel_height / 2) {
        for j in kernel_width / 2..new_width + (kernel_width / 2) {
            let mut red = 0f32;
            let mut green = 0f32;
            let mut blue = 0f32;

            for (k_row, ki) in kernel_vertical_offset.iter().enumerate() {
                for (k_col, kj) in kernel_horizontal_offset.iter().enumerate() {
                    let ki = *ki;
                    let kj = *kj;

                    let (r, g, b) = img[(i as i32 + ki) as usize][(j as i32 + kj) as usize];
                    red = red + (kernel[k_row][k_col] * r as f32);
                    green = green + (kernel[k_row][k_col] * g as f32);
                    blue = blue + (kernel[k_row][k_col] * b as f32);
                }
            }
            new_pixels.push(red as u8);
            new_pixels.push(green as u8);
            new_pixels.push(blue as u8);
            new_pixels.push(255u8);
        }
    }

    new_pixels
}

#[derive(Serialize)]
pub struct KernelParseResponse {
    kernel_format: String,
    height: u32,
    width: u32,
    value: Vec<f32>,
    kernel: Vec<Vec<f32>>,
    kernelParseErrors: String,
    kernelParseSuccessful: bool,
}

#[wasm_bindgen(js_name = ass3ParseKernel)]
pub fn ass3_parse_kernel(text: &str) -> Result<JsValue, JsValue> {
    let mut file = match FileParser::parse(Rule::KERNEL, &text) {
        Ok(val) => val,
        Err(err) => {
            let errors = format!("{}", err);
            return Err(JsValue::from(errors));
        }
    };

    let mut value: Vec<f32> = vec![];

    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut kernel_format = String::new();
    let mut num_closings = 0u32;

    let mut widths = vec![0; 99];

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::KERNEL => (),
            Rule::EOI => (),
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::KERNEL_2D_ARRAY => {
                kernel_format.push_str("2D Array");
                for child_val in val.into_inner() {
                    match child_val.as_rule() {
                        Rule::OPENING_SQUARE_BRACKET => (),
                        Rule::CLOSING_SQUARE_BRACKET => num_closings += 1,
                        Rule::FLOATING_POINT_NUMBER => {
                            value.push(child_val.as_str().parse::<f32>().unwrap());
                            widths[num_closings as usize] += 1;
                        }
                        Rule::START_OF_NEW_ROW => (),
                        Rule::COMMA => (),
                        _ => {
                            log(format!(
                                "Should not have reached child_val {:#?}",
                                child_val.as_rule()
                            )
                            .as_str());
                            unreachable!()
                        }
                    }
                }
            }
            Rule::SEMICOLON_SEPARATED_KERNEL => {
                kernel_format.push_str("Semicolon Separated");
                for child_val in val.into_inner() {
                    match child_val.as_rule() {
                        Rule::OPENING_SQUARE_BRACKET => (),
                        Rule::CLOSING_SQUARE_BRACKET => (),
                        Rule::FLOATING_POINT_NUMBER => {
                            value.push(child_val.as_str().parse::<f32>().unwrap());
                            widths[num_closings as usize] += 1;
                        }
                        Rule::START_OF_NEW_ROW => num_closings += 1,
                        Rule::COMMA => (),
                        _ => {
                            log(format!(
                                "Should not have reached child_val {:#?}",
                                child_val.as_rule()
                            )
                            .as_str());
                            unreachable!()
                        }
                    }
                }
            }
            _ => {
                log(format!("Should not have reached val {:#?}", val.as_rule()).as_str());
                unreachable!()
            }
        }
    }

    let height = if kernel_format == "2D Array" {
        num_closings - 1
    } else if kernel_format == "Semicolon Separated" {
        num_closings + 1
    } else {
        log(format!("kernel_format should not be {:?}", kernel_format).as_str());
        unreachable!()
    };

    let widths = widths
        .iter()
        .filter(|w| **w != 0)
        .map(|w| *w)
        .collect::<Vec<i32>>();

    let width = widths[0] as u32;

    if widths.iter().all(|w| width as i32 == *w) {
        Ok(JsValue::from_serde(&KernelParseResponse {
            kernel_format,
            height,
            width,
            value,
            kernel: vec![],
            kernelParseErrors: String::from(""),
            kernelParseSuccessful: true,
        })
        .unwrap())
    } else {
        Err(JsValue::from(format!(
            "The width of each row respectively is {:?} - they need to all be the same.",
            widths
        )))
    }
}

#[wasm_bindgen(js_name = ass3Compute)]
pub fn ass3_parse_and_compute(
    file_text: &str,
    kernel_height: u32,
    kernel_width: u32,
    kernel: Vec<f32>,
) -> Result<Vec<u8>, JsValue> {
    let (header, _errs) = viewer_parse_header(file_text);
    let pixels = viewer_parse_pixels_json(file_text)?;
    let filtered_pixels = ass3_compute_kernel(
        pixels,
        kernel_height,
        kernel_width,
        kernel,
        header.height as u32,
        header.width as u32,
    );

    Ok(filtered_pixels)
}
