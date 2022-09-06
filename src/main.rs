use image::GenericImageView;

const PALETTE: [u8; 64] = [
    b'$', b'@', b'B', b'%', b'8', b'&', b'W', b'M', b'#', b'o', b'a', b'h', b'k', b'b', b'd',
    b'p', b'q', b'w', b'm', b'O', b'0', b'Q', b'L', b'C', b'J', b'U', b'Y', b'X', b'z', 
    b'v', b'u', b'n', b'x', b'r', b'j', b'f', b't', b'/', b'|', b'(', b')', b'1', b'{',
    b'}', b'[', b']', b'?', b'-', b'+', b'~', b'<', b'>', b'i', b'!', b'l', b'I', b';', b':',
    b'"', b'^', b'`', b'\'', b'.', b' ',
];

fn main() {
    let painting = paint("sad-frog.webp", 8);
    println!("{painting}");
}

fn paint(path: &str, scale_down: usize) -> String {
    let original = image::open(path).unwrap();
    let (width, height) = original.dimensions();
    let (width, height) = (width as usize, height as usize);
    let pixel_count = width / (scale_down * 2usize) * height / scale_down;
    let mut painting = Vec::with_capacity(pixel_count);
    for y in (0..height).filter(|i| i % (scale_down * 2) == 0) {
        for x in (0..width).filter(|i| i % scale_down == 0) {
            let pixel = original.get_pixel(x as u32, y as u32).0;
            painting.push(compute_pixel(pixel));
        }
        painting.push(b'\n');
    }
    // SAFETY: We know this string is comprised only of the characters in PALETTE, and '\n'.
    unsafe { String::from_utf8_unchecked(painting) }
}

fn compute_pixel(pixel: [u8; 4]) -> u8 {
    let [r, g, b, a] = pixel;
    // Max intensity: 255
    let intensity = r / 4 + g / 4 + b / 4 + a / 4;
    // Squash the range from 0..255 to 0..63
    let index = intensity / 4;
    PALETTE[index as usize]
}
