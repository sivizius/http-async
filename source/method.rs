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

/// Method of this Hyper Text Transfer Protocol Request.
#[allow(dead_code)]
pub enum      Method
{
  /// Placeholder.
  Dummy,
  /// Establish a Tunnel to the Server identified by the Target Resource.
  Connect,
  /// Delete the Specified Resource.
  Delete,
  /// Request a Representation of the Specified Resource. Requests using this Method shall only retrieve data.
  Get,
  /// Asks for a Response identical to that of a GET Request, but without the Response Content.
  Head,
  /// Describe the Communication Options for the Target Resource.
  Options,
  /// Apply Partial Modifications to a Resource.
  Patch,
  /// Submit an Entity to the Specified Resource, often causing a Change in State or Side Effects on the Server.
  Post,
  /// Replace all Current Representations of the Target Resource with the Request Payload.
  Put,
  /// Perform a Message Loop-Back Test along the Path to the Target Resource.
  Trace,
}

impl          Method
{
  /// Try to parse Hyper Text Transfer Protocol Request Method from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut stream:                         &mut TcpStream,
  )
  ->  Option  < Method  >
  {
    match Request::readChar ( &mut stream )
    {
      Some  ( 'C' )
      =>  Some  ( Method::Connect )
            .and_then ( | method  | Request::ifChar ( 'O',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'N',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'N',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'C',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) ),
      Some  ( 'D' )
      =>  Some  ( Method::Delete  )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'L',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) ),
      Some  ( 'G' )
      =>  Some  ( Method::Get     )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) ),
      Some  ( 'H' )
      =>  Some  ( Method::Post    )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'A',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'D',  &mut stream,  method  ) ),
      Some  ( 'O' )
      =>  Some  ( Method::Options )
            .and_then ( | method  | Request::ifChar ( 'P',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'I',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'O',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'N',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'S',  &mut stream,  method  ) ),
      Some  ( 'P' )
      =>  match Request::readChar ( &mut stream )
          {
            Some  ( 'A' )
            =>  Some  ( Method::Patch   )
                  .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) )
                  .and_then ( | method  | Request::ifChar ( 'C',  &mut stream,  method  ) )
                  .and_then ( | method  | Request::ifChar ( 'H',  &mut stream,  method  ) ),
            Some  ( 'O' )
            =>  Some  ( Method::Post    )
                  .and_then ( | method  | Request::ifChar ( 'S',  &mut stream,  method  ) )
                  .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) ),
            Some  ( 'U' )
            =>  Some  ( Method::Put     )
                  .and_then ( | method  | Request::ifChar ( 'T',  &mut stream,  method  ) ),
            _
            =>  None,
          },
      Some  ( 'T' )
      =>  Some  ( Method::Trace   )
            .and_then ( | method  | Request::ifChar ( 'R',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'A',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'C',  &mut stream,  method  ) )
            .and_then ( | method  | Request::ifChar ( 'E',  &mut stream,  method  ) ),
      _
      =>  None,
    }
      .and_then ( | method  | Request::ifChar ( ' ',  &mut stream,  method  ) )
  }
}

impl          Display                   for Method
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
          Method::Connect               =>  "CONNECT",
          Method::Delete                =>  "DELETE",
          Method::Dummy                 =>  "????",
          Method::Get                   =>  "GET",
          Method::Head                  =>  "HEAD",
          Method::Options               =>  "OPTIONS",
          Method::Patch                 =>  "PATCH",
          Method::Post                  =>  "POST",
          Method::Put                   =>  "PUT",
          Method::Trace                 =>  "TRACE",
        }
      )
  }
}
