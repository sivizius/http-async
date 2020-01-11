use
{
  super::
  {
    KeyValuePair,
    method::
    {
      Method,
    },
    path::
    {
      Path,
    },
    version::
    {
      Version,
    },
  },
  async_std::
  {
    net::
    {
      TcpStream,
    },
    prelude::*,
    task,
  },
};

/// Hyper Text Transfer Protocol Request.
pub struct    Request
{
  /// Method to use to get the desired Resource.
  pub method:                           Method,
  /// Path to the desired Resource.
  pub path:                             String,
  /// When parsing a Request, how should I differentiate the Key Value Pairs of the Query?
  pub querySeperator:                   char,
  /// List of Query Key Value Pairs.
  pub query:                            Vec < KeyValuePair  >,
  /// Protocol Version.
  pub version:                          Version,
  /// List of Header Key Value Pairs.
  pub header:                           Vec < KeyValuePair  >,
  /// Content of Request.
  pub content:                          String,
}

/// Constructor for a dummy `Request`.
pub fn  Request ()
->  Request
{
  Request
  {
    method:                               Method::Dummy,
    path:                                 "".to_owned(),
    querySeperator:                       '&',
    query:                                Vec::new(),
    version:                              Version::Dummy,
    header:                               Vec::new(),
    content:                              "".to_owned(),
  }
}

impl          Request
{
  /// Read `char` from `stream` and compare it with `character`.
  ///
  /// # Arguments
  /// * `character`                     – `char` to compare with,
  /// * `stream`                        – Transmission Control Protocol Stream,
  /// * `inner`                         – inner value to return on success.
  pub fn        ifChar
  <
    Inner,
  >
  (
    character:                          char,
    stream:                             &mut TcpStream,
    inner:                              Inner,
  )
  ->  Option  < Inner >
  {
    if  let Some  ( this  )             =   Self::readChar  ( stream  )
    {
      if  character ==  this
      {
        Some  ( inner )
      }
      else
      {
        None
      }
    }
    else
    {
      None
    }
  }

  /// Try to read a single `char` from Transmission Control Protocol stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub fn        readChar
  (
    stream:                             &mut TcpStream,
  )
  ->  Option  < char  >
  {
    task::block_on
    (
      async
      {
        let mut buffer                  =   vec!  [ 0u8;  1 ];
        if  let Ok  ( length  )         =   stream.read  ( &mut buffer ).await
        {
          if  length  ==  1
          {
            Some  ( buffer  [ 0 ] as  char  )
          }
          else
          {
            None
          }
        }
        else
        {
          None
        }
      }
    )
  }

  /// Parse `TcpStream` as Hyper Text Transfer Protocol Request.
  ///
  /// # Arguments
  /// * `stream`                          – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut self,
    mut stream:                           &mut TcpStream,
  )
  ->  Result
      <
        Self,
        String,
      >
  {
    if  let Some  ( method  )             =   Method::parse   ( &mut stream                       ).await
    {
      self.method                         =   method;
      if let Some ( path )                =   Path::parse     ( &mut stream,  self.querySeperator ).await
      {
        println!
        (
          "{}?({:?})",
          path.path,
          path.query,
        );
        self.path                         =   path.path;
        self.query                        =   path.query;
        if let Some ( version )           =   Version::parse  ( &mut stream                       ).await
        {
          self.version                    =   version;
          //  ToDo: Parse Header and Content.
          Ok  ( self  )
        }
        else
        {
          Err ( "Could not parse version".to_owned  ( ) )
        }
      }
      else
      {
        Err   ( "Could not parse path".to_owned     ( ) )
      }
    }
    else
    {
      Err     ( "Could not parse method.".to_owned  ( ) )
    }
  }
}
