
use clap::Parser;
use resvg::usvg;
use image::io::Reader as ImageReader;
use anyhow::{anyhow, Result, Context};
use colored::Colorize;
use spinners::{Spinners, Spinner};


use crate::drawer::{Drawable, Drawer}; 

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// blur filter radius
    #[arg(long = "bf", default_value_t = 1)]
    blur_factor: usize,

    /// sobel factor radius
    #[arg(long = "sf", default_value_t = 6)]
    sobel_factor: usize,

    /// sobel filter threshold
    #[arg(long = "pt", default_value_t = 10)]
    points_threshold: i32,
    
    /// maximum number of points in the generated image
    #[arg(long = "mp", default_value_t = 2500)]
    max_points: usize,

    /// control the number of point in the generated image 
    #[arg(long = "pr", default_value_t = 0.075)]
    point_rate: f64,
    
    /// convert image to grayscale 
    #[arg(long = "gr", default_value_t = false)]
    grayscale: bool,
    
    /// source image
    #[arg(long="in")]
    input: String,
    
    /// destination image
    #[arg(long="out")]
    output: String,
    
    /// do not fill triangle in generated image, only stroke 
    #[arg(long = "ow", default_value_t = false)]
    only_wireframe: bool,
    
    /// stroke width in the generated image
    #[arg(long = "sw", default_value_t = 0.1)]
    stroke_witdh: f64,

     /// whenever the generated image should have a background
     #[arg(long = "wb")]
     with_background: bool,
    
    /// background color in the generated image in hex format 
    #[arg(long="bc", value_parser = color_from_hex)]
    background_color: Option<usvg::Color>,

    /// stroke color in the generated image in hex format 
    #[arg(long = "sc", value_parser = color_from_hex)]
    stroke_color: Option<usvg::Color>,
}

fn color_from_hex(hex: &str) -> Result<usvg::Color, String> {
    let parsing_error = "invalid hex color !";
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| parsing_error)?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| parsing_error)?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| parsing_error)?;

    Ok(usvg::Color { red: r, green: g, blue: b})
}

fn create_triangulation(args : &Arguments) -> super::Triangulation {

    super::Triangulation {
        blur_factor: args.blur_factor, 
        sobel_factor: args.sobel_factor, 
        points_threshold: args.points_threshold, 
        point_rate: args.point_rate, 
        max_points: args.max_points, 
        grayscale: args.grayscale,
        ..Default::default()
        
    }
} 

fn create_drawer(args : &Arguments) -> Drawer {
    Drawer {
        only_wireframe: args.only_wireframe, 
        stroke_color: args.stroke_color,
        background_color: args.background_color, 
        ..Default::default()
    }
}

fn format_error(message: &str) -> String {
    format!("{} {}", "\u{2718}".red().to_owned(), message.to_owned())
}

fn format_success(message: &str) -> String {
    format!("{} {}", "\u{2714}".green().to_owned(), message.to_owned())
}

pub fn execute() -> Result<()>{
    let args = Arguments::parse(); 

    let input_image = ImageReader::open(&args.input)
        .with_context(|| format_error("could not open input image"))?
        .decode()?;

    let triangulation = create_triangulation(&args);
    let drawer = create_drawer(&args);

    let mut sp = Spinner::new(Spinners::Dots, format_success("start generating delaunay image ...."));
    let (triangles, source_image) = triangulation.generate_triangle(input_image);
    
    if triangles.is_empty() {
        sp.stop_with_newline();
        return Err(anyhow!(format_error("could not generate delaunay triangles")))
    }

    let result_image =  drawer.draw(source_image, triangles)
        .ok_or_else(|| {
            sp.stop_with_newline();
            anyhow!(format_error("error occure during image generation. please retry")) 
        })?;

    sp.stop_with_newline();

    result_image.save_as_png(&args.output)
        .map_err(|_| anyhow!(format_error("could not save output image")))?;

    println!("{}", format_success("done (delaunay image is saved)"));
    return Ok(())
}
