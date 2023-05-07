
use std::rc::Rc;


use image::{ImageBuffer, Rgba};
use resvg::{usvg::{self, NodeExt}, tiny_skia};

use crate::delaunay::Triangle;


#[derive(Debug)]
pub enum DrawingError {
    Encoding,  
    Rendering, 
}

pub struct Drawing {
    svg_tree: usvg::Tree,
}


impl Drawing {

    pub fn render(&self) -> Result<tiny_skia::Pixmap, DrawingError> {
        let pixmap_size: usvg::ScreenSize = self.svg_tree.size.to_screen_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .ok_or(DrawingError::Rendering)?;

        let rendering = resvg::render(
            &self.svg_tree,
            resvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        );

        if rendering.is_some() {
            Ok(pixmap)
        } else {
            Err(DrawingError::Rendering)
        }

    
    }

    pub fn save_as_png(&self, filepath: &str) -> Result<(), DrawingError>{
        let pixmap = self.render()?;
        pixmap.save_png(filepath).map_err(|_|  DrawingError::Encoding)
    }
    
}


pub trait Drawable {
  fn draw(&self, source_image: ImageBuffer<Rgba<u8>, Vec<u8>>, triangles: Vec<Triangle>) -> Option<Drawing>; 
    
}

pub struct  Drawer {
    pub only_wireframe: bool, 
    pub stroke_width: f64, 
    pub stroke_color: Option<usvg::Color>,
    pub with_background: bool, 
    pub background_color: Option<usvg::Color>
}

impl Default for Drawer {
    fn default() -> Self {
        Self { 
            only_wireframe: true, 
            stroke_color: None, 
            stroke_width: 0.1, 
            with_background: false, 
            background_color: None 
        }
    }
}

impl Drawable for Drawer {
    fn draw(&self, source_image: ImageBuffer<Rgba<u8>, Vec<u8>>, triangles: Vec<Triangle>) -> Option<Drawing> {

        if triangles.len() == 0 {
            return None;
        }


        let (width, height) = source_image.dimensions();
        let size = usvg::Size::new(width as f64, height as f64)?;

        let tree = usvg::Tree {
            size,
            view_box: usvg::ViewBox {
                rect: size.to_rect(0.0, 0.0),
                aspect: usvg::AspectRatio::default(),
            },
            root: usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default())),
        };

        // drawing background
        if self.with_background {
            let fill_background = usvg::Fill::from_paint(
                usvg::Paint::Color(self.background_color.unwrap_or(usvg::Color::white()))
            );

            let background =  usvg::Rect::new(0.0, 0.0, width as f64, height as f64)?;

            let node_background = usvg::NodeKind::Path(usvg::Path {
                fill: Some(fill_background),
                data: Rc::new(usvg::PathData::from_rect(background)),
                ..usvg::Path::default()
            });
            tree.root.append_kind(node_background);
        }


        // drawing triangle
        for triangle in triangles {

            let center = triangle.center();
            let pixel = source_image.get_pixel(center.x as u32, center.y as u32);
            let color =  usvg::Color::new_rgb(pixel[0], pixel[1], pixel[2]);
            let vertex = triangle.vertex();

            let stroke_triangle = if self.stroke_width > 0.0 {
                let stroke_color = self.stroke_color.unwrap_or(usvg::Color::black());
                Some(usvg::Stroke {
                    paint: usvg::Paint::Color(stroke_color),
                    width: usvg::NonZeroPositiveF64::new(self.stroke_width).unwrap(),
                    linejoin: usvg::LineJoin::Round,
                    ..Default::default()
                })
            } else {
                None
            };

            let fill_triangle =  if !self.only_wireframe  {
                Some(
                    usvg::Fill::from_paint(usvg::Paint::Color(color))
                )
            } else {
                None
            };

            let mut path_triangle = usvg::PathData::new();
            path_triangle.push_move_to(vertex[0].x, vertex[0].y);
            path_triangle.push_line_to(vertex[1].x, vertex[1].y);
            path_triangle.push_line_to(vertex[2].x, vertex[2].y);
            path_triangle.push_line_to(vertex[0].x, vertex[0].y);
            path_triangle.push_close_path();

            let node_triangle = usvg::NodeKind::Path(usvg::Path {
                fill: fill_triangle,
                stroke: stroke_triangle,
                data: Rc::new(path_triangle),
                ..usvg::Path::default()
            });
    
            tree.root.append_kind(node_triangle);

        }
    
        Some(Drawing { svg_tree: tree})
    }
}


