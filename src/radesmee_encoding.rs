use std::str::from_utf8;

const BIT_REPRENSENTATIONS: [&str; 2] = [":radaesmee:", ":radesmee:"];

fn text_to_bin(text: &str) -> Vec<u8> {
    text.as_bytes()
        .into_iter()
        .map(|x| (0..8).map(move |i| x >> i & 1).rev())
        .flatten()
        .collect()
}

fn bin_to_text(bin: &Vec<u8>) -> Result<String, String> {
    match from_utf8(bin) {
        Ok(x) => Ok(x.to_string()),
        Err(_) => Err("UTF8 error".to_string()),
    }
}

fn bin_to_radesmee(bin: &Vec<u8>) -> String {
    bin.iter()
        .map(|x| BIT_REPRENSENTATIONS[*x as usize])
        .collect::<Vec<&str>>()
        .join("")
}

fn radesmee_to_bin(radesmee: &str) -> Result<Vec<u8>, String> {
    let bits: Vec<u8> = radesmee
        .split(":")
        .map(|x| ":".to_owned() + x + ":")
        .filter(|x| *x == BIT_REPRENSENTATIONS[0] || *x == BIT_REPRENSENTATIONS[1])
        .map(|x| x == BIT_REPRENSENTATIONS[1])
        .map(|x| x as u8)
        .collect();

    if bits.len() % 8 != 0 {
        return Err("number of bytes should be a multiple of 8".to_string());
    }

    let sub_vec_number = bits.len() / 8;
    Ok((0..sub_vec_number)
        .map(|i| {
            (0..8)
                .map(|j| bits[i * 8 + j] << (7 - j))
                .fold(0, |acc, x| acc | x)
        })
        .collect())
}

pub fn text_to_radesmee(text: &str) -> String {
    bin_to_radesmee(&text_to_bin(text))
}

pub fn radesmee_to_text(radesmee: &str) -> Result<String, String> {
    bin_to_text(&radesmee_to_bin(radesmee)?)
}
