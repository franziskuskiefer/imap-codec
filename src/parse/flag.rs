use crate::{
    parse::core::atom,
    types::{
        core::Atom,
        flag::{Flag, FlagNameAttribute},
    },
};
use abnf_core::streaming::SP;
use nom::{
    branch::alt,
    bytes::streaming::{tag, tag_no_case},
    combinator::{map, value},
    multi::{separated_list, separated_nonempty_list},
    sequence::{delimited, preceded},
    IResult,
};

/// flag = "\Answered" /
///        "\Flagged" /
///        "\Deleted" /
///        "\Seen" /
///        "\Draft" /
///        flag-keyword /
///        flag-extension
///          ; Does not include "\Recent"
pub fn flag(input: &[u8]) -> IResult<&[u8], Flag> {
    alt((
        value(Flag::Answered, tag_no_case(b"\\Answered")),
        value(Flag::Flagged, tag_no_case(b"\\Flagged")),
        value(Flag::Deleted, tag_no_case(b"\\Deleted")),
        value(Flag::Seen, tag_no_case(b"\\Seen")),
        value(Flag::Draft, tag_no_case(b"\\Draft")),
        flag_keyword,
        map(flag_extension, Flag::Extension),
    ))(input)
}

/// flag-fetch = flag / "\Recent"
pub fn flag_fetch(input: &[u8]) -> IResult<&[u8], Flag> {
    alt((flag, value(Flag::Recent, tag_no_case(b"\\Recent"))))(input)
}

/// flag-perm = flag / "\*"
pub fn flag_perm(input: &[u8]) -> IResult<&[u8], Flag> {
    alt((flag, value(Flag::Permanent, tag(b"\\*"))))(input)
}

/// flag-keyword = atom
pub fn flag_keyword(input: &[u8]) -> IResult<&[u8], Flag> {
    map(atom, Flag::Keyword)(input)
}

/// flag-list = "(" [flag *(SP flag)] ")"
pub fn flag_list(input: &[u8]) -> IResult<&[u8], Vec<Flag>> {
    delimited(tag(b"("), separated_list(SP, flag), tag(b")"))(input)
}

/// mbx-list-flags = *(mbx-list-oflag SP) mbx-list-sflag *(SP mbx-list-oflag) /
///                                       mbx-list-oflag *(SP mbx-list-oflag)
///
/// Note:
/// ABNF is so weird, because it enforces that sflag is only used once (or not at all).
/// We just parse any flag and check for multiple occurrences of sflag later.
pub fn mbx_list_flags(input: &[u8]) -> IResult<&[u8], Vec<FlagNameAttribute>> {
    let (remaining, flags) =
        separated_nonempty_list(SP, alt((mbx_list_sflag, mbx_list_oflag)))(input)?;

    if flags
        .iter()
        .filter(|&flag| {
            [
                FlagNameAttribute::Noselect,
                FlagNameAttribute::Marked,
                FlagNameAttribute::Unmarked,
            ]
            .contains(flag)
        })
        .count()
        > 1
    {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    Ok((remaining, flags))
}

/// mbx-list-oflag = "\Noinferiors" /
///                  flag-extension
///                    ; Other flags; multiple possible per LIST response
pub fn mbx_list_oflag(input: &[u8]) -> IResult<&[u8], FlagNameAttribute> {
    alt((
        value(
            FlagNameAttribute::Noinferiors,
            tag_no_case(b"\\Noinferiors"),
        ),
        map(flag_extension, FlagNameAttribute::Extension),
    ))(input)
}

/// mbx-list-sflag = "\Noselect" /
///                  "\Marked" /
///                  "\Unmarked"
///                    ; Selectability flags; only one per LIST response
pub fn mbx_list_sflag(input: &[u8]) -> IResult<&[u8], FlagNameAttribute> {
    alt((
        value(FlagNameAttribute::Noselect, tag_no_case(b"\\Noselect")),
        value(FlagNameAttribute::Marked, tag_no_case(b"\\Marked")),
        value(FlagNameAttribute::Unmarked, tag_no_case(b"\\Unmarked")),
    ))(input)
}

/// flag-extension = "\" atom
///                   ; Future expansion.  Client implementations
///                   ; MUST accept flag-extension flags.  Server
///                   ; implementations MUST NOT generate
///                   ; flag-extension flags except as defined by
///                   ; future standard or standards-track
///                   ; revisions of this specification.
pub fn flag_extension(input: &[u8]) -> IResult<&[u8], Atom> {
    preceded(tag(b"\\"), atom)(input)
}
