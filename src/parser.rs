use nom::multispace;

/// Captures macro result to aid type inference (alt, opt etc.)
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

macro_rules! parse_field(
    ($i:expr, $sa:tt) => (
        do_parse!($i, tag!($sa) >> x: cap!(opt!(cap!(
            is_not!(&("\r".to_string() + $sa))
        ))) >> (x))
    );
);

macro_rules! parse_fields(
    ($i:expr, $sa:tt) => (
        many1!($i, parse_field!($sa))
    );
);

macro_rules! parse_segment(
    ($i:expr, $sa:tt) => (
        do_parse!($i,
            name: take!(3) >>
                fields: parse_fields!($sa) >>
                cap!(opt!(complete!(multispace))) >>
                (name, fields)
        )
    );
);

named!(pub parse_segments<&str, Vec<(&str, Vec<Option<&str>>)> >,
       do_parse!(
           msh: do_parse!(                                                                                                        
               name: tag!("MSH") >>
                   sep: take!(1) >>
                   com: take!(1) >>
                   rep: take!(1) >>
                   esc: take!(1) >>
                   sub: take!(1) >>
               fields: parse_fields!(sep) >>//, is_not!(com), is_not!(rep), is_not!(esc), is_not!(sub)) >>
               multispace >>
                   (name, fields, (sep, com, rep, esc, sub))
           ) >>
           t: many1!(parse_segment!(((msh.2).0))) >>
               (
                   {let mut x = t;
                    x.insert(0, (msh.0, msh.1));
                    x}
               )
       )
);
