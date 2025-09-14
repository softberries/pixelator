use crate::config::{PixelatorConfig, SampleMode};
use crate::error::Result;
use image::{DynamicImage, Rgba};

#[derive(Debug, Clone)]
pub struct PixelData {
    pub x: f32,
    pub y: f32,
    pub color: Rgba<u8>,
}

pub struct ImageProcessor<'a> {
    config: &'a PixelatorConfig,
}

impl<'a> ImageProcessor<'a> {
    pub fn new(config: &'a PixelatorConfig) -> Self {
        Self { config }
    }
    
    pub fn sample_image(&self, image: &DynamicImage) -> Result<Vec<PixelData>> {
        let rgba_image = image.to_rgba8();
        let (img_width, img_height) = (rgba_image.width(), rgba_image.height());
        
        let total_spacing = self.config.get_total_spacing();
        
        let cols = ((img_width as f32) / total_spacing).floor() as usize;
        let rows = ((img_height as f32) / total_spacing).floor() as usize;
        
        let mut pixels = Vec::new();
        
        match self.config.sample_mode {
            SampleMode::Grid => {
                for row in 0..rows {
                    for col in 0..cols {
                        let x = col as f32 * total_spacing + self.config.circle_diameter / 2.0;
                        let y = row as f32 * total_spacing + self.config.circle_diameter / 2.0;
                        
                        let sample_x = (x as u32).min(img_width - 1);
                        let sample_y = (y as u32).min(img_height - 1);
                        
                        let color = self.sample_area(&rgba_image, sample_x, sample_y);
                        
                        pixels.push(PixelData { x, y, color });
                    }
                }
            }
            SampleMode::Hexagonal => {
                let row_height = total_spacing * 0.866;
                let hex_rows = ((img_height as f32) / row_height).floor() as usize;
                
                for row in 0..hex_rows {
                    let offset = if row % 2 == 0 { 0.0 } else { total_spacing / 2.0 };
                    let y = row as f32 * row_height + self.config.circle_diameter / 2.0;
                    
                    let mut col = 0;
                    loop {
                        let x = col as f32 * total_spacing + offset + self.config.circle_diameter / 2.0;
                        if x >= img_width as f32 {
                            break;
                        }
                        
                        let sample_x = (x as u32).min(img_width - 1);
                        let sample_y = (y as u32).min(img_height - 1);
                        
                        let color = self.sample_area(&rgba_image, sample_x, sample_y);
                        
                        pixels.push(PixelData { x, y, color });
                        col += 1;
                    }
                }
            }
        }
        
        Ok(pixels)
    }
    
    fn sample_area(&self, image: &image::RgbaImage, center_x: u32, center_y: u32) -> Rgba<u8> {
        let radius = (self.config.circle_diameter / 2.0) as u32;
        let (img_width, img_height) = (image.width(), image.height());
        
        let mut r_sum = 0u32;
        let mut g_sum = 0u32;
        let mut b_sum = 0u32;
        let mut a_sum = 0u32;
        let mut count = 0u32;
        
        let x_start = center_x.saturating_sub(radius);
        let x_end = (center_x + radius).min(img_width - 1);
        let y_start = center_y.saturating_sub(radius);
        let y_end = (center_y + radius).min(img_height - 1);
        
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                
                if (dx * dx + dy * dy) as f32 <= (radius * radius) as f32 {
                    let pixel = image.get_pixel(x, y);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                    a_sum += pixel[3] as u32;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            Rgba([
                (r_sum / count) as u8,
                (g_sum / count) as u8,
                (b_sum / count) as u8,
                (a_sum / count) as u8,
            ])
        } else {
            *image.get_pixel(center_x, center_y)
        }
    }
}