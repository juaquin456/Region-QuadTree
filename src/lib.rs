mod region_qt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tree = region_qt::RegionQt::new();
        tree.build("src/Untitled.png");
    }
}
