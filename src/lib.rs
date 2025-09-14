pub mod config;
pub mod processor;
pub mod svg_generator;
pub mod error;

#[cfg(test)]
mod tests;

pub use config::PixelatorConfig;
pub use processor::ImageProcessor;
pub use svg_generator::SvgGenerator;
pub use error::{PixelatorError, Result};

use std::path::Path;

/// Main structure for converting images to SVG circle art
/// 
/// # Examples
/// ```no_run
/// use pixelator::{Pixelator, PixelatorConfig};
/// 
/// let config = PixelatorConfig::new(10.0, 2.0).unwrap();
/// let pixelator = Pixelator::new(config);
/// pixelator.process_image_to_file("input.png", "output.svg").unwrap();
/// ```
pub struct Pixelator {
    config: PixelatorConfig,
}

impl Pixelator {
    /// Creates a new Pixelator instance with the given configuration
    pub fn new(config: PixelatorConfig) -> Self {
        Self { config }
    }

    /// Processes an image and returns the SVG content as a string
    /// 
    /// # Arguments
    /// * `input_path` - Path to the input image file
    /// 
    /// # Returns
    /// * `Result<String>` - The SVG content or an error
    pub fn process_image<P: AsRef<Path>>(&self, input_path: P) -> Result<String> {
        let image = image::open(input_path)?;
        
        let processor = ImageProcessor::new(&self.config);
        let sampled_pixels = processor.sample_image(&image)?;
        
        let svg_gen = SvgGenerator::new(&self.config);
        let svg_content = svg_gen.generate_svg(&sampled_pixels, image.width(), image.height())?;
        
        Ok(svg_content)
    }

    /// Processes an image and writes the SVG to a file
    /// 
    /// # Arguments
    /// * `input_path` - Path to the input image file
    /// * `output_path` - Path where the SVG file will be written
    /// 
    /// # Returns
    /// * `Result<()>` - Success or an error
    pub fn process_image_to_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q,
    ) -> Result<()> {
        let svg_content = self.process_image(input_path)?;
        std::fs::write(output_path, svg_content)?;
        Ok(())
    }
}