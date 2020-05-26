use pest::Parser;

#[derive(Parser)]
#[grammar = "file.pest"]
struct ImageFileParser;

pub fn parse_file(file_text: &str) -> (Vec<u8>, String) {
    let mut errors = String::from("");
    let file_text = file_text.replace(",", ",\n");

    let mut file = match ImageFileParser::parse(Rule::IMAGE_FILE, &file_text) {
        Ok(val) => val,
        Err(err) => {
            errors = format!("{}", err);
            return (Vec::new(), errors);
        }
    };
    let file = file // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    let mut file_type = String::new();
    let mut img_height = 0;
    let mut img_width = 0;

    let mut reds: Vec<u8> = vec![];
    let mut greens: Vec<u8> = vec![];
    let mut blues: Vec<u8> = vec![];

    let mut greys: Vec<u8> = vec![];

    for val in file.into_inner() {
        match val.as_rule() {
            Rule::IMAGE_FILE_TYPE => {
                file_type = String::from(val.as_str());
            }
            Rule::IMAGE_HEIGHT => {
                img_height = val.as_str().parse().unwrap();
            }
            Rule::IMAGE_WIDTH => {
                img_width = val.as_str().parse().unwrap();
            }
            Rule::RED_COMPONENT => {
                reds.push(val.as_str().parse::<u8>().unwrap());
            }
            Rule::BLUE_COMPONENT => {
                blues.push(val.as_str().parse::<u8>().unwrap());
            }
            Rule::GREEN_COMPONENT => {
                greens.push(val.as_str().parse::<u8>().unwrap());
            }
            Rule::GREY_COMPONENT => {
                greys.push(val.as_str().parse().unwrap());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let mut array: Vec<u8> = Vec::with_capacity(img_height * img_width * 3);

    let all_colors = (reds.iter().zip(greens.iter())).zip(blues.iter());

    for ((r, g), b) in all_colors {
        array.push(*r);
        array.push(*g);
        array.push(*b);
    }

    (array, errors)
}
