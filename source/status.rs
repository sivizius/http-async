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

/// Status Code of Response.
#[allow(dead_code)]
pub enum      Status
{
  /// 100 – Continue, see [RFC7231, Section 6.2.1].
  Continue,
  /// 101 – Switching Protocols, see [RFC7231, Section 6.2.2].
  SwitchingProtocols,
  /// 102 – Processing, see [RFC2518].
  Processing,
  /// 103 – Early Hints, see [RFC8297].
  EarlyHints,
  /// 200 – OK, see [RFC7231, Section 6.3.1].
  Ok,
  /// 201 – Created, see [RFC7231, Section 6.3.2].
  Created,
  /// 202 – Accepted, see [RFC7231, Section 6.3.3].
  Accepted,
  /// 203 – Non-Authoritative Information, see [RFC7231, Section 6.3.4].
  NonAuthoritativeInformation,
  /// 204 – No Content, see [RFC7231, Section 6.3.5].
  NoContent,
  /// 205 – Reset Content, see [RFC7231, Section 6.3.6].
  ResetContent,
  /// 206 – Partial Content, see [RFC7233, Section 4.1].
  PartialContent,
  /// 207 – Multi-Status, see [RFC4918].
  MultiStatus,
  /// 208 – Already Reported, see [RFC5842].
  AlreadyReported,
  /// 226 – IM Used, see [RFC3229].
  IMUsed,
  /// 300 – Multiple Choices, see [RFC7231, Section 6.4.1].
  MultipleChoices,
  /// 301 – Moved Permanently, see [RFC7231, Section 6.4.2].
  MovedPermanently,
  /// 302 – Found, see [RFC7231, Section 6.4.3].
  Found,
  /// 303 – See Other, see [RFC7231, Section 6.4.4].
  SeeOther,
  /// 304 – Not Modified, see [RFC7232, Section 4.1].
  NotModified,
  /// 305 – Use Proxy, see [RFC7231, Section 6.4.5].
  UseProxy,
  /// 307 – Temporary Redirect, see [RFC7231, Section 6.4.7].
  TemporaryRedirect,
  /// 308 – Permanent Redirect, see [RFC7538].
  PermanentRedirect,
  /// 400 – Bad Request, see [RFC7231, Section 6.5.1].
  BadRequest,
  /// 401 – Unauthorized, see [RFC7235, Section 3.1].
  Unauthorized,
  /// 402 – Payment Required, see [RFC7231, Section 6.5.2].
  PaymentRequired,
  /// 403 – Forbidden, see [RFC7231, Section 6.5.3].
  Forbidden,
  /// 404 – Not Found, see [RFC7231, Section 6.5.4].
  NotFound,
  /// 405 – Method Not Allowed, see [RFC7231, Section 6.5.5].
  MethodNotAllowed,
  /// 406 – Not Acceptable, see [RFC7231, Section 6.5.6].
  NotAcceptable,
  /// 407 – Proxy Authentication Required, see [RFC7235, Section 3.2].
  ProxyAuthenticationRequired,
  /// 408 – Request Timeout, see [RFC7231, Section 6.5.7].
  RequestTimeout,
  /// 409 – Conflict, see [RFC7231, Section 6.5.8].
  Conflict,
  /// 410 – Gone, see [RFC7231, Section 6.5.9].
  Gone,
  /// 411 – Length Required, see [RFC7231, Section 6.5.10].
  LengthRequired,
  /// 412 – Precondition Failed, see [RFC7232, Section 4.2][RFC8144, Section 3.2].
  PreconditionFailed,
  /// 413 – Payload Too Large, see [RFC7231, Section 6.5.11].
  PayloadTooLarge,
  /// 414 – URI Too Long, see [RFC7231, Section 6.5.12].
  URITooLong,
  /// 415 – Unsupported Media Type, see [RFC7231, Section 6.5.13][RFC7694, Section 3].
  UnsupportedMediaType,
  /// 416 – Range Not Satisfiable, see [RFC7233, Section 4.4].
  RangeNotSatisfiable,
  /// 417 – Expectation Failed, see [RFC7231, Section 6.5.14].
  ExpectationFailed,
  /// 421 – Misdirected Request, see [RFC7540, Section 9.1.2].
  MisdirectedRequest,
  /// 422 – Unprocessable Entity, see [RFC4918].
  UnprocessableEntity,
  /// 423 – Locked, see [RFC4918].
  Locked,
  /// 424 – Failed Dependency, see [RFC4918].
  FailedDependency,
  /// 425 – Too Early, see [RFC8470].
  TooEarly,
  /// 426 – Upgrade Required, see [RFC7231, Section 6.5.15].
  UpgradeRequired,
  /// 428 – Precondition Required, see [RFC6585].
  PreconditionRequired,
  /// 429 – Too Many Requests, see [RFC6585].
  TooManyRequests,
  /// 431 – Request Header Fields Too Large, see [RFC6585].
  RequestHeaderFieldsTooLarge,
  /// 451 – Unavailable For Legal Reasons, see [RFC7725].
  UnavailableForLegalReasons,
  /// 500 – Internal Server Error, see [RFC7231, Section 6.6.1].
  InternalServerError,
  /// 501 – Not Implemented, see [RFC7231, Section 6.6.2].
  NotImplemented,
  /// 502 – Bad Gateway, see [RFC7231, Section 6.6.3].
  BadGateway,
  /// 503 – Service Unavailable, see [RFC7231, Section 6.6.4].
  ServiceUnavailable,
  /// 504 – Gateway Timeout, see [RFC7231, Section 6.6.5].
  GatewayTimeout,
  /// 505 – HTTP Version Not Supported, see [RFC7231, Section 6.6.6].
  HTTPVersionNotSupported,
  /// 506 – Variant Also Negotiates, see [RFC2295].
  VariantAlsoNegotiates,
  /// 507 – Insufficient Storage, see [RFC4918].
  InsufficientStorage,
  /// 508 – Loop Detected, see [RFC5842].
  LoopDetected,
  /// 510 – Not Extended, see [RFC2774].
  NotExtended,
  /// 511 – Network Authentication Required, see [RFC6585].
  NetworkAuthenticationRequired,
}

impl          Display                   for Status
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
            Status::Continue                      =>  "100 Continue",
            Status::SwitchingProtocols            =>  "101 Switching Protocols",
            Status::Processing                    =>  "102 Processing",
            Status::EarlyHints                    =>  "103 Early Hints",
            Status::Ok                            =>  "200 OK",
            Status::Created                       =>  "201 Created",
            Status::Accepted                      =>  "202 Accepted",
            Status::NonAuthoritativeInformation   =>  "203 Non-Authoritative Information",
            Status::NoContent                     =>  "204 No Content",
            Status::ResetContent                  =>  "205 Reset Content",
            Status::PartialContent                =>  "206 Partial Content",
            Status::MultiStatus                   =>  "207 Multi-Status",
            Status::AlreadyReported               =>  "208 Already Reported",
            Status::IMUsed                        =>  "226 IM Used",
            Status::MultipleChoices               =>  "300 Multiple Choices",
            Status::MovedPermanently              =>  "301 Moved Permanently",
            Status::Found                         =>  "302 Found",
            Status::SeeOther                      =>  "303 See Other",
            Status::NotModified                   =>  "304 Not Modified",
            Status::UseProxy                      =>  "305 Use Proxy",
            Status::TemporaryRedirect             =>  "307 Temporary Redirect",
            Status::PermanentRedirect             =>  "308 Permanent Redirect",
            Status::BadRequest                    =>  "400 Bad Request",
            Status::Unauthorized                  =>  "401 Unauthorized",
            Status::PaymentRequired               =>  "402 Payment Required",
            Status::Forbidden                     =>  "403 Forbidden",
            Status::NotFound                      =>  "404 Not Found",
            Status::MethodNotAllowed              =>  "405 Method Not Allowed",
            Status::NotAcceptable                 =>  "406 Not Acceptable",
            Status::ProxyAuthenticationRequired   =>  "407 Proxy Authentication Required",
            Status::RequestTimeout                =>  "408 Request Timeout",
            Status::Conflict                      =>  "409 Conflict",
            Status::Gone                          =>  "410 Gone",
            Status::LengthRequired                =>  "411 Length Required",
            Status::PreconditionFailed            =>  "412 Precondition Failed",
            Status::PayloadTooLarge               =>  "413 Payload Too Large",
            Status::URITooLong                    =>  "414 URI Too Long",
            Status::UnsupportedMediaType          =>  "415 Unsupported Media Type",
            Status::RangeNotSatisfiable           =>  "416 Range Not Satisfiable",
            Status::ExpectationFailed             =>  "417 Expectation Failed",
            Status::MisdirectedRequest            =>  "421 Misdirected Request",
            Status::UnprocessableEntity           =>  "422 Unprocessable Entity",
            Status::Locked                        =>  "423 Locked",
            Status::FailedDependency              =>  "424 Failed Dependency",
            Status::TooEarly                      =>  "425 Too Early",
            Status::UpgradeRequired               =>  "426 Upgrade Required",
            Status::PreconditionRequired          =>  "428 Precondition Required",
            Status::TooManyRequests               =>  "429 Too Many Requests",
            Status::RequestHeaderFieldsTooLarge   =>  "431 Request Header Fields Too Large",
            Status::UnavailableForLegalReasons    =>  "451 Unavailable For Legal Reasons",
            Status::InternalServerError           =>  "500 Internal Server Error",
            Status::NotImplemented                =>  "501 Not Implemented",
            Status::BadGateway                    =>  "502 Bad Gateway",
            Status::ServiceUnavailable            =>  "503 Service Unavailable",
            Status::GatewayTimeout                =>  "504 Gateway Timeout",
            Status::HTTPVersionNotSupported       =>  "505 HTTP Version Not Supported",
            Status::VariantAlsoNegotiates         =>  "506 Variant Also Negotiates",
            Status::InsufficientStorage           =>  "507 Insufficient Storage",
            Status::LoopDetected                  =>  "508 Loop Detected",
            Status::NotExtended                   =>  "510 Not Extended",
            Status::NetworkAuthenticationRequired =>  "511 Network Authentication Required",
          }
        )
      )
  }
}
