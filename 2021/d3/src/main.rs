use std::env;
use std::fs;
use std::path::Path;

type Readout = u16;

fn read_report<P>(path: P) -> (Vec<Readout>, usize)
where
    P: AsRef<Path>,
{
    let raw = fs::read_to_string(path).unwrap();

    let bits_per_readout = raw.lines().next().unwrap().len();

    let report = raw
        .lines()
        .filter_map(|l| Readout::from_str_radix(l, 2).ok())
        .collect();

    (report, bits_per_readout)
}

fn get_bit_as_readout(r: Readout, bit: usize) -> Readout {
    (r & (1 << bit)) >> bit
}

fn get_most_common_bits(report: &[Readout], bits_per_readout: usize) -> Readout {
    let report_len = report.len();

    (0..bits_per_readout)
        .map(|bit| {
            let sum: Readout = report
                .iter()
                .map(|readout| get_bit_as_readout(*readout, bit))
                .sum();

            (bit, sum)
        })
        .map(|(bit, sum)| {
            let most_common_bit: Readout = (sum as usize > report_len / 2).into();
            most_common_bit << bit
        })
        .sum()
}

fn multiply_with_complement(x: Readout, bits_per_readout: usize) -> u64 {
    let comp = (!x) & (Readout::MAX >> (Readout::BITS - bits_per_readout as u32));
    x as u64 * comp as u64
}

fn part1() {
    let (report, bits_per_readout) = read_report("input.txt");
    let gamma = get_most_common_bits(&report, bits_per_readout);
    println!("{}", multiply_with_complement(gamma, bits_per_readout));
}

fn part2() {
    let (report, bits_per_readout) = read_report("input.txt");
    let _most_common = get_most_common_bits(&report, bits_per_readout);
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}
