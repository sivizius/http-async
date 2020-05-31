use
{
  super::
  {
    KeyValuePair,
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

/// Path and Query to a Resource.
pub struct    Path
{
  /// Path to a Resource.
  pub path:                             String,
  /// List of Query Key Value Pairs.
  pub query:                            Vec < KeyValuePair  >,
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
                String::new ( )
              },
        )
      )
  }
}
