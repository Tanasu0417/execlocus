use std::{fs, path::Path};

const ICON_SIZE: u32 = 32;

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_development_icon(path: &Path) -> std::io::Result<()> {
    let width = ICON_SIZE;
    let height = ICON_SIZE;
    let xor_size = width * height * 4;
    let mask_stride = width.div_ceil(32) * 4;
    let mask_size = mask_stride * height;
    let image_size = 40 + xor_size + mask_size;
    let mut bytes = Vec::with_capacity((22 + image_size) as usize);

    push_u16(&mut bytes, 0);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 1);
    bytes.extend_from_slice(&[
        u8::try_from(width).expect("development icon width must fit in the ICO directory"),
        u8::try_from(height).expect("development icon height must fit in the ICO directory"),
        0,
        0,
    ]);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 32);
    push_u32(&mut bytes, image_size);
    push_u32(&mut bytes, 22);

    push_u32(&mut bytes, 40);
    push_i32(
        &mut bytes,
        i32::try_from(width).expect("development icon width must fit in a bitmap header"),
    );
    push_i32(
        &mut bytes,
        i32::try_from(height * 2).expect("development icon height must fit in a bitmap header"),
    );
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 32);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, xor_size);
    bytes.extend_from_slice(&[0; 16]);

    let mut opaque = vec![false; (width * height) as usize];
    for y in (0..height).rev() {
        for x in 0..width {
            let dx = f64::from(x) - 14.0;
            let dy = f64::from(y) - 16.0;
            let radius = dx.mul_add(dx, dy * dy).sqrt();
            let ring = (10.0..=13.5).contains(&radius);
            let marker = ((dx - 13.0).mul_add(dx - 13.0, dy * dy)).sqrt() <= 2.2;
            let is_opaque = ring || marker;
            opaque[(y * width + x) as usize] = is_opaque;
            if is_opaque {
                bytes.extend_from_slice(&[196, 216, 103, 255]);
            } else {
                bytes.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }

    for y in (0..height).rev() {
        let mut row = vec![0_u8; mask_stride as usize];
        for x in 0..width {
            if !opaque[(y * width + x) as usize] {
                row[(x / 8) as usize] |= 1 << (7 - (x % 8));
            }
        }
        bytes.extend_from_slice(&row);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)
}

fn main() {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be available to the build script");
    let icon_path = Path::new(&manifest_dir).join("icons/icon.ico");
    write_development_icon(&icon_path).expect("the development app icon could not be generated");
    tauri_build::build();
}
