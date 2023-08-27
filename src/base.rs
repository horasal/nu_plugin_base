use nu_plugin::LabeledError;
use nu_protocol::Span;
use num_bigint::BigUint;
use num_traits::Zero;

pub fn str_to_binary(s: &str, table: &str, span: &Span) -> Result<Vec<u8>, LabeledError> {
    s.chars()
        .map(|c| {
            table.find(c).map(|c| c as u8).ok_or(LabeledError {
                msg: "Input contains unknown chars than not in table".into(),
                label: "incorrect table or input".into(),
                span: Some(span.clone()),
            })
        })
        .collect()
}

pub fn convert(buffer: &[u8], from: usize, _to: usize) -> Vec<u8> {
    let mut base: BigUint = Zero::zero();
    for i in buffer {
        base = base * from + i;
    }
    base.to_bytes_be()
}
