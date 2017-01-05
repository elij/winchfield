#[macro_use]
extern crate nom;
extern crate winchfield;

#[cfg(test)]
mod tests {
    use winchfield::parse_segments;

    #[test]
    fn test() {
        let i_msh = include_str!("../assets/test_a01.dat");
        let x = parse_segments(i_msh);
        let y = x.unwrap().1;
        println!("{:?}", y);
        assert_eq!(y[0].1[1].unwrap(), "EPICADT");
    }
}
