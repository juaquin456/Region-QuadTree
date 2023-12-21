pub mod region_qt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tree = region_qt::RegionQt::new();
        tree.build("img/1.png");
        tree.write("src/tests/1.bin");
        tree.plot();
        let dims0 = tree.dimensions();

        let tree = region_qt::RegionQt::from_file("src/tests/1.bin");
        tree.write("src/tests/2.bin");
        let dims1 = tree.dimensions();

        assert_eq!(dims0, dims1);
    }
}
