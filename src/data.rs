type Score = usize;

#[derive(Debug)]
pub struct InputData {
    // TODO add fields
}

#[derive(Debug)]
pub struct OutputData {
    // TODO add fields
}

// You can remove this clippy rule override once this function is implemented
#[allow(unused_variables)]
/// Parse input file content as a `InputData`
///
/// s: input file content
///
/// returns InputData parsed from input file content
pub fn parse_input(s: &str) -> anyhow::Result<InputData> {
    Ok(InputData {})
}

// You can remove this clippy rule override once this function is implemented
#[allow(unused_variables)]
/// Solves the Google Hashcode problem
///
/// Note: Depending on the Google Hashcode and the way the solver works
///       it is often possible to compute the score at the same time as the solution.
///       If the solver happens to be able to compute the score then it can be returned
///       (the score will be displayed as INFO log in main()) otherwise simply return None.
///
/// returns OutputData, Option(Score)
pub fn solve(input: &InputData) -> anyhow::Result<(OutputData, Option<Score>)> {
    Ok((OutputData {}, None))
}

// You can remove this clippy rule override once this function is implemented
#[allow(unused_variables)]
/// Note: it is not necessary to append "\n" to each line.
///       Lines will be written to output file using `writeln!` in main().
///
/// returns output as lines (`Vec<String>`)
pub fn output_as_lines(output: &OutputData) -> anyhow::Result<Vec<String>> {
    Ok(vec![])
}
