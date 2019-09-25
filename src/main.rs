use gwasm_api::{dispatcher, SplitContext};

fn main() {
    dispatcher::run(split, exec, merge).unwrap()
}

type Task = Vec<u64>;
type TaskResult = u64;

fn split(_ctx: &mut dyn SplitContext) -> Vec<(Task,)> {
    const NUM_SUBTASKS: usize = 10;
    let arr: Vec<u64> = (1..=100).collect();
    let mut tasks: Vec<(Task,)> = Vec::new();
    for chunk in arr.chunks(NUM_SUBTASKS) {
        let task: Task = chunk.to_vec();
        tasks.push((task,));
    }
    tasks
}

fn exec(task: Task) -> (TaskResult,) {
    let task_result: u64 = task.into_iter().sum();
    (task_result,)
}

fn merge(_args: &Vec<String>, results: Vec<((Task,), (TaskResult,))>) {
    let task_results: Vec<TaskResult> = results.into_iter().map(|(_, (result,))| result).collect();
    let final_sum: u64 = task_results.into_iter().sum();
    let expected: u64 = (1..=100).sum();
    assert_eq!(final_sum, expected, "the sums should be equal")
}
