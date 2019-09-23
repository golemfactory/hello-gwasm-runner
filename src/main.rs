use failure::Fallible;
use gwasm_api::{dispatcher, SplitContext};

fn main() -> Fallible<()> {
    dispatcher::run(split, exec, merge)
}

type Task = Vec<u64>;
type TaskResult = u64;

const NUM_SUBTASKS: usize = 10;

fn split(_ctx: &mut dyn SplitContext) -> Vec<(Task,)> {
    let arr: Vec<u64> = (1..=100).collect();
    let mut tasks = Vec::new();
    for chunk in arr.chunks(NUM_SUBTASKS) {
        let task = (chunk.to_vec(),);
        tasks.push(task);
    }
    tasks
}

fn exec(task: Task) -> (TaskResult,) {
    (task.into_iter().sum(),)
}

fn merge(_args: &Vec<String>, results: Vec<((Task,), (TaskResult,))>) {
    let given: u64 = results.iter().map(|(_, (result,))| result).sum();
    let expected: u64 = (1..=100).sum();
    assert_eq!(expected, given, "sums should be equal")
}
