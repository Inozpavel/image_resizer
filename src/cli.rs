use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    ResizeSingleImage(ResizeSingleImage),
}

#[derive(Parser, Debug)]
pub struct ResizeSingleImage {
    /// Input image file path
    #[arg(long, short = 'i')]
    pub input_file_path: String,

    /// New image width
    #[arg(long, short = 'w')]
    pub width: u32,

    /// New image height
    #[arg(long, short = 'H')]
    pub height: u32,

    /// Output image file path. Default to input file path
    #[arg(long, short = 'r')]
    pub output_file_path: Option<String>,

    /// Filter type (https://docs.rs/image/latest/image/imageops/enum.FilterType.html)
    #[arg(long, short = 'f', default_value = "gaussian")]
    pub filter_type: InputFilterType,
}

#[derive(Debug, ValueEnum, Copy, Clone)]
pub enum InputFilterType {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Triangle,

    /// Cubic Filter
    CatmullRom,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}
