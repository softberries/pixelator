use crate::config::PixelatorConfig;
use crate::error::Result;
use crate::processor::PixelData;
use svg::Document;
use svg::node::element::Circle;

pub struct SvgGenerator<'a> {
    config: &'a PixelatorConfig,
}

impl<'a> SvgGenerator<'a> {
    pub fn new(config: &'a PixelatorConfig) -> Self {
        Self { config }
    }
    
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
        
        if let Some(ref bg_color) = self.config.background_color {
            document = document.set("style", format!("background-color: {}", bg_color));
        }
        
        let radius = self.config.circle_diameter / 2.0;
        
        for pixel in pixels {
            let color = format!(
                "rgb({}, {}, {})",
                pixel.color[0],
                pixel.color[1],
                pixel.color[2]
            );
            
            let opacity = pixel.color[3] as f32 / 255.0;
            
            let circle = Circle::new()
                .set("cx", pixel.x)
                .set("cy", pixel.y)
                .set("r", radius)
                .set("fill", color)
                .set("fill-opacity", opacity);
            
            document = document.add(circle);
        }
        
        Ok(document.to_string())
    }
}