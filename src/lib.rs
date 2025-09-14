pub mod config;
pub mod processor;
pub mod svg_generator;
pub mod error;

pub use config::PixelatorConfig;
pub use processor::ImageProcessor;
pub use svg_generator::SvgGenerator;
pub use error::{PixelatorError, Result};

use std::path::Path;

pub struct Pixelator {
    config: PixelatorConfig,
}

impl Pixelator {
    pub fn new(config: PixelatorConfig) -> Self {
        Self { config }
    }

    pub fn process_image<P: AsRef<Path>>(&self, input_path: P) -> Result<String> {
        let image = image::open(input_path)?;
        
        let processor = ImageProcessor::new(&self.config);
        let sampled_pixels = processor.sample_image(&image)?;
        
        let svg_gen = SvgGenerator::new(&self.config);
        let svg_content = svg_gen.generate_svg(&sampled_pixels, image.width(), image.height())?;
        
        Ok(svg_content)
    }

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