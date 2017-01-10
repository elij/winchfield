use nom::{IResult,multispace};

/// Captures macro result to aid type inference (alt, opt etc.)
#[macro_export]
macro_rules! cap(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
      {
          let rs: ::nom::IResult<_, _> =
              $submac!($i, $($args)* );
          rs
      }
  );
  ($i:expr,$f:expr) => (
    cap!($i, call!($f));
  );
);

pub fn parse_field( i: &str ) -> IResult<&str, Option<&str>, u32> {
    do_parse!(i, tag!("|") >> x: cap!(opt!(cap!(is_not!("|\r")))) >> (x))
}

pub fn parse_fields( i: &str ) -> IResult<&str, Vec<Option<&str> >, u32> {
    many1!(i, parse_field)
}

pub fn parse_segment( i: &str ) -> IResult<&str, (&str, Vec<Option<&str>>), u32> {
    do_parse!(i,
           name: take!(3) >>
           fields: parse_fields >>
           cap!(opt!(complete!(multispace))) >>
               (name, fields)
    )
}

named!(pub parse_segments<&str, Vec<(&str, Vec<Option<&str>>)> >,
       do_parse!(
           msh: do_parse!(                                                                                                        
               name: tag!("MSH") >>
                   sep: take!(1) >>
                   com: take!(1) >>
                   rep: take!(1) >>
                   esc: take!(1) >>
                   sub: take!(1) >>
//               fields: parse_fields(is_not!(sep), is_not!(com), is_not!(rep), is_not!(esc), is_not!(sub)) >>
               fields: parse_fields >>
               multispace >>
                   (name, fields, (sep, com, rep, esc, sub))
           ) >>
           t: many1!(parse_segment) >>
               (
                   {let mut x = t;
                    x.insert(0, (msh.0, msh.1));
                    x}
               )
       )
);
