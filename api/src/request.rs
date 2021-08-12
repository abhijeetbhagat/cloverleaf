enum PayloadType {
    Offer,
    Answer,
}

pub struct Payload {
    pt: PayloadType,
    payload: String,
}
