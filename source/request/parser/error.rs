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

/// Type of Failure while Request-Parsing.
pub enum      ParserError
{
  /// Body is too long.
  Body,
  /// Carriage Return expected.
  CarriageReturn,
  /// Cannot Parse Content Length.
  ContentLength,
  /// An Error occured while parsing a Header Key of the Request.
  HeaderKey,
  /// An Error occured while parsing a Header Value of the Request.
  HeaderValue,
  /// An Error occured while parsing a Cookie Key of the Request.
  CookieKey,
  /// An Error occured while parsing a Cookie Value of the Request.
  CookieValue,
  /// Line Feed expected.
  LineFeed,
  /// An Error occured while parsing the Method of the Request.
  Method,
  /// An Error occured while parsing the Path of the Request.
  Path,
  /// An Error occured while parsing the Query of the Request.
  Query,
  /// An Error occured while parsing the Version of the Request.
  Version,
}

impl          Display                   for ParserError
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
        match self
        {
          ParserError::Body             =>  "Body too long.",
          ParserError::CarriageReturn   =>  "Carriage Return expected.",
          ParserError::ContentLength    =>  "Cannot Parse Content Length.",
          ParserError::HeaderKey        =>  "An Error occured while parsing a Header Key of the Request.",
          ParserError::HeaderValue      =>  "An Error occured while parsing a Header Value of the Request.",
          ParserError::CookieKey        =>  "An Error occured while parsing a Cookie Key of the Request.",
          ParserError::CookieValue      =>  "An Error occured while parsing a Cookie Value of the Request.",
          ParserError::LineFeed         =>  "Line Feed expected.",
          ParserError::Method           =>  "An Error occured while parsing the Method of the Request.",
          ParserError::Path             =>  "An Error occured while parsing the Path of the Request.",
          ParserError::Query            =>  "An Error occured while parsing the Query of the Request.",
          ParserError::Version          =>  "An Error occured while parsing the Version of the Request.",
        }
      )
  }
}
