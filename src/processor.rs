use crate::config::{PixelatorConfig, SampleMode};
use crate::error::Result;
use image::{DynamicImage, Rgba};
use rayon::prelude::*;

// Hexagonal grid constant: sqrt(3)/2 for row height calculation
pub const HEXAGONAL_ROW_HEIGHT_FACTOR: f32 = 0.866;

/// Data for a single sampled pixel/circle
#[derive(Debug, Clone)]
pub struct PixelData {
    pub x: f32,
    pub y: f32,
    pub color: Rgba<u8>,
    pub brightness: f32,  // Brightness value for halftone mode (0.0 to 1.0)
    pub dot_size: f32,     // Variable dot size for halftone mode
}

/// Processes images by sampling pixels at regular intervals
pub struct ImageProcessor<'a> {
    config: &'a PixelatorConfig,
}

impl<'a> ImageProcessor<'a> {
    /// Creates a new image processor with the given configuration
    pub fn new(config: &'a PixelatorConfig) -> Self {
        Self { config }
    }
    
    /// Samples the image according to the configured pattern and returns pixel data
    /// Uses parallel processing for improved performance on multi-core systems
    pub fn sample_image(&self, image: &DynamicImage) -> Result<Vec<PixelData>> {
        let rgba_image = std::sync::Arc::new(image.to_rgba8());
        let (img_width, img_height) = (rgba_image.width(), rgba_image.height());
        
        let total_spacing = self.config.get_total_spacing();
        
        let cols = ((img_width as f32) / total_spacing).floor() as usize;
        let rows = ((img_height as f32) / total_spacing).floor() as usize;
        
        let pixels = match self.config.sample_mode {
            SampleMode::Grid => {
                // Use parallel iterator for grid sampling
                let pixel_data: Vec<PixelData> = (0..rows)
                    .into_par_iter()
                    .flat_map(|row| {
                        let rgba_image = rgba_image.clone();
                        let total_spacing = total_spacing;
                        let circle_diameter = self.config.circle_diameter;
                        
                        (0..cols).into_par_iter().map(move |col| {
                            let x = col as f32 * total_spacing + circle_diameter / 2.0;
                            let y = row as f32 * total_spacing + circle_diameter / 2.0;
                            
                            let sample_x = (x as u32).min(img_width - 1);
                            let sample_y = (y as u32).min(img_height - 1);
                            
                            let color = Self::sample_area_static(&rgba_image, sample_x, sample_y, circle_diameter);
                            let brightness = Self::calculate_brightness(&color);
                            let dot_size = self.calculate_dot_size(brightness);
                            
                            PixelData { x, y, color, brightness, dot_size }
                        })
                    })
                    .collect();
                
                pixel_data
            }
            SampleMode::Hexagonal => {
                let row_height = total_spacing * HEXAGONAL_ROW_HEIGHT_FACTOR;
                let hex_rows = ((img_height as f32) / row_height).floor() as usize;
                
                // Use parallel iterator for hexagonal sampling
                let pixel_data: Vec<Vec<PixelData>> = (0..hex_rows)
                    .into_par_iter()
                    .map(|row| {
                        let rgba_image = rgba_image.clone();
                        let offset = if row % 2 == 0 { 0.0 } else { total_spacing / 2.0 };
                        let y = row as f32 * row_height + self.config.circle_diameter / 2.0;
                        
                        let mut row_pixels = Vec::new();
                        let mut col = 0;
                        loop {
                            let x = col as f32 * total_spacing + offset + self.config.circle_diameter / 2.0;
                            if x >= img_width as f32 {
                                break;
                            }
                            
                            let sample_x = (x as u32).min(img_width - 1);
                            let sample_y = (y as u32).min(img_height - 1);
                            
                            let color = Self::sample_area_static(&rgba_image, sample_x, sample_y, self.config.circle_diameter);
                            let brightness = Self::calculate_brightness(&color);
                            let dot_size = self.calculate_dot_size(brightness);
                            
                            row_pixels.push(PixelData { x, y, color, brightness, dot_size });
                            col += 1;
                        }
                        row_pixels
                    })
                    .collect();
                
                // Flatten the results
                pixel_data.into_iter().flatten().collect()
            }
        };
        
        Ok(pixels)
    }
    
    fn sample_area_static(image: &image::RgbaImage, center_x: u32, center_y: u32, circle_diameter: f32) -> Rgba<u8> {
        let radius = (circle_diameter / 2.0) as i32;
        let (img_width, img_height) = (image.width(), image.height());
        
        let mut r_sum = 0u32;
        let mut g_sum = 0u32;
        let mut b_sum = 0u32;
        let mut a_sum = 0u32;
        let mut count = 0u32;
        
        // Use integer bounds to avoid conversions in the loop
        let x_start = (center_x as i32).saturating_sub(radius).max(0) as u32;
        let x_end = ((center_x as i32) + radius).min(img_width as i32 - 1) as u32;
        let y_start = (center_y as i32).saturating_sub(radius).max(0) as u32;
        let y_end = ((center_y as i32) + radius).min(img_height as i32 - 1) as u32;
        
        let radius_squared = radius * radius;
        
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                
                // Use integer arithmetic for circle check
                if dx * dx + dy * dy <= radius_squared {
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
    
    /// Calculate brightness from an RGBA color (0.0 = black, 1.0 = white)
    pub fn calculate_brightness(color: &Rgba<u8>) -> f32 {
        // Use standard luminance formula (ITU-R BT.709)
        let r = color[0] as f32 / 255.0;
        let g = color[1] as f32 / 255.0;
        let b = color[2] as f32 / 255.0;
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    
    /// Calculate dot size based on brightness for halftone effect
    fn calculate_dot_size(&self, brightness: f32) -> f32 {
        use crate::config::{RenderMode, HalftoneStyle};
        
        match &self.config.render_mode {
            RenderMode::Color => self.config.circle_diameter,
            RenderMode::Halftone(style) => {
                // Invert brightness for black-on-white (darker = larger dots)
                // Keep normal for white-on-black (brighter = larger dots)
                let adjusted_brightness = match style {
                    HalftoneStyle::BlackOnWhite => 1.0 - brightness,
                    HalftoneStyle::WhiteOnBlack => brightness,
                };
                
                // Map brightness to dot size range
                self.config.min_dot_size + 
                    (self.config.max_dot_size - self.config.min_dot_size) * adjusted_brightness
            }
        }
    }
}