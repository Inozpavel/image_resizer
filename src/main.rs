mod cli;

use crate::cli::{Cli, Command, InputFilterType, ResizeSingleImage};
use anyhow::Context;
use clap::Parser;
use image::imageops::FilterType;
use image::{EncodableLayout, ImageFormat, ImageReader};
use std::io::Cursor;
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    debug!("Parsed: {:?}", cli);

    match cli.command {
        Command::ResizeSingleImage(input) => resize_single_image(input)
            .await
            .context("Failed to resize image")?,
    }

    Ok(())
}

async fn resize_single_image(input: ResizeSingleImage) -> Result<(), anyhow::Error> {
    let image_content = tokio::fs::read(&input.input_file_path)
        .await
        .context("Failed to read image file")?;

    let image = ImageReader::new(Cursor::new(image_content));
    let dynamic_image = image
        .with_guessed_format()
        .context("Failed to guess format")?
        .decode()
        .context("Failed to decode image")?;

    let filter_type = match input.filter_type {
        InputFilterType::Nearest => FilterType::Nearest,
        InputFilterType::Triangle => FilterType::Triangle,
        InputFilterType::CatmullRom => FilterType::CatmullRom,
        InputFilterType::Gaussian => FilterType::Gaussian,
        InputFilterType::Lanczos3 => FilterType::Lanczos3,
    };
    let resized = dynamic_image.resize(input.width, input.height, filter_type);

    let mut result = Cursor::new(vec![]);
    resized
        .write_to(&mut result, ImageFormat::Png)
        .context("failed to write result")?;

    let result_path = input.output_file_path.unwrap_or(input.input_file_path);

    tokio::fs::write(result_path, result.get_ref().as_bytes())
        .await
        .context("Failed to write result image")?;

    Ok(())
}
