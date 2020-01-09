#[derive(Debug)]
pub(crate) struct RecipientLine {
    tag: String,
    args: Vec<String>,
    body: Vec<u8>,
}

pub(super) mod read {
    use nom::{combinator::map, IResult};

    use super::*;
    use crate::format::read::recipient_stanza;

    pub(crate) fn recipient_line(input: &[u8]) -> IResult<&[u8], RecipientLine> {
        map(recipient_stanza, |stanza| RecipientLine {
            tag: stanza.tag.to_string(),
            args: stanza.args.into_iter().map(|s| s.to_string()).collect(),
            body: stanza.body,
        })(input)
    }
}

pub(super) mod write {
    use cookie_factory::{combinator::string, multi::separated_list, sequence::tuple, SerializeFn};
    use std::io::Write;
    use std::iter;

    use super::*;
    use crate::util::write::wrapped_encoded_data;

    pub(crate) fn recipient_line<'a, W: 'a + Write>(
        r: &'a RecipientLine,
    ) -> impl SerializeFn<W> + 'a {
        tuple((
            separated_list(
                string(" "),
                iter::once(&r.tag)
                    .chain(r.args.iter())
                    .map(|arg| string(arg)),
            ),
            string("\n"),
            wrapped_encoded_data(&r.body),
        ))
    }
}
