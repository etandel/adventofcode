use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stream {
    data: Vec<u8>,
    nnibbles: usize,
}

impl Stream {
    fn from_str(s: &str) -> Self {
        let nibbles: Vec<u8> = s
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect();
        let mut condensed: Vec<u8> = Vec::new();
        let mut chunks = nibbles.chunks(2);

        while let Some(&[n1, n2]) = chunks.next() {
            condensed.push((n1 << 4) | n2);
        }

        Self {
            data: condensed,
            nnibbles: nibbles.len(),
        }
    }

    fn iter(self) -> IterBitStream {
        IterBitStream::new(self)
    }

    fn get(&self, pos: usize) -> Option<u8> {
        if pos >= self.nnibbles * 4 {
            None
        } else {
            let shift = pos % 8;
            let mask = 0b10000000 >> shift;
            let index = pos / 8;
            let bit = (self.data[index] & mask) >> (8 - shift - 1);
            Some(bit)
        }
    }
}

#[derive(Debug)]
struct IterBitStream {
    bit_stream: Stream,
    pos: usize,
}

impl IterBitStream {
    fn new(bit_stream: Stream) -> Self {
        Self { bit_stream, pos: 0 }
    }
}

impl Iterator for IterBitStream {
    type Item = u8;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = self.bit_stream.get(self.pos);
        self.pos += 1;
        ret
    }
}

fn parse_n<I>(i: &mut I, n: usize) -> u64
where
    I: Iterator<Item = u8>,
{
    let mut res: u64 = 0;

    for _ in 0..n {
        let bit = i.next().unwrap();
        res <<= 1;
        res |= bit as u64;
    }

    res
}

enum PLen {
    Bits(u64),
    Packets(u64),
}

/*
#[derive(Debug, Clone, PartialEq, Eq)]
enum PVal {
    Packet(Packet),
    Const(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Packet {
    version: u64,
    type_id: u64,
    val: Vec<PVal>,
}
*/

fn parse_const<I>(i: &mut I) -> u64
where
    I: Iterator<Item = u8>,
{
    let mut val = 0;

    while let Some(1) = i.next() {
        val += parse_n(i, 4);
    }
    val += parse_n(i, 4);

    val
}

fn sum_versions(i: &mut IterBitStream, expected: PLen) -> u64 {
    let mut sum = 0;

    match expected {
        PLen::Bits(mut n) => {
            let mut pos_before = i.pos;
            while n > 0 {
                sum += parse_n(i, 3);
                let type_id = parse_n(i, 3);

                if type_id == 4 {
                    // skip type 4 const bits
                    parse_const(i);
                } else {
                    let len_type = parse_n(i, 1);

                    if len_type == 0 {
                        let len = parse_n(i, 15);
                        sum += sum_versions(i, PLen::Bits(len));
                    } else {
                        let len = parse_n(i, 11);
                        sum += sum_versions(i, PLen::Packets(len));
                    }
                };

                let delta = i.pos - pos_before;
                pos_before = i.pos;
                n -= delta as u64;
            }
        }
        PLen::Packets(n) => {
            for _ in 0..n {
                sum += parse_n(i, 3);
                let type_id = parse_n(i, 3);

                if type_id == 4 {
                    // skip type 4 const bits
                    parse_const(i);
                } else {
                    let len_type = parse_n(i, 1);

                    if len_type == 0 {
                        let len = parse_n(i, 15);
                        sum += sum_versions(i, PLen::Bits(len));
                    } else {
                        let len = parse_n(i, 11);
                        sum += sum_versions(i, PLen::Packets(len));
                    }
                };
            }
        }
    }

    sum
}

/*
fn sum_versions(p: &Packet) -> u64 {
    let mut version = p.version;

    if p.type_id != 4 {
        version += p
            .val
            .iter()
            .map(
                |pval| match pval {
                    PVal::Packet(p) => sum_versions(p),
                    _ => panic!("Invalid packet: {:?}", p)
            })
            .sum::<u64>()
    }

    version
}
*/

fn read_stream<P>(path: P) -> Stream
where
    P: AsRef<Path>,
{
    Stream::from_str(&fs::read_to_string(path).unwrap())
}

fn part1() {
    let stream = read_stream("input.txt");

    let sum = sum_versions(&mut stream.iter(), PLen::Packets(1));

    println!("{}", sum);
}

fn part2() {
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
    use super::*;

    fn assert_bit_stream_eq(original: &str, expected_data: Vec<u8>, expected_total_nibbles: usize) {
        let expected = Stream {
            data: expected_data,
            nnibbles: expected_total_nibbles,
        };
        let got = Stream::from_str(original);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_bit_stream_from_str() {
        assert_bit_stream_eq("", vec![], 0);
        assert_bit_stream_eq("D2FE28", vec![0b11010010, 0b11111110, 0b00101000], 6);
        assert_bit_stream_eq(
            "38006F45291200",
            vec![
                0b00111000, 0b00000000, 0b01101111, 0b01000101, 0b00101001, 0b00010010, 0b00000000,
            ],
            14,
        );
    }

    #[test]
    fn test_bit_stream_get_bit() {
        let stream = Stream {
            data: vec![0b11010010, 0b11111110, 0b00101000],
            nnibbles: 6,
        };
        assert_eq!(stream.get(0), Some(1));
        assert_eq!(stream.get(1), Some(1));
        assert_eq!(stream.get(2), Some(0));

        assert_eq!(stream.get(4), Some(0));
        assert_eq!(stream.get(5), Some(0));
        assert_eq!(stream.get(6), Some(1));

        assert_eq!(stream.get(8), Some(1));
        assert_eq!(stream.get(9), Some(1));
        assert_eq!(stream.get(15), Some(0));

        assert_eq!(stream.get(16), Some(0));
        assert_eq!(stream.get(17), Some(0));

        assert_eq!(stream.get(20), Some(1));
        assert_eq!(stream.get(23), Some(0));

        assert_eq!(stream.get(24), None);
        assert_eq!(stream.get(25), None);
        assert_eq!(stream.get(100), None);
    }

    #[test]
    fn test_iter_bit_stream() {
        let stream = Stream {
            data: vec![0b11010010, 0b11111110, 0b00101000],
            nnibbles: 6,
        };
        let got: Vec<u8> = stream.iter().collect();
        let expected = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        assert_eq!(got, expected);
    }

    #[test]
    fn test_parse_n() {
        let stream = Stream {
            data: vec![0b11010010, 0b11111110, 0b00101000],
            nnibbles: 6,
        };
        assert_eq!(
            parse_n(&mut stream.clone().iter(), 8),
            stream.data[0] as u64
        );
        assert_eq!(
            parse_n(&mut stream.clone().iter(), 4),
            ((stream.data[0] & 0xF0) >> 4) as u64
        );
    }

    #[test]
    fn test_sum_versions() {
        fn assert_sum(inp: &str, expected: u64) {
            let s = Stream::from_str(inp);
            let got = sum_versions(&mut s.iter(), PLen::Packets(1));
            assert_eq!(got, expected);
        }

        assert_sum("D2FE28", 6);
        assert_sum("EE00D40C823060", 7 + 2 + 4 + 1);
        assert_sum("38006F45291200", 1 + 6 + 2);

        assert_sum("8A004A801A8002F478", 16);
        assert_sum("620080001611562C8802118E34", 12);
        assert_sum("C0015000016115A2E0802F182340", 23);
        assert_sum("A0016C880162017C3686B18A3D4780", 31);
    }
}
