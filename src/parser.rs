use nom::multispace;

#[macro_export]
macro_rules! opt_alias(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      let rs: ::nom::IResult<_, _> =
      match $submac!($i, $($args)*) {
        ::nom::IResult::Done(i,o)     => ::nom::IResult::Done(i, ::std::option::Option::Some(o)),
        ::nom::IResult::Error(_)      => ::nom::IResult::Done($i, ::std::option::Option::None),
        ::nom::IResult::Incomplete(i) => ::nom::IResult::Incomplete(i)
      };
      rs
    }
  );
  ($i:expr, $f:expr) => (
    opt!($i, call!($f));
  );
);

named!(pub parse_field<&str, Option<&str> >,
       do_parse!(tag!("|") >> x: opt_alias!(take_until_either!("|\r\n")) >> (x)));
named!(pub parse_fields<&str, Vec<Option<&str> > >, many1!(parse_field));
named!(pub parse_segment<&str, (&str, Vec<Option<&str>>)>,
       do_parse!(
           name: is_not!("|") >>
           fields: parse_fields >>
           opt_alias!(multispace) >>
               (name, fields)
       ));
named!(pub parse_segments<&str, Vec<(&str, Vec<Option<&str>>)> >, many1!(parse_segment));
