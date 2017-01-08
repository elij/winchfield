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

    #[test]
    fn should_see_even_without_trailing_cr() {
        let i_msh = "MSH|^~\\&|EPICADT|DH|LABADT|DH|201301011226||ADT^A01|HL7MSG00001|P|2.3|\rEVN|A01|201301011223||";
        let x = parse_segments(i_msh);
        let y = x.unwrap().1;
        println!("{:?}", y);
        assert_eq!(y[1].1[1].unwrap(), "201301011223");
    }

}
