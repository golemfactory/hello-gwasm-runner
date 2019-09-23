use failure::Fallible;
use gwasm_api::{dispatcher, SplitContext};

fn main() -> Fallible<()> {
    dispatcher::run(
        move |_: &mut dyn SplitContext| {
            const NUM_SUBTASKS: usize = 10;
            let arr: Vec<u64> = (1..=100).collect();
            let chunks: Vec<_> = arr.chunks(NUM_SUBTASKS).map(|x| (Vec::from(x),)).collect();
            chunks.into_iter()
        },
        move |task: Vec<u64>| {
            let sum = task.into_iter().sum();
            (sum,)
        },
        move |_: &Vec<String>, results: Vec<(_, _)>| {
            let given: u64 = results.iter().map(|(_, (result,))| result).sum();
            let expected: u64 = (1..=100).sum();
            assert_eq!(expected, given, "sums should be equal")
        },
    )
}
