use std::time::Instant;

type Data = Vec<Vec<SnafuNumber>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SnafuNumber {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two
}

impl SnafuNumber {
    pub fn calculate(&self) -> i64 {
        match self {
            SnafuNumber::DoubleMinus => -2,
            SnafuNumber::Minus => -1,
            SnafuNumber::Zero => 0,
            SnafuNumber::One => 1,
            SnafuNumber::Two => 2
        }
    }
}

fn regular_to_snafu(mut regular: i64) -> String {
    println!("Converting {} to snafu", regular);
    let mut snafu = String::new();
    let mut multiplier = 1;
    let mut digits = 0;
    while multiplier/2 < regular {
        multiplier *= 5;
        digits += 1;
    }
    multiplier /= 5;
    if multiplier == 0 {
        return "0".to_string();
    }

    while multiplier > 0 {
        if regular >= 2*multiplier-multiplier/2 {
            snafu.push_str("2");
            regular -= 2*multiplier;
        } else if regular >= multiplier-multiplier/2 {
            snafu.push_str("1");
            regular -= multiplier;
        } else if regular >= -multiplier/2 {
            snafu.push_str("0");
        } else if regular >= -multiplier-multiplier/2 {
            snafu.push_str("-");
            regular += multiplier;
        } else {
            snafu.push_str("=");
            regular += 2*multiplier;
        }
        multiplier /= 5;
        digits -= 1;
    }
    snafu
}

#[cfg(test)]
mod tests {
    macro_rules! test_snafu {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $expected;
                let actual = super::regular_to_snafu(input);
                assert_eq!(actual, expected);
            }
        };
    }

    test_snafu!(test_snafu_0, 0, "0");
    test_snafu!(test_snafu_1, 1, "1");
    test_snafu!(test_snafu_2, 2, "2");
    test_snafu!(test_snafu_3, 3, "1=");
    test_snafu!(test_snafu_4, 4, "1-");
    test_snafu!(test_snafu_5, 5, "10");
    test_snafu!(test_snafu_6, 6, "11");
    test_snafu!(test_snafu_7, 7, "12");
    test_snafu!(test_snafu_8, 8, "2=");
    test_snafu!(test_snafu_9, 9, "2-");
    test_snafu!(test_snafu_10, 10, "20");
    test_snafu!(test_snafu_15, 15, "1=0");
    test_snafu!(test_snafu_20, 20, "1-0");
    test_snafu!(test_snafu_2022, 2022, "1=11-2");
    test_snafu!(test_snafu_12345, 12345, "1-0---0");
    test_snafu!(test_snafu_314159265, 314159265, "1121-1110-1=0");
    test_snafu!(test_snafu_rev_1747, 1747, "1=-0-2");
    test_snafu!(test_snafu_rev_906, 906, "12111");
    test_snafu!(test_snafu_rev_198, 198, "2=0=");
    test_snafu!(test_snafu_rev_11, 11, "21");
    test_snafu!(test_snafu_rev_201, 201, "2=01");
    test_snafu!(test_snafu_rev_31, 31, "111");
    test_snafu!(test_snafu_rev_1257, 1257, "20012");
    test_snafu!(test_snafu_rev_32, 32, "112");
    test_snafu!(test_snafu_rev_353, 353, "1=-1=");
    test_snafu!(test_snafu_rev_107, 107, "1-12");
    test_snafu!(test_snafu_rev_7, 7, "12");
    test_snafu!(test_snafu_rev_3, 3, "1=");
    test_snafu!(test_snafu_rev_37, 37, "122");
}

pub fn run(data: Data) -> String {
    regular_to_snafu(data.into_iter().map(|x| {
        let v = x.into_iter().rev().fold((0, 1), |(sum, mul), v| {
            (sum + v.calculate() * mul, mul * 5)
        }).0;
        v
    }).sum())
}

#[cfg(feature = "part-two")]
pub fn run_step2(_data: Data) -> usize {
    0
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.into_iter().map(|x| {
        x.chars().map(|c| {
            match c {
                '=' => SnafuNumber::DoubleMinus,
                '-' => SnafuNumber::Minus,
                '0' => SnafuNumber::Zero,
                '1' => SnafuNumber::One,
                '2' => SnafuNumber::Two,
                _ => panic!("Unknown char")
            }
        }).collect()
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
    // data.iter().map(|x| x.as_bytes()).collect()
}