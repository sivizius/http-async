use
{
  super::
  {
    Content,
    KeyValuePair,
    status::
    {
      Status,
    },
    version::
    {
      Version,
    },
  },
};

/// Hyper Text Transfer Protocol Response.
pub struct    Response
{
  /// Protocol Version.
  pub version:                          Version,
  /// Status Code.
  pub status:                           Status,
  /// List of Header Key Value Pairs.
  pub header:                           Vec < KeyValuePair  >,
  /// Content of Response.
  pub content:                          Vec < u8            >,
}

/// Constructor for a dummy `Response`.
pub fn  Response  ( )
->  Response
{
  Response
  {
    version:                            Version::HTTP_10,
    status:                             Status::Ok,
    header:                             Vec::new(),
    content:                            Vec::new(),
  }
}

impl          Response
{
  /// Consumes `Response` and converts it to `Vec<u8>`.
  pub fn        into_vector
  (
    mut self,
  )
  ->  Vec < u8  >
  {
    let mut buffer:         Vec < u8  > =   Vec::new();
    let mut header
    =   format!
        (
          "{} {}\r\n{}\r\n",
          self.version,
          self.status,
          {
            let mut result              =   String::new ( );
            for pair                    in  &self.header
            {
              result
                .push_str
                (
                  &format!
                  (
                    "{}: {}\r\n",
                    pair.key,
                    pair.value,
                  )
                );
            }
            result
          },
        ).into_bytes  ( );
    buffer.append           ( &mut header       );
    buffer.append           ( &mut self.content );
    buffer
  }

  /// Add a `header`-value to `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `key`                           – key of new header entry,
  /// * `value`                         – value of new header entry.
  pub fn        addHeader
  (
    mut self,
    key:                                String,
    value:                              String,
  )
  ->  Self
  {
    self
      .header
      .push
      (
        KeyValuePair
        {
          key,
          value,
        }
      );
    self
  }

  /// Set `version` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `version`                       – hyper text protocol version.
  pub fn        version
  (
    mut self,
    version:                            Version,
  )
  ->  Self
  {
    self.version                        =   version;
    self
  }

  /// Set `status` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `status`                       – hyper text protocol respone status code.
  pub fn        status
  (
    mut self,
    status:                             Status,
  )
  ->  Self
  {
    self.status                         =   status;
    self
  }

  /// Set `content` of `Response` in an OOP-Style.
  ///
  /// # Arguments
  /// * `contentType`                   – type of body,
  /// * `contentBody`                   – actual body.
  pub fn        content
  (
    mut self,
    this:                               Content,
  )
  ->  Self
  {
    let     length                      =   this.contentBody.len ( );
    self.content                        =   this.contentBody;
    self.status                         =   this.statusCode;
    self
      .addHeader
      (
        "Content-Type".to_owned(),
        this.contentType.to_owned(),
      )
      .addHeader
      (
        "Content-Length".to_owned(),
        format!
        (
          "{}",
          length,
        ),
      )
  }
}
