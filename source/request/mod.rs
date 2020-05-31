/// Request-Parser.
pub mod parser;

use
{
  super::
  {
    KeyValuePair,
    method::
    {
      Method,
    },
    version::
    {
      Version,
    },
  },
};

/// Hyper Text Transfer Protocol Request.
#[derive(Debug)]
pub struct    Request
{
  /// Method to use to get the desired Resource.
  pub method:                           Method,
  /// Path to the desired Resource.
  pub path:                             String,
  /// List of Query Key Value Pairs.
  pub query:                            Vec < KeyValuePair  >,
  /// Protocol Version.
  pub version:                          Version,
  /// List of Unknown Header Key Value Pairs.
  pub header:                           RequestHeader,
  /// List of Cookie Key Value Pairs.
  pub cookies:                          Vec < KeyValuePair  >,
  /// Content of Request.
  pub content:                          Vec < u8            >,
}

/// Constructor for a dummy `Request`.
pub fn  Request ( )
->  Request
{
  Request
  {
    method:                             Method::Dummy,
    path:                               String::new   ( ),
    query:                              Vec::new      ( ),
    version:                            Version::HTTP_09,
    header:                             RequestHeader ( ),
    cookies:                            Vec::new      ( ),
    content:                            Vec::new      ( ),
  }
}

/// Header of a Hyper Text Transfer Protocol Request.
#[derive(Debug)]
pub struct    RequestHeader
{
  contentLength:                        Option  < usize         >,
  host:                                 Option  < String        >,
  userAgent:                            Option  < String        >,
  other:                                Vec     < KeyValuePair  >,
}

/// Constructor for a dummy `RequestHeader`.
pub fn        RequestHeader ( )
->  RequestHeader
{
  RequestHeader
  {
    contentLength:                      None,
    host:                               None,
    userAgent:                          None,
    other:                              Vec::new      ( ),
  }
}

impl          RequestHeader
{
  /// Push an arbitary key-value-pair to the header.
  ///
  /// # Arguments
  /// * `pair`                          –   key-value-pair to push.
  pub fn        push
  (
    &mut  self,
    pair:                               KeyValuePair,
  )
  {
    self.other.push ( pair  );
  }

  /// Calculate length of Request Body.
  pub fn        bodyLength
  (
    &self,
  )
  ->  usize
  {
    if  let Some  ( bytes )
        =   self.contentLength
    {
      bytes
    }
    else
    {
      0
    }
  }

  /// Try to set a numerical header field.
  ///
  /// # Arguments
  /// * `field`                         –   header field to set,
  /// * `value`                         –   numeric value.
  pub fn        setNumber
  (
    &mut self,
    field:                              RequestHeaderField,
    value:                              usize,
  )
  ->  Result
      <
        ( ),
        RequestHeaderError,
      >
  {
    match field
    {
      RequestHeaderField::ContentLength
      =>  if  self.contentLength
              .is_none  ( )
          {
            self.contentLength          =   Some  ( value );
            Ok  ( ( ) )
          }
          else
          {
            Err ( RequestHeaderError::AlreadySet  )
          },
      _
      =>  Err   ( RequestHeaderError::NotANumber  ),
    }
  }

  /// Try to set a numerical header field.
  ///
  /// # Arguments
  /// * `field`                         –   header field to set,
  /// * `value`                         –   numeric value.
  pub fn        setString
  (
    &mut self,
    field:                              RequestHeaderField,
    value:                              String,
  )
  ->  Result
      <
        ( ),
        RequestHeaderError,
      >
  {
    match field
    {
      RequestHeaderField::Host
      =>  if  self.host
              .is_none  ( )
          {
            self.host                   =   Some  ( value );
            Ok  ( ( ) )
          }
          else
          {
            Err ( RequestHeaderError::AlreadySet  )
          },
      RequestHeaderField::UserAgent
      =>  if  self.userAgent
              .is_none  ( )
          {
            self.userAgent              =   Some  ( value );
            Ok  ( ( ) )
          }
          else
          {
            Err ( RequestHeaderError::AlreadySet  )
          },
      _
      =>  Err   ( RequestHeaderError::NotANumber  ),
    }
  }
}

/// Failures that might occur when setting header fields.
#[derive(Debug)]
pub enum      RequestHeaderError
{
  /// Header field was already set and cannot be set twice.
  AlreadySet,
  /// Header field is of type number, but another type was provided.
  NotANumber,
}

/// Request Header Fields.
#[derive(Clone,Copy,Debug)]
pub enum      RequestHeaderField
{
  /// Set the Content Length field.
  ContentLength,
  /// Set the Host field.
  Host,
  /// Set the User Agent field.
  UserAgent,
  /// Placeholder.
  Dummy,
}
