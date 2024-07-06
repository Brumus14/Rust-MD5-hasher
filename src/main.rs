use std::env;
use std::fs::read;

macro_rules! round1 {
    ($a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $i: literal, $x: ident, $t: ident) => {
        $a = $b.wrapping_add(
            $a.wrapping_add(F($b, $c, $d))
                .wrapping_add($x[$k])
                .wrapping_add($t[$i as usize])
                .rotate_left($s),
        );
    };
}

macro_rules! round2 {
    ($a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $i: literal, $x: ident, $t: ident) => {
        $a = $b.wrapping_add(
            $a.wrapping_add(G($b, $c, $d))
                .wrapping_add($x[$k])
                .wrapping_add($t[$i as usize])
                .rotate_left($s),
        );
    };
}

macro_rules! round3 {
    ($a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $i: literal, $x: ident, $t: ident) => {
        $a = $b.wrapping_add(
            $a.wrapping_add(H($b, $c, $d))
                .wrapping_add($x[$k])
                .wrapping_add($t[$i as usize])
                .rotate_left($s),
        );
    };
}

macro_rules! round4 {
    ($a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $i: literal, $x: ident, $t: ident) => {
        $a = $b.wrapping_add(
            $a.wrapping_add(I($b, $c, $d))
                .wrapping_add($x[$k])
                .wrapping_add($t[$i as usize])
                .rotate_left($s),
        );
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "-h" => {
            print!(
                r#"Usage: md5 [OPTION]
-t,         hash passed text
-f          hash a file at path"#
            );
        }
        "-f" => {
            let bytes = read(&args[2]).unwrap();
            md5(bytes);
        }
        "-t" => {
            let bytes = args[2].as_bytes().to_vec();
            md5(bytes);
        }
        _ => println!("md5: Invalid usage"),
    }
}

fn md5(bits: Vec<u8>) {
    // Clone the inputted bits
    let mut bits = bits.clone();

    // Pad the bits to a multiple of 512 bits
    pad_bits(&mut bits);

    // Initialise buffers
    let mut A: u32 = 0x67452301;
    let mut B: u32 = 0xefcdab89;
    let mut C: u32 = 0x98badcfe;
    let mut D: u32 = 0x10325476;

    // Initialise table t with 64 zeros
    let mut t: Vec<u32> = vec![0; 64];

    // Generate table t values
    for i in 1..=64 {
        t[i - 1] = (4294967296. * (i as f64).sin().abs()).trunc() as u32;
    }

    // Initialise x buffer to store 512 bit blocks
    let mut x: Vec<u32> = vec![0; 16];

    // For each 512 bit blocks
    for i in 0..(bits_length(&bits) / 512) {
        // Copy the bit block to buffer x
        for j in 0..16 {
            x[j] = u32::from_le_bytes([
                bits[i * 64 + j * 4],
                bits[i * 64 + j * 4 + 1],
                bits[i * 64 + j * 4 + 2],
                bits[i * 64 + j * 4 + 3],
            ]);
        }

        let mut a = A;
        let mut b = B;
        let mut c = C;
        let mut d = D;

        // Round 1
        round1!(a, b, c, d, 0, 7, 0, x, t);
        round1!(d, a, b, c, 1, 12, 1, x, t);
        round1!(c, d, a, b, 2, 17, 2, x, t);
        round1!(b, c, d, a, 3, 22, 3, x, t);

        round1!(a, b, c, d, 4, 7, 4, x, t);
        round1!(d, a, b, c, 5, 12, 5, x, t);
        round1!(c, d, a, b, 6, 17, 6, x, t);
        round1!(b, c, d, a, 7, 22, 7, x, t);

        round1!(a, b, c, d, 8, 7, 8, x, t);
        round1!(d, a, b, c, 9, 12, 9, x, t);
        round1!(c, d, a, b, 10, 17, 10, x, t);
        round1!(b, c, d, a, 11, 22, 11, x, t);

        round1!(a, b, c, d, 12, 7, 12, x, t);
        round1!(d, a, b, c, 13, 12, 13, x, t);
        round1!(c, d, a, b, 14, 17, 14, x, t);
        round1!(b, c, d, a, 15, 22, 15, x, t);

        // Round 2
        round2!(a, b, c, d, 1, 5, 16, x, t);
        round2!(d, a, b, c, 6, 9, 17, x, t);
        round2!(c, d, a, b, 11, 14, 18, x, t);
        round2!(b, c, d, a, 0, 20, 19, x, t);

        round2!(a, b, c, d, 5, 5, 20, x, t);
        round2!(d, a, b, c, 10, 9, 21, x, t);
        round2!(c, d, a, b, 15, 14, 22, x, t);
        round2!(b, c, d, a, 4, 20, 23, x, t);

        round2!(a, b, c, d, 9, 5, 24, x, t);
        round2!(d, a, b, c, 14, 9, 25, x, t);
        round2!(c, d, a, b, 3, 14, 26, x, t);
        round2!(b, c, d, a, 8, 20, 27, x, t);

        round2!(a, b, c, d, 13, 5, 28, x, t);
        round2!(d, a, b, c, 2, 9, 29, x, t);
        round2!(c, d, a, b, 7, 14, 30, x, t);
        round2!(b, c, d, a, 12, 20, 31, x, t);

        // Round 3
        round3!(a, b, c, d, 5, 4, 32, x, t);
        round3!(d, a, b, c, 8, 11, 33, x, t);
        round3!(c, d, a, b, 11, 16, 34, x, t);
        round3!(b, c, d, a, 14, 23, 35, x, t);

        round3!(a, b, c, d, 1, 4, 36, x, t);
        round3!(d, a, b, c, 4, 11, 37, x, t);
        round3!(c, d, a, b, 7, 16, 38, x, t);
        round3!(b, c, d, a, 10, 23, 39, x, t);

        round3!(a, b, c, d, 13, 4, 40, x, t);
        round3!(d, a, b, c, 0, 11, 41, x, t);
        round3!(c, d, a, b, 3, 16, 42, x, t);
        round3!(b, c, d, a, 6, 23, 43, x, t);

        round3!(a, b, c, d, 9, 4, 44, x, t);
        round3!(d, a, b, c, 12, 11, 45, x, t);
        round3!(c, d, a, b, 15, 16, 46, x, t);
        round3!(b, c, d, a, 2, 23, 47, x, t);

        // Round 4
        round4!(a, b, c, d, 0, 6, 48, x, t);
        round4!(d, a, b, c, 7, 10, 49, x, t);
        round4!(c, d, a, b, 14, 15, 50, x, t);
        round4!(b, c, d, a, 5, 21, 51, x, t);

        round4!(a, b, c, d, 12, 6, 52, x, t);
        round4!(d, a, b, c, 3, 10, 53, x, t);
        round4!(c, d, a, b, 10, 15, 54, x, t);
        round4!(b, c, d, a, 1, 21, 55, x, t);

        round4!(a, b, c, d, 8, 6, 56, x, t);
        round4!(d, a, b, c, 15, 10, 57, x, t);
        round4!(c, d, a, b, 6, 15, 58, x, t);
        round4!(b, c, d, a, 13, 21, 59, x, t);

        round4!(a, b, c, d, 4, 6, 60, x, t);
        round4!(d, a, b, c, 11, 10, 61, x, t);
        round4!(c, d, a, b, 2, 15, 62, x, t);
        round4!(b, c, d, a, 9, 21, 63, x, t);

        A = A.wrapping_add(a);
        B = B.wrapping_add(b);
        C = C.wrapping_add(c);
        D = D.wrapping_add(d);
    }

    // Output computed hash in hexadecimal
    println!(
        "{:08x}{:08x}{:08x}{:08x}",
        A.swap_bytes(),
        B.swap_bytes(),
        C.swap_bytes(),
        D.swap_bytes()
    );
}

fn pad_bits(bits: &mut Vec<u8>) {
    // Store the original bit length
    let original_length = bits_length(&bits);

    // Push bits 10000000
    bits.push(128);

    // Push 0 bits until 64 bits below multiple of 512
    while bits_length(&bits) % 512 != 448 {
        bits.push(0);
    }

    // Append length in bits
    bits.append(&mut original_length.to_le_bytes().to_vec());
}

// Calculate the number of bits in a byte vector
fn bits_length(bits: &Vec<u8>) -> usize {
    bits.len() * 8
}

// Define auxiliary functions
fn F(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn G(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn H(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn I(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
