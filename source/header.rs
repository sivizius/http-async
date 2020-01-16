use
{
  super::
  {
    KeyValuePair,
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
};

/// A struct-stub as a Nmespace.
pub struct    Header                    ();

impl          Header
{
  /// Try to parse Header from from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut stream:                         &mut TcpStream,
  )
  ->  Result
      <
        Option  < KeyValuePair  >,
        (),
      >
  {
    let mut result                      =   Err ( ( ) );
    let mut key                         =   "".to_owned ( );
    let mut value                       =   "".to_owned ( );
    let mut state                       =   HeaderState::Key;
    while let Some  ( char  )           =   Request::readChar ( &mut stream )
    {
      match state
      {
        HeaderState::Key
        =>  match char
            {
              ':'
              =>  if  let Some  ( ' ' )
                          =   Request::readChar ( &mut stream )
                  {
                    state               =   HeaderState::Value;
                  }
                  else
                  {
                    break;
                  },
              '\r'
              =>  {
                    if  let Some  ( '\n'  )
                          =   Request::readChar ( &mut stream )
                    {
                      result            =   Ok  ( None  );
                    }
                    break;
                  },
              _
              =>  key.push    ( char  ),
            },
        HeaderState::Value
        =>  match char
            {
              '\r'
              =>  {
                    if  let Some  ( '\n'  )
                          =   Request::readChar ( &mut stream )
                    {
                      result
                      =   Ok
                          (
                            Some
                            (
                              KeyValuePair
                              {
                                key,
                                value,
                              }
                            )
                          );
                    }
                    break;
                  },
              _
              =>  value.push  ( char  ),
            },
      }
    }
    result
  }
}

/// State of the Parser parsing a Header.
enum          HeaderState
{
  Key,
  Value,
}
