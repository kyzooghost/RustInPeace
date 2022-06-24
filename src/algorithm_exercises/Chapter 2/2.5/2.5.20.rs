#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 2.5.20

// Idle time. Suppose that a parallel machine processes N jobs. Write a program that, given the list of job start and finish times, finds the largest interval where the machine is idle and the largest interval where the machine is not idle.

// Ok, mm not sure if an interval search tree is the best approach here
// Let's just use a weird data type

mod utils {pub mod BinarySearchTree;}
use utils::BinarySearchTree::BinarySearchTree as BinarySearchTree;

#[derive(Debug, Clone)]
struct Job {
    start: usize,
    end: usize
}

#[derive(Debug, Clone)]
struct Rest {
    start: usize,
    end: usize
}

fn main() {
    let mut vec: Vec<Job> = Vec::new();
    vec.push(Job{start: 5, end: 6});
    vec.push(Job{start: 12, end: 14});
    vec.push(Job{start: 1, end: 3});
    vec.push(Job{start: 9, end: 12});
    vec.push(Job{start: 15, end: 16});

    println!("{:?}", find_largest_idle_interval(vec));

}

fn find_largest_idle_interval(vec: Vec<Job>) -> (Job, Rest) {
    // Maintain internal map of working times

    let mut working_times: Vec<Job> = Vec::new();

    // Cases - No intersection - Add one interval before, Add one interval after
    // Intersection - Extend to the left, complete overlap, or extend to the right

    for (i, job) in vec.iter().enumerate() {
        // First job, just add as is
        if i == 0 {working_times.push(job.clone())}
        else {
            // No intersection case - insert at start
            if job.end < working_times[0].start {
                working_times.insert(0, job.clone());

            // No intersection case - insert at end
            } else if job.start > working_times[working_times.len() - 1].end {
                working_times.push(job.clone());

            } else {
                let working_times_length = working_times.len();
                let working_times_clone = working_times.clone();

                for (i, working_interval) in working_times.iter_mut().enumerate() {
                    // No intersection case - between two intervals
                    if i + 1 <= working_times_length - 1 && job.start > working_interval.end && job.end < working_times_clone[i+1].start {
                        working_times.insert(i + 1, job.clone());
                        break; // Break out of inner loop, insertion found

                    // Intersection case - extend to the left only
                    } else if job.start < working_interval.start && job.end >= working_interval.start && job.end <= working_interval.end {
                        working_interval.start = job.start;
                        break;

                    // Intersection case - extend to the right only
                    } else if job.start >= working_interval.start && job.start <= working_interval.end && job.end > working_interval.end {
                        working_interval.end = job.end;
                        break;

                    // Intersection case - extend to the left and the right
                    // How far to the left does it extend?
                    // How far to the right does it extend?
                    // Can replace current interval
                    // !! Didn't deal properly with edge case here, where new job extends
                    // beyond more than 1 previous job
                    } else if job.start <= working_interval.start && job.end >= working_interval.end {
                        working_interval.start = job.start;
                        working_interval.end = job.end;
                    }
                }
            }
        }
    }

    // Create resting times
    let mut resting_times: Vec<Rest> = Vec::new();

    // Fill in for period before each working_time
    for (i, working_time) in working_times.iter().enumerate() {
        // Edge case for first working interval
        if i == 0 {
            if working_time.start > 0 {
                resting_times.push(Rest{
                    start: 0,
                    end: working_time.start
                })
            }
        } else {
            resting_times.push(Rest{
                start: working_times[i - 1].end,
                end: working_time.start
            })
        }
    }

    let mut largest_resting_period = Rest{start: 0, end: 0};
    let mut largest_working_period = Job{start: 0, end: 0};

    for working_time in working_times.iter() {
        if working_time.end - working_time.start > largest_working_period.end - largest_working_period.start {
            largest_working_period = working_time.clone();
        }
    }

    for resting_time in resting_times.iter() {
        if resting_time.end - resting_time.start > largest_resting_period.end - largest_resting_period.start {
            largest_resting_period = resting_time.clone();
        }
    }

    (largest_working_period, largest_resting_period)
}