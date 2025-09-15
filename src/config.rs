use crate::error::{PixelatorError, Result};

/// Configuration for the Pixelator image processor
#[derive(Debug, Clone)]
pub struct PixelatorConfig {
    pub circle_diameter: f32,
    pub circle_spacing: f32,
    pub output_width_mm: Option<f32>,
    pub output_height_mm: Option<f32>,
    pub background_color: Option<String>,
    pub sample_mode: SampleMode,
    pub render_mode: RenderMode,
    pub max_dot_size: f32,  // Maximum dot size for halftone mode
    pub min_dot_size: f32,  // Minimum dot size for halftone mode
}

impl Default for PixelatorConfig {
    fn default() -> Self {
        Self {
            circle_diameter: 10.0,
            circle_spacing: 2.0,
            output_width_mm: None,
            output_height_mm: None,
            background_color: None,
            sample_mode: SampleMode::Grid,
            render_mode: RenderMode::Color,
            max_dot_size: 10.0,
            min_dot_size: 1.0,
        }
    }
}

/// Sampling mode for pixel extraction
#[derive(Debug, Clone)]
pub enum SampleMode {
    /// Regular grid pattern
    Grid,
    /// Hexagonal/honeycomb pattern
    Hexagonal,
}

/// Rendering style for the output
#[derive(Debug, Clone)]
pub enum RenderMode {
    /// Full color circles (original mode)
    Color,
    /// Halftone effect with variable dot sizes
    Halftone(HalftoneStyle),
}

/// Halftone rendering style options
#[derive(Debug, Clone)]
pub enum HalftoneStyle {
    /// Black dots on white background
    BlackOnWhite,
    /// White dots on black background
    WhiteOnBlack,
}

impl PixelatorConfig {
    /// Creates a new configuration with the specified circle dimensions
    /// 
    /// # Arguments
    /// * `circle_diameter` - Diameter of each circle in pixels (must be positive)
    /// * `circle_spacing` - Space between circles in pixels (must be non-negative)
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
            render_mode: RenderMode::Color,
            max_dot_size: circle_diameter,
            min_dot_size: circle_diameter * 0.1,
        })
    }
    
    /// Sets the output dimensions in millimeters for printing
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
    
    /// Sets the background color of the SVG
    pub fn with_background_color(mut self, color: String) -> Self {
        self.background_color = Some(color);
        self
    }
    
    /// Sets the sampling mode (Grid or Hexagonal)
    pub fn with_sample_mode(mut self, mode: SampleMode) -> Self {
        self.sample_mode = mode;
        self
    }
    
    /// Returns the total spacing between circle centers
    pub fn get_total_spacing(&self) -> f32 {
        self.circle_diameter + self.circle_spacing
    }
    
    /// Sets the rendering mode
    pub fn with_render_mode(mut self, mode: RenderMode) -> Self {
        self.render_mode = mode;
        self
    }
    
    /// Sets the halftone dot size range
    pub fn with_halftone_range(mut self, min_size: f32, max_size: f32) -> Result<Self> {
        if min_size <= 0.0 || max_size <= 0.0 {
            return Err(PixelatorError::InvalidConfig(
                "Dot sizes must be positive".to_string(),
            ));
        }
        if min_size > max_size {
            return Err(PixelatorError::InvalidConfig(
                "Minimum dot size must be less than maximum".to_string(),
            ));
        }
        self.min_dot_size = min_size;
        self.max_dot_size = max_size;
        Ok(self)
    }
}