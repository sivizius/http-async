use
{
  super::
  {
    request::
    {
      Request,
    },
  },
  async_std::
  {
    net::
    {
      TcpStream,
    },
  },
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
pub enum      Version
{
  /// Placeholder.
  Dummy,
  /// Version 0.9.
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

impl          Version
{
  /// Try to parse Hyper Text Transfer Protocol Version from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut stream:                         &mut TcpStream,
  )
  ->  Option  < Version >
  {
    match Some  ( Version::Dummy  )
            .and_then ( | version | Request::ifChar ( 'H',  &mut stream,  version ) )
            .and_then ( | version | Request::ifChar ( 'T',  &mut stream,  version ) )
            .and_then ( | version | Request::ifChar ( 'T',  &mut stream,  version ) )
            .and_then ( | version | Request::ifChar ( 'P',  &mut stream,  version ) )
            .and_then ( | version | Request::ifChar ( '/',  &mut stream,  version ) )
    {
      Some  ( _ )
      =>  match Request::readChar ( &mut stream )
          {
            Some  ( '0' )
            =>  Some  ( Version::HTTP_09  )
                  .and_then ( | version | Request::ifChar ( '.',  &mut stream,  version ) )
                  .and_then ( | version | Request::ifChar ( '9',  &mut stream,  version ) ),
            Some  ( '1' )
            =>  match Request::readChar ( &mut stream )
                {
                  Some  ( '.' )
                  =>  match Request::readChar ( &mut stream )
                      {
                        Some  ( '0' )
                        =>  Some  ( Version::HTTP_10  ),
                        Some  ( '1' )
                        =>  Some  ( Version::HTTP_11  ),
                        _
                        =>  None,
                      },
                  _
                  =>  None,
                },
            Some  ( '2' )
            =>  Some  ( Version::HTTP_2   )
                  .and_then ( | version | Request::ifChar ( '.',  &mut stream,  version ) )
                  .and_then ( | version | Request::ifChar ( '0',  &mut stream,  version ) ),
            Some  ( '3' )
            =>  Some  ( Version::HTTP_3   )
                  .and_then ( | version | Request::ifChar ( '.',  &mut stream,  version ) )
                  .and_then ( | version | Request::ifChar ( '0',  &mut stream,  version ) ),
            _
            =>  None,
          }
      None
      =>  None,
    }
      .and_then ( | version | Request::ifChar ( '\r',  &mut stream,  version ) )
      .and_then ( | version | Request::ifChar ( '\n',  &mut stream,  version ) )
  }
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
            Version::Dummy              =>  "?.?",
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
