mod region_qt;
mod primitives;

pub fn add() {
    let img = ImageReader::open("src/Sample_User_Icon.png").unwrap().decode().unwrap();
    println!("Image dimensions: {:?}", img);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        add()
    }
}
