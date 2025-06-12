use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;
const CODE_DIGITS: &str = "\
100101102103104105106107108109110111112113114115116117118119\
120121122123124125126127128129130131132133134135136137138139\
140141142143144145146147148149150151152153154155156157158159\
160161162163164165166167168169170171172173174175176177178179\
180181182183184185186187188189190191192193194195196197198199\
200201202203204205206207208209210211212213214215216217218219\
220221222223224225226227228229230231232233234235236237238239\
240241242243244245246247248249250251252253254255256257258259\
260261262263264265266267268269270271272273274275276277278279\
280281282283284285286287288289290291292293294295296297298299\
300301302303304305306307308309310311312313314315316317318319\
320321322323324325326327328329330331332333334335336337338339\
340341342343344345346347348349350351352353354355356357358359\
360361362363364365366367368369370371372373374375376377378379\
380381382383384385386387388389390391392393394395396397398399\
400401402403404405406407408409410411412413414415416417418419\
420421422423424425426427428429430431432433434435436437438439\
440441442443444445446447448449450451452453454455456457458459\
460461462463464465466467468469470471472473474475476477478479\
480481482483484485486487488489490491492493494495496497498499\
500501502503504505506507508509510511512513514515516517518519\
520521522523524525526527528529530531532533534535536537538539\
540541542543544545546547548549550551552553554555556557558559\
560561562563564565566567568569570571572573574575576577578579\
580581582583584585586587588589590591592593594595596597598599\
600601602603604605606607608609610611612613614615616617618619\
620621622623624625626627628629630631632633634635636637638639\
640641642643644645646647648649650651652653654655656657658659\
660661662663664665666667668669670671672673674675676677678679\
680681682683684685686687688689690691692693694695696697698699\
700701702703704705706707708709710711712713714715716717718719\
720721722723724725726727728729730731732733734735736737738739\
740741742743744745746747748749750751752753754755756757758759\
760761762763764765766767768769770771772773774775776777778779\
780781782783784785786787788789790791792793794795796797798799\
800801802803804805806807808809810811812813814815816817818819\
820821822823824825826827828829830831832833834835836837838839\
840841842843844845846847848849850851852853854855856857858859\
860861862863864865866867868869870871872873874875876877878879\
880881882883884885886887888889890891892893894895896897898899\
900901902903904905906907908909910911912913914915916917918919\
920921922923924925926927928929930931932933934935936937938939\
940941942943944945946947948949950951952953954955956957958959\
960961962963964965966967968969970971972973974975976977978979\
980981982983984985986987988989990991992993994995996997998999";
macro_rules! status_codes {
    ($($(#[$docs:meta])* ($num:expr, $konst:ident, $phrase:expr);)+) => {
        impl StatusCode { $($(#[$docs])* pub const $konst : StatusCode =
        StatusCode(unsafe { NonZeroU16::new_unchecked($num) });)+ } fn
        canonical_reason(num : u16) -> Option <&'static str > { match num { $($num =>
        Some($phrase),)+ _ => None } }
    };
}
status_codes! {
    #[doc = " 100 Continue"] #[doc =
    " [[RFC9110, Section 15.2.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.1)]"]
    (100, CONTINUE, "Continue"); #[doc = " 101 Switching Protocols"] #[doc =
    " [[RFC9110, Section 15.2.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.2.2)]"]
    (101, SWITCHING_PROTOCOLS, "Switching Protocols"); #[doc = " 102 Processing"] #[doc =
    " [[RFC2518, Section 10.1](https://datatracker.ietf.org/doc/html/rfc2518#section-10.1)]"]
    (102, PROCESSING, "Processing"); #[doc = " 200 OK"] #[doc =
    " [[RFC9110, Section 15.3.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.1)]"]
    (200, OK, "OK"); #[doc = " 201 Created"] #[doc =
    " [[RFC9110, Section 15.3.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.2)]"]
    (201, CREATED, "Created"); #[doc = " 202 Accepted"] #[doc =
    " [[RFC9110, Section 15.3.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.3)]"]
    (202, ACCEPTED, "Accepted"); #[doc = " 203 Non-Authoritative Information"] #[doc =
    " [[RFC9110, Section 15.3.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.4)]"]
    (203, NON_AUTHORITATIVE_INFORMATION, "Non Authoritative Information"); #[doc =
    " 204 No Content"] #[doc =
    " [[RFC9110, Section 15.3.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.5)]"]
    (204, NO_CONTENT, "No Content"); #[doc = " 205 Reset Content"] #[doc =
    " [[RFC9110, Section 15.3.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.6)]"]
    (205, RESET_CONTENT, "Reset Content"); #[doc = " 206 Partial Content"] #[doc =
    " [[RFC9110, Section 15.3.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.3.7)]"]
    (206, PARTIAL_CONTENT, "Partial Content"); #[doc = " 207 Multi-Status"] #[doc =
    " [[RFC4918, Section 11.1](https://datatracker.ietf.org/doc/html/rfc4918#section-11.1)]"]
    (207, MULTI_STATUS, "Multi-Status"); #[doc = " 208 Already Reported"] #[doc =
    " [[RFC5842, Section 7.1](https://datatracker.ietf.org/doc/html/rfc5842#section-7.1)]"]
    (208, ALREADY_REPORTED, "Already Reported"); #[doc = " 226 IM Used"] #[doc =
    " [[RFC3229, Section 10.4.1](https://datatracker.ietf.org/doc/html/rfc3229#section-10.4.1)]"]
    (226, IM_USED, "IM Used"); #[doc = " 300 Multiple Choices"] #[doc =
    " [[RFC9110, Section 15.4.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.1)]"]
    (300, MULTIPLE_CHOICES, "Multiple Choices"); #[doc = " 301 Moved Permanently"] #[doc
    =
    " [[RFC9110, Section 15.4.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.2)]"]
    (301, MOVED_PERMANENTLY, "Moved Permanently"); #[doc = " 302 Found"] #[doc =
    " [[RFC9110, Section 15.4.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.3)]"]
    (302, FOUND, "Found"); #[doc = " 303 See Other"] #[doc =
    " [[RFC9110, Section 15.4.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.4)]"]
    (303, SEE_OTHER, "See Other"); #[doc = " 304 Not Modified"] #[doc =
    " [[RFC9110, Section 15.4.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.5)]"]
    (304, NOT_MODIFIED, "Not Modified"); #[doc = " 305 Use Proxy"] #[doc =
    " [[RFC9110, Section 15.4.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.6)]"]
    (305, USE_PROXY, "Use Proxy"); #[doc = " 307 Temporary Redirect"] #[doc =
    " [[RFC9110, Section 15.4.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.7)]"]
    (307, TEMPORARY_REDIRECT, "Temporary Redirect"); #[doc = " 308 Permanent Redirect"]
    #[doc =
    " [[RFC9110, Section 15.4.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.4.8)]"]
    (308, PERMANENT_REDIRECT, "Permanent Redirect"); #[doc = " 400 Bad Request"] #[doc =
    " [[RFC9110, Section 15.5.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1)]"]
    (400, BAD_REQUEST, "Bad Request"); #[doc = " 401 Unauthorized"] #[doc =
    " [[RFC9110, Section 15.5.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.2)]"]
    (401, UNAUTHORIZED, "Unauthorized"); #[doc = " 402 Payment Required"] #[doc =
    " [[RFC9110, Section 15.5.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.3)]"]
    (402, PAYMENT_REQUIRED, "Payment Required"); #[doc = " 403 Forbidden"] #[doc =
    " [[RFC9110, Section 15.5.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.4)]"]
    (403, FORBIDDEN, "Forbidden"); #[doc = " 404 Not Found"] #[doc =
    " [[RFC9110, Section 15.5.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.5)]"]
    (404, NOT_FOUND, "Not Found"); #[doc = " 405 Method Not Allowed"] #[doc =
    " [[RFC9110, Section 15.5.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.6)]"]
    (405, METHOD_NOT_ALLOWED, "Method Not Allowed"); #[doc = " 406 Not Acceptable"] #[doc
    =
    " [[RFC9110, Section 15.5.7](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.7)]"]
    (406, NOT_ACCEPTABLE, "Not Acceptable"); #[doc =
    " 407 Proxy Authentication Required"] #[doc =
    " [[RFC9110, Section 15.5.8](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.8)]"]
    (407, PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required"); #[doc =
    " 408 Request Timeout"] #[doc =
    " [[RFC9110, Section 15.5.9](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.9)]"]
    (408, REQUEST_TIMEOUT, "Request Timeout"); #[doc = " 409 Conflict"] #[doc =
    " [[RFC9110, Section 15.5.10](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.10)]"]
    (409, CONFLICT, "Conflict"); #[doc = " 410 Gone"] #[doc =
    " [[RFC9110, Section 15.5.11](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.11)]"]
    (410, GONE, "Gone"); #[doc = " 411 Length Required"] #[doc =
    " [[RFC9110, Section 15.5.12](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.12)]"]
    (411, LENGTH_REQUIRED, "Length Required"); #[doc = " 412 Precondition Failed"] #[doc
    =
    " [[RFC9110, Section 15.5.13](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.13)]"]
    (412, PRECONDITION_FAILED, "Precondition Failed"); #[doc = " 413 Payload Too Large"]
    #[doc =
    " [[RFC9110, Section 15.5.14](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.14)]"]
    (413, PAYLOAD_TOO_LARGE, "Payload Too Large"); #[doc = " 414 URI Too Long"] #[doc =
    " [[RFC9110, Section 15.5.15](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.15)]"]
    (414, URI_TOO_LONG, "URI Too Long"); #[doc = " 415 Unsupported Media Type"] #[doc =
    " [[RFC9110, Section 15.5.16](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.16)]"]
    (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type"); #[doc =
    " 416 Range Not Satisfiable"] #[doc =
    " [[RFC9110, Section 15.5.17](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.17)]"]
    (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable"); #[doc =
    " 417 Expectation Failed"] #[doc =
    " [[RFC9110, Section 15.5.18](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.18)]"]
    (417, EXPECTATION_FAILED, "Expectation Failed"); #[doc = " 418 I'm a teapot"] #[doc =
    " [curiously not registered by IANA but [RFC2324, Section 2.3.2](https://datatracker.ietf.org/doc/html/rfc2324#section-2.3.2)]"]
    (418, IM_A_TEAPOT, "I'm a teapot"); #[doc = " 421 Misdirected Request"] #[doc =
    " [[RFC9110, Section 15.5.20](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.20)]"]
    (421, MISDIRECTED_REQUEST, "Misdirected Request"); #[doc =
    " 422 Unprocessable Entity"] #[doc =
    " [[RFC9110, Section 15.5.21](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.21)]"]
    (422, UNPROCESSABLE_ENTITY, "Unprocessable Entity"); #[doc = " 423 Locked"] #[doc =
    " [[RFC4918, Section 11.3](https://datatracker.ietf.org/doc/html/rfc4918#section-11.3)]"]
    (423, LOCKED, "Locked"); #[doc = " 424 Failed Dependency"] #[doc =
    " [[RFC4918, Section 11.4](https://tools.ietf.org/html/rfc4918#section-11.4)]"] (424,
    FAILED_DEPENDENCY, "Failed Dependency"); #[doc = " 425 Too early"] #[doc =
    " [[RFC8470, Section 5.2](https://httpwg.org/specs/rfc8470.html#status)]"] (425,
    TOO_EARLY, "Too Early"); #[doc = " 426 Upgrade Required"] #[doc =
    " [[RFC9110, Section 15.5.22](https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.22)]"]
    (426, UPGRADE_REQUIRED, "Upgrade Required"); #[doc = " 428 Precondition Required"]
    #[doc =
    " [[RFC6585, Section 3](https://datatracker.ietf.org/doc/html/rfc6585#section-3)]"]
    (428, PRECONDITION_REQUIRED, "Precondition Required"); #[doc =
    " 429 Too Many Requests"] #[doc =
    " [[RFC6585, Section 4](https://datatracker.ietf.org/doc/html/rfc6585#section-4)]"]
    (429, TOO_MANY_REQUESTS, "Too Many Requests"); #[doc =
    " 431 Request Header Fields Too Large"] #[doc =
    " [[RFC6585, Section 5](https://datatracker.ietf.org/doc/html/rfc6585#section-5)]"]
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large"); #[doc =
    " 451 Unavailable For Legal Reasons"] #[doc =
    " [[RFC7725, Section 3](https://tools.ietf.org/html/rfc7725#section-3)]"] (451,
    UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons"); #[doc =
    " 500 Internal Server Error"] #[doc =
    " [[RFC9110, Section 15.6.1](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1)]"]
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error"); #[doc =
    " 501 Not Implemented"] #[doc =
    " [[RFC9110, Section 15.6.2](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.2)]"]
    (501, NOT_IMPLEMENTED, "Not Implemented"); #[doc = " 502 Bad Gateway"] #[doc =
    " [[RFC9110, Section 15.6.3](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.3)]"]
    (502, BAD_GATEWAY, "Bad Gateway"); #[doc = " 503 Service Unavailable"] #[doc =
    " [[RFC9110, Section 15.6.4](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.4)]"]
    (503, SERVICE_UNAVAILABLE, "Service Unavailable"); #[doc = " 504 Gateway Timeout"]
    #[doc =
    " [[RFC9110, Section 15.6.5](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.5)]"]
    (504, GATEWAY_TIMEOUT, "Gateway Timeout"); #[doc = " 505 HTTP Version Not Supported"]
    #[doc =
    " [[RFC9110, Section 15.6.6](https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.6)]"]
    (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported"); #[doc =
    " 506 Variant Also Negotiates"] #[doc =
    " [[RFC2295, Section 8.1](https://datatracker.ietf.org/doc/html/rfc2295#section-8.1)]"]
    (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates"); #[doc =
    " 507 Insufficient Storage"] #[doc =
    " [[RFC4918, Section 11.5](https://datatracker.ietf.org/doc/html/rfc4918#section-11.5)]"]
    (507, INSUFFICIENT_STORAGE, "Insufficient Storage"); #[doc = " 508 Loop Detected"]
    #[doc =
    " [[RFC5842, Section 7.2](https://datatracker.ietf.org/doc/html/rfc5842#section-7.2)]"]
    (508, LOOP_DETECTED, "Loop Detected"); #[doc = " 510 Not Extended"] #[doc =
    " [[RFC2774, Section 7](https://datatracker.ietf.org/doc/html/rfc2774#section-7)]"]
    (510, NOT_EXTENDED, "Not Extended"); #[doc = " 511 Network Authentication Required"]
    #[doc =
    " [[RFC6585, Section 6](https://datatracker.ietf.org/doc/html/rfc6585#section-6)]"]
    (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");
}
pub struct InvalidStatusCode {
    _priv: (),
}
impl fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid status code")
    }
}
