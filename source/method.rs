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

/// Method of this Hyper Text Transfer Protocol Request.
#[allow(dead_code)]
#[derive(Debug)]
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
