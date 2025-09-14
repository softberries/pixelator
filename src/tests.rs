#[cfg(test)]
mod tests {
    use crate::{PixelatorConfig, Pixelator, config::SampleMode};
    use crate::processor::{ImageProcessor, PixelData};
    use image::{DynamicImage, RgbaImage, Rgba};

    #[test]
    fn test_config_validation() {
        // Test invalid circle diameter
        assert!(PixelatorConfig::new(-1.0, 2.0).is_err());
        assert!(PixelatorConfig::new(0.0, 2.0).is_err());
        
        // Test invalid circle spacing
        assert!(PixelatorConfig::new(10.0, -1.0).is_err());
        
        // Test valid configuration
        assert!(PixelatorConfig::new(10.0, 2.0).is_ok());
        assert!(PixelatorConfig::new(10.0, 0.0).is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = PixelatorConfig::default();
        assert_eq!(config.circle_diameter, 10.0);
        assert_eq!(config.circle_spacing, 2.0);
        assert!(config.output_width_mm.is_none());
        assert!(config.output_height_mm.is_none());
        assert!(config.background_color.is_none());
        assert!(matches!(config.sample_mode, SampleMode::Grid));
    }

    #[test]
    fn test_config_builder_pattern() {
        let config = PixelatorConfig::new(15.0, 3.0)
            .unwrap()
            .with_output_dimensions(100.0, 150.0)
            .unwrap()
            .with_background_color("white".to_string())
            .with_sample_mode(SampleMode::Hexagonal);

        assert_eq!(config.circle_diameter, 15.0);
        assert_eq!(config.circle_spacing, 3.0);
        assert_eq!(config.output_width_mm, Some(100.0));
        assert_eq!(config.output_height_mm, Some(150.0));
        assert_eq!(config.background_color, Some("white".to_string()));
        assert!(matches!(config.sample_mode, SampleMode::Hexagonal));
    }

    #[test]
    fn test_output_dimensions_validation() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        
        // Test invalid dimensions
        assert!(config.clone().with_output_dimensions(-10.0, 100.0).is_err());
        assert!(config.clone().with_output_dimensions(100.0, -10.0).is_err());
        assert!(config.clone().with_output_dimensions(0.0, 100.0).is_err());
        assert!(config.clone().with_output_dimensions(100.0, 0.0).is_err());
        
        // Test valid dimensions
        assert!(config.with_output_dimensions(100.0, 150.0).is_ok());
    }

    #[test]
    fn test_total_spacing_calculation() {
        let config = PixelatorConfig::new(10.0, 5.0).unwrap();
        assert_eq!(config.get_total_spacing(), 15.0);
        
        let config = PixelatorConfig::new(20.0, 0.0).unwrap();
        assert_eq!(config.get_total_spacing(), 20.0);
    }

    #[test]
    fn test_pixel_data_creation() {
        let pixel = PixelData {
            x: 10.0,
            y: 20.0,
            color: Rgba([255, 128, 64, 255]),
        };
        
        assert_eq!(pixel.x, 10.0);
        assert_eq!(pixel.y, 20.0);
        assert_eq!(pixel.color[0], 255);
        assert_eq!(pixel.color[1], 128);
        assert_eq!(pixel.color[2], 64);
        assert_eq!(pixel.color[3], 255);
    }

    #[test]
    fn test_image_processor_creation() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let _processor = ImageProcessor::new(&config);
        // Test that processor is created successfully
        // Further testing would require actual image data
    }

    #[test]
    fn test_sample_mode_grid_vs_hexagonal() {
        // Create a small test image
        let img = RgbaImage::from_pixel(100, 100, Rgba([128, 128, 128, 255]));
        let dynamic_img = DynamicImage::ImageRgba8(img);
        
        // Test with Grid mode
        let config_grid = PixelatorConfig::new(10.0, 5.0)
            .unwrap()
            .with_sample_mode(SampleMode::Grid);
        let processor_grid = ImageProcessor::new(&config_grid);
        let pixels_grid = processor_grid.sample_image(&dynamic_img).unwrap();
        
        // Test with Hexagonal mode
        let config_hex = PixelatorConfig::new(10.0, 5.0)
            .unwrap()
            .with_sample_mode(SampleMode::Hexagonal);
        let processor_hex = ImageProcessor::new(&config_hex);
        let pixels_hex = processor_hex.sample_image(&dynamic_img).unwrap();
        
        // Both should produce pixels but potentially different counts
        assert!(!pixels_grid.is_empty());
        assert!(!pixels_hex.is_empty());
    }

    #[test]
    fn test_svg_generator_creation() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let _generator = crate::svg_generator::SvgGenerator::new(&config);
        // Test that generator is created successfully
    }

    #[test]
    fn test_svg_generation_with_empty_pixels() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let generator = crate::svg_generator::SvgGenerator::new(&config);
        
        let pixels: Vec<PixelData> = vec![];
        let svg = generator.generate_svg(&pixels, 100, 100).unwrap();
        
        // Should produce valid SVG even with no circles
        assert!(svg.contains("svg"));
        assert!(!svg.is_empty());
    }

    #[test]
    fn test_svg_generation_with_pixels() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let generator = crate::svg_generator::SvgGenerator::new(&config);
        
        let pixels = vec![
            PixelData {
                x: 10.0,
                y: 10.0,
                color: Rgba([255, 0, 0, 255]),
            },
            PixelData {
                x: 30.0,
                y: 30.0,
                color: Rgba([0, 255, 0, 255]),
            },
        ];
        
        let svg = generator.generate_svg(&pixels, 100, 100).unwrap();
        
        // Check that SVG contains circles
        assert!(svg.contains("<circle"));
        assert!(svg.contains("rgb(255,0,0)"));
        assert!(svg.contains("rgb(0,255,0)"));
    }

    #[test]
    fn test_pixelator_creation() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let _pixelator = Pixelator::new(config);
        // Test that pixelator is created successfully
    }

    #[test]
    fn test_color_caching_optimization() {
        let config = PixelatorConfig::new(10.0, 2.0).unwrap();
        let generator = crate::svg_generator::SvgGenerator::new(&config);
        
        // Create many pixels with the same color
        let mut pixels = Vec::new();
        for i in 0..100 {
            pixels.push(PixelData {
                x: (i * 10) as f32,
                y: 10.0,
                color: Rgba([128, 128, 128, 255]), // Same color for all
            });
        }
        
        let svg = generator.generate_svg(&pixels, 1000, 100).unwrap();
        
        // All circles should reference the same color
        assert!(svg.contains("rgb(128,128,128)"));
        assert_eq!(svg.matches("<circle").count(), 100);
    }

    #[test]
    fn test_hexagonal_constant() {
        use crate::processor::HEXAGONAL_ROW_HEIGHT_FACTOR;
        
        // Check that the constant is approximately sqrt(3)/2
        let expected = (3.0_f32).sqrt() / 2.0;
        assert!((HEXAGONAL_ROW_HEIGHT_FACTOR - expected).abs() < 0.001);
    }
}