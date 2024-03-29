use std::fs;

const FILENAME: &str = "input/day16.txt";

#[allow(dead_code)]
struct Packet {
    version: u32,
    tid: u32,
    subpackets: Option<Vec<Packet>>,
    value: u64,
}

struct BitStream<'a> {
    data: &'a str,
    position: usize,
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    b.to_string()
}

impl<'a> BitStream<'a> {
    fn decode_int(self: &mut Self, mov: usize) -> u32 {
        let result = u32::from_str_radix(
            self.data.get(self.position..self.position + mov).unwrap(),
            2,
        )
        .unwrap();
        self.position += mov;
        result
    }

    fn decode_value_packet(self: &mut Self) -> u64 {
        let mut group = 0b10000u64;
        let mut value = 0;
        while group & 0b10000u64 != 0 {
            group = self.decode_int(5) as u64;
            value <<= 4;
            value += group & 0b1111u64;
        }
        value
    }

    fn decode_operator_packet(self: &mut Self, tid: u32) -> (u64, Option<Vec<Packet>>) {
        let subpackets: Vec<Packet> = match self.decode_int(1) {
            0 => {
                let mut subpackets = Vec::new();
                let mut end = self.decode_int(15) as usize;
                end += self.position;
                while self.position < end {
                    subpackets.push(self.decode_packet());
                }
                subpackets
            }
            1 => (0..self.decode_int(11))
                .map(|_| self.decode_packet())
                .collect(),
            _ => panic!("Wrong value"),
        };
        let mut iter = subpackets.iter().map(|p| p.value);
        let value: u64 = match tid {
            0 => iter.sum(),
            1 => iter.fold(1, |a, n| a * n),
            2 => iter.min().unwrap(),
            3 => iter.max().unwrap(),
            5 => (iter.next().unwrap() > iter.next().unwrap()) as u64,
            6 => (iter.next().unwrap() < iter.next().unwrap()) as u64,
            7 => (iter.next().unwrap() == iter.next().unwrap()) as u64,
            _ => panic!(""),
        };

        (value, Some(subpackets))
    }

    fn decode_packet(self: &mut Self) -> Packet {
        let version = self.decode_int(3);
        let tid = self.decode_int(3);

        match tid {
            4 => Packet {
                version,
                tid,
                value: self.decode_value_packet(),
                subpackets: None,
            },
            _ => {
                let (value, subpackets) = self.decode_operator_packet(tid);
                Packet {
                    version,
                    tid,
                    value,
                    subpackets,
                }
            }
        }
    }
}

impl Packet {
    fn sum_version(self: &Self) -> u32 {
        self.version
            + self
                .subpackets
                .as_ref()
                .and_then(|v| Some(v.iter().map(|p| p.sum_version()).sum::<u32>()))
                .unwrap_or(0)
    }
}

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let codes = content.lines().map(|l| {
        l.chars()
            .map(|c| to_binary(c))
            .fold(String::new(), |mut a, c| {
                a.push_str(&c);
                a
            })
    });
    for line in codes {
        let mut stream = BitStream {
            data: &line,
            position: 0,
        };

        let packet = stream.decode_packet();

        println!("Version {}", packet.sum_version());
        println!("{}", packet.value);
    }
}
