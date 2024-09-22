use crate::bindings::export;
use crate::bindings::exports::wasi::http::incoming_handler::Guest;
use crate::bindings::wasi::http::types::*;

struct LeptosServer;

impl Guest for LeptosServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap_or_default();

        let headers = Fields::new();
        headers
            .set(
                &FieldKey::from("Content-Type"),
                &[FieldValue::from("text/plain; charset=utf-8")],
            )
            .expect("setting content-type header");

        let response = OutgoingResponse::new(headers);
        let body = response.body().expect("outgoing request's body");

        ResponseOutparam::set(response_out, Ok(response));

        let body_stream = body.write().expect("outgoing body write stream");
        body_stream
            .blocking_write_and_flush(format!("You requested the path {} :D", path).as_bytes())
            .expect("writing to body");

        drop(body_stream);
        OutgoingBody::finish(body, None).unwrap();
    }
}

export!(LeptosServer with_types_in crate::bindings);
