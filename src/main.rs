use anyhow::Result;
use clap::{Parser, ValueEnum};
use pixelator::{Pixelator, PixelatorConfig, config::SampleMode};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SampleModeArg {
    Grid,
    Hexagonal,
    Hex,
}

impl From<SampleModeArg> for SampleMode {
    fn from(mode: SampleModeArg) -> Self {
        match mode {
            SampleModeArg::Grid => SampleMode::Grid,
            SampleModeArg::Hexagonal | SampleModeArg::Hex => SampleMode::Hexagonal,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input image file path")]
    input: PathBuf,

    #[arg(help = "Output SVG file path")]
    output: PathBuf,

    #[arg(short = 'd', long, default_value = "10.0", help = "Circle diameter in pixels")]
    circle_diameter: f32,

    #[arg(short = 's', long, default_value = "2.0", help = "Spacing between circles in pixels")]
    circle_spacing: f32,

    #[arg(short = 'w', long, help = "Output width in millimeters")]
    width_mm: Option<f32>,

    #[arg(short = 'h', long, help = "Output height in millimeters")]
    height_mm: Option<f32>,

    #[arg(short = 'b', long, help = "Background color (e.g., #FFFFFF or white)")]
    background: Option<String>,

    #[arg(short = 'm', long, default_value = "grid", value_enum, help = "Sampling mode")]
    mode: SampleModeArg,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", args.input);
    }

    let mut config = PixelatorConfig::new(args.circle_diameter, args.circle_spacing)?;

    if let (Some(w), Some(h)) = (args.width_mm, args.height_mm) {
        config = config.with_output_dimensions(w, h)?;
    }

    if let Some(bg) = args.background {
        config = config.with_background_color(bg);
    }

    config = config.with_sample_mode(args.mode.into());

    println!("Processing image: {:?}", args.input);
    println!("Configuration:");
    println!("  Circle diameter: {} pixels", args.circle_diameter);
    println!("  Circle spacing: {} pixels", args.circle_spacing);
    println!("  Sample mode: {:?}", args.mode);
    
    if let (Some(w), Some(h)) = (args.width_mm, args.height_mm) {
        println!("  Output dimensions: {}mm x {}mm", w, h);
    }

    let pixelator = Pixelator::new(config);
    
    pixelator.process_image_to_file(&args.input, &args.output)?;
    
    println!("Successfully generated SVG: {:?}", args.output);
    println!("Ready for printing!");

    Ok(())
}
