use advent_of_code::solutions::day_one::{parser, secret_decoder_alpha, secret_decoder_beta};

fn main() {
    let test_nums = vec![-100];
    assert_eq!(&1, &secret_decoder_beta(&test_nums));

    let test_nums = vec![-20, -30];
    assert_eq!(&1, &secret_decoder_beta(&test_nums));

    let test_nums = vec![-30, 80];
    assert_eq!(&1, &secret_decoder_beta(&test_nums));

    let test_nums = vec![-30, 280];
    assert_eq!(&3, &secret_decoder_beta(&test_nums));

    let test_nums = vec![-50, 0, 0];
    assert_eq!(&1, &secret_decoder_beta(&test_nums));

    let num_parse = parser("./data/day_1.txt");
    match num_parse {
        Ok(nums) => {
            println!("Alpha secret is {}", secret_decoder_alpha(&nums));
            println!("Beta secret is {}", secret_decoder_beta(&nums));
        }
        Err(e) => {
            println!("unsuccessful parse {}", e);
        }
    }
}
