mod region_qt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tree = region_qt::RegionQt::new();
        tree.build("src/img/Untitled.png");
        tree.write("src/test/1.p");

        let tree = region_qt::RegionQt::from_file("src/test/1.p");
        tree.write("src/test/2.p");
    }
}
