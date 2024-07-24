use std::fs::File;
use std::io::{BufWriter, Write};
use crate::framebuffer::Framebuffer;

const BMP_HEADER_SIZE: usize = 54;

pub trait WriteBmp {
    fn write_bmp(&self, filename: &str) -> std::io::Result<()>;
}

impl WriteBmp for Framebuffer {
    fn write_bmp(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        let padding = (4 - (self.width * 3) % 4) % 4;
        let row_size = self.width * 3 + padding;
        let data_size = row_size * self.height;
        let file_size = BMP_HEADER_SIZE + data_size;

        // BMP Header
        writer.write_all(b"BM")?;
        writer.write_all(&(file_size as u32).to_le_bytes())?;
        writer.write_all(&[0u8; 4])?;
        writer.write_all(&(BMP_HEADER_SIZE as u32).to_le_bytes())?;

        // DIB Header
        writer.write_all(&40u32.to_le_bytes())?;
        writer.write_all(&(self.width as i32).to_le_bytes())?;
        writer.write_all(&(-(self.height as i32)).to_le_bytes())?;
        writer.write_all(&1u16.to_le_bytes())?;
        writer.write_all(&24u16.to_le_bytes())?;
        writer.write_all(&[0u8; 24])?;

        // Pixel data
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.buffer[y * self.width + x];
                let r = (pixel >> 16) as u8;
                let g = (pixel >> 8) as u8;
                let b = pixel as u8;
                writer.write_all(&[b, g, r])?;
            }
            writer.write_all(&[0u8; 3][..padding])?;
        }

        Ok(())
    }
}