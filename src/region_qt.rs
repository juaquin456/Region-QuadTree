use std::rc::Rc;

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

use primitives::BoundingBox;
use crate::region_qt::Color::{Black, White};

use crate::region_qt::primitives::Point;
mod primitives;

#[derive(PartialEq)]
pub enum Color {
    Black,
    Gray,
    White,
}

fn get_color(img: &DynamicImage, coord: (u32, u32)) -> Color {
    let real_color = img.get_pixel(coord.0, coord.1);
    if real_color[0] == 0 {
        Black
    }
    else {
        White
    }
}

struct RegionNodeQt {
    data: Color,
    bounding: BoundingBox,
    children: [Option<Rc<RegionNodeQt>>; 4],
}

impl RegionNodeQt {
    fn calculate_color(&self, img: &DynamicImage) -> Color {
        let xl = self.bounding.min().x;
        let yl = self.bounding.min().y;

        let mut current_color = get_color(img, (xl, yl));

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
        let (width, height) = img.dimensions();

        self.root = Some(Rc::new(RegionNodeQt {
            data: Color::Black,
            bounding: BoundingBox::new(Point::from((0, 0)), Point::from((width, height))),
            children: [None, None, None, None],
        }));

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
