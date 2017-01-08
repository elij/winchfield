use nom::multispace;

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

named!(pub parse_field<&str, Option<&str> >,
       do_parse!(tag!("|") >> x: cap!(opt!(cap!(is_not!("|\r\n")))) >> (x)));
named!(pub parse_fields<&str, Vec<Option<&str> > >, many1!(parse_field));
named!(pub parse_segment<&str, (&str, Vec<Option<&str>>)>,
       do_parse!(
           name: is_not!("|") >>
           fields: parse_fields >>
           cap!(opt!(complete!(multispace))) >>
               (name, fields)
       ));
named!(pub parse_segments<&str, Vec<(&str, Vec<Option<&str>>)> >, many1!(parse_segment));
