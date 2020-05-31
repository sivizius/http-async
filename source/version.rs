use
{
  std::
  {
    fmt::
    {
      self,
      Display,
    },
  },
};

/// Protocol Version of Request and Response.
#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
pub enum      Version
{
  /// Version 0.9 (Default).
  HTTP_09,
  /// Version 1.0.
  HTTP_10,
  /// Version 1.1.
  HTTP_11,
  /// Version 2.0.
  HTTP_2,
  /// Version 3.0.
  HTTP_3,
}

impl          Display                   for Version
{
  fn fmt
  (
    &self,
    formatter:                          &mut fmt::Formatter<'_>
  )
  ->  fmt::Result
  {
    formatter
      .write_str
      (
        &format!
        (
          "HTTP/{}",
          match self
          {
            Version::HTTP_09            =>  "0.9",
            Version::HTTP_10            =>  "1.0",
            Version::HTTP_11            =>  "1.1",
            Version::HTTP_2             =>  "2.0",
            Version::HTTP_3             =>  "3.0",
          }
        )
      )
  }
}
