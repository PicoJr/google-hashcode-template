#![allow(dead_code)]
use log::error;
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::combinator::{map_res, verify};
use nom::error::{context, convert_error, VerboseError};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

pub(crate) type N = usize;
pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn number(input: &str) -> Res<&str, &str> {
    context("number", take_while1(|c: char| c.is_digit(10)))(input)
}

fn single_space(input: &str) -> Res<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}

fn positive_number(input: &str) -> Res<&str, N> {
    map_res(number, |out| N::from_str_radix(out, 10))(input)
}

fn number_list(s: &str) -> Res<&str, Vec<N>> {
    separated_list1(single_space, positive_number)(s)
}

/// Parse a space-separated list of (positive) integers written on a line ending with "\n"
pub(crate) fn number_list_line(s: &str) -> Res<&str, Vec<N>> {
    terminated(number_list, tag("\n"))(s)
}

/// Parse a space-separated list of (positive) integers written on a line ending with "\n"
///
/// logs errors if any
pub(crate) fn number_list_line_verbose(s: &str) -> Res<&str, Vec<N>> {
    verbose_error(s, number_list_line(s))
}

fn number_list_exact(s: &str, expected_size: usize) -> Res<&str, Vec<N>> {
    verify(
        separated_list1(single_space, positive_number),
        |s: &[N]| s.len() == expected_size,
    )(s)
}

/// Parse a space-separated list of exactly `expected_size` (positive) integers written on a line ending with "\n"
pub(crate) fn number_list_line_exact(s: &str, expected_size: usize) -> Res<&str, Vec<N>> {
    context(
        "parsing exact numbers of integers",
        terminated(|s| number_list_exact(s, expected_size), tag("\n")),
    )(s)
}

/// Parse a space-separated list of exactly `expected_size` (positive) integers written on a line ending with "\n"
///
/// logs errors if any
pub(crate) fn number_list_line_exact_verbose(s: &str, expected_size: usize) -> Res<&str, Vec<N>> {
    verbose_error(s, number_list_line_exact(s, expected_size))
}

/// wrapper for parsing functions, formats and logs errors if any.
pub(crate) fn verbose_error<'a, U>(s: &str, res: Res<&'a str, U>) -> Res<&'a str, U> {
    if let Err(nom::Err::Error(ref e)) = res {
        error!("{}", convert_error(s, e.clone()));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        number_list, number_list_exact, number_list_line, number_list_line_exact, verbose_error,
    };
    use nom::error::ErrorKind::Verify;
    use nom::error::VerboseError;
    use nom::error::VerboseErrorKind::{Context, Nom};

    #[test]
    fn test_number_list() {
        let nbl = number_list("42 43 44");
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_number_list_exact() {
        let nbl = number_list_exact("42 43 44", 3);
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_number_list_line() {
        let nbl = number_list_line("42 43 44\n");
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_number_list_line_exact() {
        let nbl = number_list_line_exact("42 43 44\n", 3);
        assert_eq!(nbl, Ok(("", vec![42, 43, 44])))
    }

    #[test]
    fn test_verbose_error() {
        let data = "42 43 44\n";
        let nbl = verbose_error(data, number_list_line_exact(data, 4));
        assert_eq!(
            nbl,
            Err(nom::Err::Error(VerboseError {
                errors: vec![
                    ("42 43 44\n", Nom(Verify)),
                    ("42 43 44\n", Context("parsing exact numbers of integers"))
                ]
            }))
        )
    }
}
