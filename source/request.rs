use
{
  super::
  {
    KeyValuePair,
    header::
    {
      Header,
    },
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
  std::
  {
    str::
    {
      FromStr,
    },
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
  pub content:                          Vec < u8            >,
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
    content:                              Vec::new(),
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
  /// * `stream`                         – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut self,
    mut stream:                          &mut TcpStream,
  )
  ->  Result
      <
        Self,
        String,
      >
  {
    if  let Some  ( method  )           =   Method::parse   ( &mut stream                       ).await
    {
      self.method                       =   method;
      if let Some ( path )              =   Path::parse     ( &mut stream,  self.querySeperator ).await
      {
        self.path                       =   path.path;
        self.query                      =   path.query;
        if let Some ( version )         =   Version::parse  ( &mut stream                       ).await
        {
          self.version                  =   version;
          let mut error                 =   Some  ( "Could not parse List of headers".to_owned  ( ) );
          let mut length                =   0;
          while let Ok  ( entry )       =   Header::parse   ( &mut stream                       ).await
          {
            if  length  < 32
            {
              if  let Some  ( entry )   =   entry
              {
                self
                  .header
                  .push ( entry );
              }
              else
              {
                error                   =   None;
                break;
              }
            }
            else
            {
              error                     =   Some  ( "Too many Header Entries, Slow Lorris Attack?".to_owned  ( ) );
              break;
            }
            length                      +=  1
          }
          if        let Some  ( message )
                    =   error
          {
            Err ( message )
          }
          else  if  let Some  ( entry )
                    =   self
                          .header
                          .iter()
                          .find
                          (
                            | entry |
                            entry.key ==  "Content-Length"
                          )
          {
            if  let Ok  ( length  )     =   usize::from_str ( &entry.value  )
            {
              if  length  < 0x0008_0000
              {
                let mut buffer          =   vec!  [ 0u8;  length  ];
                if  let Ok  ( size  )   =   stream.read ( &mut buffer ).await
                {
                  if  let Ok  ( text  )
                      =   String::from_utf8 ( buffer.clone  ( ) )
                  {
                    println!
                    (
                      "\n<{}> {}?({:?})\n{}Length: {}\nContent:\n»{}« ({:?})",
                      self.method,
                      self.path,
                      self.query,
                      self
                        .header
                        .iter()
                        .fold
                        (
                          "".to_owned(),
                          | mut text, entry |
                          {
                            text
                              .push_str
                              (
                                &format!
                                (
                                  "→{} = {}\n",
                                  entry.key,
                                  entry.value,
                                )
                              );
                            text
                          }
                        ),
                        length,
                        text,
                        buffer,
                    );
                  }
                  self
                    .content            =   buffer;
                  if  size  ==  length
                  {
                    Ok  ( self  )
                  }
                  else
                  {
                    Err
                    (
                      format!
                      (
                        "Expected Content with {} bytes, but received only {}",
                        length,
                        size,
                      )
                    )
                  }
                }
                else
                {
                  Err
                  (
                    format!
                    (
                      "Could not read {} bytes of content",
                      length,
                    )
                  )
                }
              }
              else
              {
                Err
                (
                  format!
                  (
                    "To avoid Denial of Service by Remote Allocation of a Buffer, the Upper Limit is below {} bytes",
                    length,
                  )
                )
              }
            }
            else
            {
              Err ( "Could not parse length of content".to_owned  ( ) )
            }
          }
          else
          {
            println!
            (
              "\n<{}> {}?({:?})\n{}No Content.",
              self.method,
              self.path,
              self.query,
              self
                .header
                .iter()
                .fold
                (
                  "".to_owned(),
                  | mut text, entry |
                  {
                    text
                      .push_str
                      (
                        &format!
                        (
                          "→{} = {}\n",
                          entry.key,
                          entry.value,
                        )
                      );
                    text
                  }
                ),
            );
            Ok    ( self  )
          }
        }
        else
        {
          Err       ( "Could not parse version".to_owned            ( ) )
        }
      }
      else
      {
        Err         ( "Could not parse path".to_owned               ( ) )
      }
    }
    else
    {
      Err           ( "Could not parse method.".to_owned            ( ) )
    }
  }
}
