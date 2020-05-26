use pest::Parser;

#[derive(Parser)]
#[grammar = "ass1.pest"]
pub struct Ass1Parser;

pub fn parse_file(file_text: &str) -> (Vec<f64>, String) {
    let mut array: Vec<f64> = Vec::with_capacity(256 * 256);
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");

    let mut file = match Ass1Parser::parse(Rule::ARRAY, &file_text) {
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
            Rule::FLOATING_POINT_NUMBER => array.push(val.as_str().parse::<f64>().unwrap()),
            Rule::OPENING_SQUARE_BRACKET => (),
            Rule::CLOSING_SQUARE_BRACKET => (),
            Rule::COMMA => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    (array, errors)
}

pub fn convert_to_csv(file_text: &str, img_width: usize) -> (String, String) {
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");
    let mut csv = String::new();
    let mut comma_count = 0;

    let mut file = match Ass1Parser::parse(Rule::ARRAY, &file_text) {
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
            Rule::FLOATING_POINT_NUMBER => csv.push_str(val.as_str()),
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

pub fn convert_to_ascii_art(
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

    let mut file = match Ass1Parser::parse(Rule::ARRAY, &file_text) {
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
                let num = val.as_str().parse::<f64>().unwrap();
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

pub fn convert_to_grayscale_img(file_text: &str) -> (Vec<u8>, String) {
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");
    let mut img = Vec::new();

    let mut file = match Ass1Parser::parse(Rule::ARRAY, &file_text) {
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
                let num = val.as_str().parse::<f64>().unwrap();
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
