use crate::error::{PixelatorError, Result};

#[derive(Debug, Clone)]
pub struct PixelatorConfig {
    pub circle_diameter: f32,
    pub circle_spacing: f32,
    pub output_width_mm: Option<f32>,
    pub output_height_mm: Option<f32>,
    pub background_color: Option<String>,
    pub sample_mode: SampleMode,
}

#[derive(Debug, Clone)]
pub enum SampleMode {
    Grid,
    Hexagonal,
}

impl PixelatorConfig {
    pub fn new(circle_diameter: f32, circle_spacing: f32) -> Result<Self> {
        if circle_diameter <= 0.0 {
            return Err(PixelatorError::InvalidConfig(
                "Circle diameter must be positive".to_string(),
            ));
        }
        
        if circle_spacing < 0.0 {
            return Err(PixelatorError::InvalidConfig(
                "Circle spacing cannot be negative".to_string(),
            ));
        }
        
        Ok(Self {
            circle_diameter,
            circle_spacing,
            output_width_mm: None,
            output_height_mm: None,
            background_color: None,
            sample_mode: SampleMode::Grid,
        })
    }
    
    pub fn with_output_dimensions(mut self, width_mm: f32, height_mm: f32) -> Result<Self> {
        if width_mm <= 0.0 || height_mm <= 0.0 {
            return Err(PixelatorError::InvalidConfig(
                "Output dimensions must be positive".to_string(),
            ));
        }
        self.output_width_mm = Some(width_mm);
        self.output_height_mm = Some(height_mm);
        Ok(self)
    }
    
    pub fn with_background_color(mut self, color: String) -> Self {
        self.background_color = Some(color);
        self
    }
    
    pub fn with_sample_mode(mut self, mode: SampleMode) -> Self {
        self.sample_mode = mode;
        self
    }
    
    pub fn get_total_spacing(&self) -> f32 {
        self.circle_diameter + self.circle_spacing
    }
}