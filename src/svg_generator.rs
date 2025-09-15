use crate::config::PixelatorConfig;
use crate::error::Result;
use crate::processor::PixelData;
use std::collections::HashMap;
use svg::Document;
use svg::node::element::Circle;

/// Generates SVG output from sampled pixel data
pub struct SvgGenerator<'a> {
    config: &'a PixelatorConfig,
}

impl<'a> SvgGenerator<'a> {
    /// Creates a new SVG generator with the given configuration
    pub fn new(config: &'a PixelatorConfig) -> Self {
        Self { config }
    }
    
    /// Generates an SVG document from pixel data
    /// Uses color caching to optimize performance for images with limited palettes
    /// Supports both color and halftone rendering modes
    /// 
    /// # Arguments
    /// * `pixels` - The sampled pixel data
    /// * `original_width` - Original image width in pixels
    /// * `original_height` - Original image height in pixels
    pub fn generate_svg(
        &self,
        pixels: &[PixelData],
        original_width: u32,
        original_height: u32,
    ) -> Result<String> {
        let (svg_width, svg_height) = if let (Some(w), Some(h)) = 
            (self.config.output_width_mm, self.config.output_height_mm) {
            (w, h)
        } else {
            (original_width as f32, original_height as f32)
        };
        
        let mut document = Document::new()
            .set("width", format!("{}mm", svg_width))
            .set("height", format!("{}mm", svg_height))
            .set("viewBox", (0, 0, original_width, original_height))
            .set("xmlns", "http://www.w3.org/2000/svg")
            .set("xmlns:xlink", "http://www.w3.org/1999/xlink");
        
        // Set background based on render mode
        use crate::config::{RenderMode, HalftoneStyle};
        let background = match &self.config.render_mode {
            RenderMode::Color => self.config.background_color.clone(),
            RenderMode::Halftone(style) => Some(match style {
                HalftoneStyle::BlackOnWhite => "white".to_string(),
                HalftoneStyle::WhiteOnBlack => "black".to_string(),
            }),
        };
        
        if let Some(ref bg_color) = background {
            document = document.set("style", format!("background-color: {}", bg_color));
        }
        
        match &self.config.render_mode {
            RenderMode::Color => {
                // Original color rendering
                let radius = self.config.circle_diameter / 2.0;
                
                // Cache color strings to avoid repeated allocations
                let mut color_cache: HashMap<(u8, u8, u8), String> = HashMap::new();
                
                for pixel in pixels {
                    let color_key = (pixel.color[0], pixel.color[1], pixel.color[2]);
                    
                    // Get or create the color string
                    let color = color_cache.entry(color_key)
                        .or_insert_with(|| {
                            format!("rgb({},{},{})", color_key.0, color_key.1, color_key.2)
                        });
                    
                    let opacity = pixel.color[3] as f32 / 255.0;
                    
                    let circle = Circle::new()
                        .set("cx", pixel.x)
                        .set("cy", pixel.y)
                        .set("r", radius)
                        .set("fill", color.as_str())
                        .set("fill-opacity", opacity);
                    
                    document = document.add(circle);
                }
            }
            RenderMode::Halftone(style) => {
                // Halftone rendering with variable dot sizes
                let dot_color = match style {
                    HalftoneStyle::BlackOnWhite => "black",
                    HalftoneStyle::WhiteOnBlack => "white",
                };
                
                for pixel in pixels {
                    // Skip very small dots (essentially white/transparent areas)
                    if pixel.dot_size < 0.5 {
                        continue;
                    }
                    
                    let radius = pixel.dot_size / 2.0;
                    
                    let circle = Circle::new()
                        .set("cx", pixel.x)
                        .set("cy", pixel.y)
                        .set("r", radius)
                        .set("fill", dot_color);
                    
                    document = document.add(circle);
                }
            }
        }
        
        Ok(document.to_string())
    }
}