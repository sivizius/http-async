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
  std::
  {
    fmt::
    {
      self,
      Display,
    },
  },
};

/// State of the Parser parsing a Path.
enum          PathState
{
  Start,
  Key,
  Value,
}

/// Path and Query to a Resource.
pub struct    Path
{
  /// Path to a Resource.
  pub path:                             String,
  /// List of Query Key Value Pairs.
  pub query:                            Vec < KeyValuePair  >,
}

impl          Path
{
  /// Try to parse path from from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut stream:                         &mut TcpStream,
    querySeperator:                     char,
  )
  ->  Option  < Path  >
  {
    let mut result                      =   None;
    let mut path                        =   "".to_owned ( );
    let mut key                         =   "".to_owned ( );
    let mut value                       =   "".to_owned ( );
    let mut query                       =   Vec::new    ( );
    let mut state                       =   PathState::Start;
    while let Some  ( char  )           =   Request::readChar ( &mut stream )
    {
      match char
      {
        ' '
        =>  {
              match state
              {
                | PathState::Value
                | PathState::Key
                =>  if !key.is_empty()
                    {
                      query
                        .push
                        (
                          KeyValuePair
                          {
                            key,
                            value,
                          }
                        );
                    },
                PathState::Start
                =>  {},
              }
              result
              =   Some
                  (
                    Path
                    {
                      path,
                      query,
                    }
                  );
              break;
            },
        '?'
        =>  match state
            {
              PathState::Start          =>  state = PathState::Key,
              PathState::Key            =>  key.push      ( char  as  char  ),
              PathState::Value          =>  value.push    ( char  as  char  ),
            },
        '='
        =>  match state
            {
              PathState::Start          =>  path.push     ( char  as  char  ),
              PathState::Key            =>  state = PathState::Value,
              PathState::Value          =>  value.push    ( char  as  char  ),
            },
        '\r' | '\n'
        =>  break,
        _
        =>  if  char  ==  querySeperator
            {
              match state
              {
                PathState::Start        =>  path.push     ( char  as  char  ),
                | PathState::Key
                | PathState::Value
                =>  {
                      state             =   PathState::Key;
                      if !key.is_empty()
                      {
                        query
                          .push
                          (
                            KeyValuePair
                            {
                              key:      key.clone(),
                              value:    value.clone(),
                            }
                          );
                      }
                      key               =   "".to_owned ( );
                      value             =   "".to_owned ( );
                    },
              }
            }
            else
            {
              match state
              {
                PathState::Start        =>  path.push     ( char  as  char  ),
                PathState::Key          =>  key.push      ( char  as  char  ),
                PathState::Value        =>  value.push    ( char  as  char  ),
              }
            },
      }
    }
    result
  }
}

impl          Display                   for Path
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
          "{}{}",
          &self
            .path,
          &if  self
                .query
                .is_empty ( )
              {
                format!
                (
                  "?{}",
                  self
                    .query
                    .iter()
                    .map
                    (
                      | entry |
                      format!
                      (
                        "{}={}",
                        entry.key,
                        entry.value,
                      )
                    )
                    .collect::< Vec < String  > >()
                    .join ( "&" ),
                )
              }
              else
              {
                "".to_owned()
              },
        )
      )
  }
}
