use std::rc::Rc;

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Rgba};

use primitives::BoundingBox;

use crate::region_qt::primitives::Point;
mod primitives;

#[derive(PartialEq)]
pub enum Color {
    Gray,
    Data(Rgba<u8>),
}

fn get_color(img: &DynamicImage, coord: (u32, u32)) -> Color {
    Color::Data(img.get_pixel(coord.0, coord.1))
}

struct RegionNodeQt {
    data: Color,
    bounding: BoundingBox,
    children: [Option<Rc<RegionNodeQt>>; 4],
}

impl RegionNodeQt {
    fn new(min: Point, max: Point) -> Self {
        RegionNodeQt {
            data: Color::Gray,
            bounding: BoundingBox::new(min, max),
            children: [None, None, None, None],
        }
    }

    fn is_leaf(&self) -> bool {
        for i in 0..4 {
            if !self.children[i].is_none() {
                return false;
            }
        }
        true
    }

    fn calculate_color(&self, img: &DynamicImage) -> Color {
        let xl = self.bounding.min().x;
        let yl = self.bounding.min().y;

        let current_color = get_color(img, (xl, yl));

        for x in xl..self.bounding.max().x {
            for y in self.bounding.min().y..self.bounding.max().y {
                let next_color = get_color(img, (x, y));
                if next_color != current_color {
                    return Color::Gray;
                }
            }
        }
        current_color
    }

    fn update(&mut self, img: &DynamicImage) {
        let color = self.calculate_color(img);
        match color {
            Color::Gray => {
                if self.is_leaf() {
                    self.initialize_children();
                    for i in 0..4 {
                        self.children[i].unwrap().update(img);
                    }
                }
            }
            _ => { self.data = color; }
        }
    }
}

pub struct RegionQt {
    root: Option<Rc<RegionNodeQt>>,
}

impl RegionQt {
    pub fn new() -> Self {
        RegionQt { root: None }
    }

    pub fn build(&mut self, path: &str) {
        let img = ImageReader::open(path)
            .expect("Can't open the file")
            .decode()
            .unwrap();
        let dim = img.dimensions();

        self.root = Some(Rc::new(RegionNodeQt::new(Point::from((0, 0)), Point::from(dim))));

        Rc::get_mut(self.root.as_mut().unwrap()).unwrap().update(&img);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let img = ImageReader::open("src/Untitled.png")
            .expect("Can't open the file")
            .decode()
            .unwrap();
        get_color(&img, (0, 9));
    }
}
