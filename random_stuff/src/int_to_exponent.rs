///
/// 13214100000 -> 1.32141 x 10^10
///

pub fn int_to_exponent(input: &String) -> String {
    let mut expo = 0;

    for last in input.as_bytes().into_iter().rev() {
        if last != &('0' as u8) {
            break;
        } else {
            expo = expo + 1;
        }
    }

    if let Some(new_format) = input.get(0..input.len() - expo) {
        expo = expo + new_format.len() - 1;
        let mut new_format = new_format.to_string();
        if new_format.len() > 1 {
            String::insert(&mut new_format, 1, '.');
        }

        return format!("{new_format} x 10^{expo}");
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use crate::int_to_exponent::int_to_exponent;

    #[test]
    fn int_to_exponent_test() {
        assert_eq!(
            int_to_exponent(&"1230000".to_string()),
            "1.23 x 10^6".to_string()
        );
        assert_eq!(
            int_to_exponent(&"14213000".to_string()),
            "1.4213 x 10^7".to_string()
        );
        assert_eq!(
            int_to_exponent(&"91200120000".to_string()),
            "9.120012 x 10^10".to_string()
        );
        assert_eq!(int_to_exponent(&"No number".to_string()), "N.o number x 10^8".to_string());
    }
}
