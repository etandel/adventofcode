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
    let cmp = build_comparator(0, |s, h| s > h);

    (0..bits_per_readout)
        .map(|bit| get_most_common_bit_with_comparator(report, bit, &cmp))
        .sum()
}

fn get_complement(x: Readout, bits_per_readout: usize) -> Readout {
    (!x) & (Readout::MAX >> (Readout::BITS - bits_per_readout as u32))
}

fn get_most_common_bit_with_comparator<F>(report: &[Readout], bit: usize, comparator: &F) -> Readout
where
    F: Fn(usize, usize) -> Readout,
{
    let report_len = report.len();
    let sum: Readout = report
        .iter()
        .map(|readout| get_bit_as_readout(*readout, bit))
        .sum();

    comparator(sum.into(), report_len) << bit
}

fn build_comparator<'a, F>(default: Readout, cmp: F) -> impl Fn(usize, usize) -> Readout + 'a
where
    F: Fn(usize, usize) -> bool + 'a,
{
    move |sum, report_len| {
        if sum == report_len - sum {
            default
        } else {
            cmp(sum, report_len / 2).into()
        }
    }
}

fn filter_report<F>(report: Vec<Readout>, bit: usize, comparator: &F) -> Vec<Readout>
where
    F: Fn(usize, usize) -> Readout,
{
    let most_common = get_most_common_bit_with_comparator(&report, bit, comparator);
    let expected_bit = get_bit_as_readout(most_common, bit);

    report
        .into_iter()
        .filter(|&readout| get_bit_as_readout(readout, bit) == expected_bit)
        .collect()
}

fn reduce_report<F>(mut report: Vec<Readout>, bits_per_readout: usize, comparator: F) -> Readout
where
    F: Fn(usize, usize) -> Readout,
{
    let mut bit = bits_per_readout;

    while report.len() > 1 {
        // if this panics then it means that filtering didn't
        // reduce the report to a single entry
        bit -= 1;
        report = filter_report(report, bit, &comparator);
    }

    report[0]
}

fn part1() {
    let (report, bits_per_readout) = read_report("input.txt");
    let gamma = get_most_common_bits(&report, bits_per_readout);
    let res = gamma as u64 * get_complement(gamma, bits_per_readout) as u64;
    println!("{}", res);
}

fn part2() {
    let (report, bits_per_readout) = read_report("input.txt");

    let oxygen = reduce_report(
        report.clone(),
        bits_per_readout,
        build_comparator(1, |s, h| s > h),
    );

    let co2 = reduce_report(report, bits_per_readout, build_comparator(0, |s, h| s <= h));
    println!("{}", oxygen as u64 * co2 as u64);
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
