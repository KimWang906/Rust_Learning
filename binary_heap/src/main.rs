use std::collections::BinaryHeap;

// use case : priority queue(우선순위 큐)

fn show_reminder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut reminder_vec = vec![];
    for number in input {
        reminder_vec.push(*number);
    }
    reminder_vec
}

fn main() {
    // let many_numbers = vec![244, 211, 174, 101, 88, 95, 253];

    // // 첫 번째 값만 정렬되어 나오고 나머지는 순서없이 나온다.
    // let mut my_heap = BinaryHeap::new();

    // for number in many_numbers {
    //     my_heap.push(number);
    // }

    // while let Some(number) = my_heap.pop() {
    //     println!("Poped off {}. Remaining numbers are : {:?}", number, show_reminder(&my_heap));
    // }

    // .peek() method 사용 가능
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Write back to email from the CEO"));
    jobs.push((80, "Finish report today"));
    jobs.push((5, "Watch some Youtube"));
    jobs.push((70, "Tell your team members thanks for alaways working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some(job) = jobs.pop() {
        println!("You need to: {:?}", job.1);
    }
}
