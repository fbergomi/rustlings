// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::*;
use thread::JoinHandle;

const JOB_RUN_DELAY: u64 = 2000;
const WATCH_PERIOD: u64 = 100;

struct JobStatus {
    completed_jobs: u32,
    running_jobs: u32,
    start_time: Instant,
    jobs: Vec<Arc<Mutex<OneJobStatus>>>,
    jobs_2_handles: HashMap<u32, JoinHandle<()>>,
}

struct OneJobStatus {
    id: u32,
    creation_date: Option<Instant>,
    run_date: Option<Instant>,
    completion_date: Option<Instant>,
}

impl OneJobStatus {
    fn print_job(&self) {
        let is_created = self.creation_date.is_some();
        let has_run = self.run_date.is_some();
        let has_completed = self.completion_date.is_some();

        let mut status = "".to_string();

        if is_created {
            status.push_str("+,");
        } else {
            status.push_str("-,");
        }

        if has_run {
            status.push_str("+,");
        } else {
            status.push_str("-,");
        }

        if has_completed {
            status.push_str("+");
        } else {
            status.push_str("-");
        }

        println!("\t<{:0>3}> : [{}]", self.id, status);
    }
}

fn join_watching_thread(status_shared: Arc<Mutex<JobStatus>>) {
    loop {
        let mut everyone_has_finished = true;
        thread::sleep(Duration::from_millis(WATCH_PERIOD));
        let status = status_shared.lock().unwrap();
        let iterator = status.jobs_2_handles.iter();
        for (job, handle) in iterator {
            //println!("{:} {:}", job, handle.is_finished());
            if handle.is_finished() {
                println!("==>> FINISHED JOB : {}", job);
            } else {
                everyone_has_finished = false;
            }
        }

        if everyone_has_finished {
            break;
        }
    }
}

fn watching_thread(status_shared: Arc<Mutex<JobStatus>>) {
    loop {
        thread::sleep(Duration::from_millis(WATCH_PERIOD));
        let status = status_shared.lock().unwrap();
        println!(
            "Status : [running {} , finished {}]",
            status.running_jobs, status.completed_jobs
        );
        for job in &status.jobs {
            job.lock().unwrap().print_job();
        }
    }
}

fn jobs_thread(status_shared: Arc<Mutex<JobStatus>>, id: u32) {
    let job: OneJobStatus = OneJobStatus {
        id: id,
        creation_date: Option::Some(Instant::now()),
        run_date: Option::None,
        completion_date: Option::None,
    };

    let mut status = status_shared.lock().unwrap();
    let currentjob = Arc::new(Mutex::new(job));

    status.jobs.push(currentjob.clone());

    status.running_jobs += 1;
    let elapsed = status.start_time.elapsed();
    println!("\tjob {} starts running at {}ms", id, elapsed.as_millis());
    let run_date = Option::Some(Instant::now());
    currentjob.lock().unwrap().run_date = run_date;
    //drop(currentjob);
    drop(status);

    let wait_time_milli: u64 = JOB_RUN_DELAY;
    println!("\t\tJob {} forces a wait for {} ms", id, wait_time_milli,);
    thread::sleep(Duration::from_millis(wait_time_milli));

    let mut status = status_shared.lock().unwrap();
    let complete_date = Option::Some(Instant::now());
    currentjob.lock().unwrap().completion_date = complete_date;
    let elapsed = status.start_time.elapsed();
    println!("\tJob {} completed at {}ms", id, elapsed.as_millis());
    status.completed_jobs += 1;
    status.running_jobs -= 1;
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus {
        completed_jobs: 0,
        running_jobs: 0,
        start_time: Instant::now(),
        jobs: Vec::new(),
        jobs_2_handles: HashMap::<u32, JoinHandle<()>>::new(),
    }));

    println!("===== Starting jobs ====");

    let status_for_watch = Arc::clone(&status);
    thread::spawn(move || watching_thread(status_for_watch));

    let status_for_join_watch = Arc::clone(&status);
    let join_watching_thread = thread::spawn(move || join_watching_thread(status_for_join_watch));

    build_jobs(status);

    let finished = join_watching_thread.join();

    /*
    let mut index = 1;
    for handle in handles {
        println!("Waiting for job {}", index);
        handle.join().unwrap();

        println!("Joined job {}", index);
        let my_status = status.lock().unwrap();
        println!("Nb of jobs completed {}", my_status.completed_jobs);
        index += 1;
        thread::sleep(Duration::from_millis(10));
    }
    */
}

fn build_jobs(status: Arc<Mutex<JobStatus>>) {
    //let mut handles = Vec::new();
    for i in 1..=10 {
        println!("Launch Job Creation {}", i);

        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || jobs_thread(status_shared, i));

        //handles.push(handle);
        update_jobs_to_handle_map(status.clone(), i, handle);
        thread::sleep(Duration::from_millis(JOB_RUN_DELAY / 5));
    }
    println!(">>>Everyone is running");

    let start_status = status.lock().unwrap();
    println!(
        "===== All job started at {}ms ====",
        start_status.start_time.elapsed().as_millis()
    );
    drop(start_status);
}

fn update_jobs_to_handle_map(status: Arc<Mutex<JobStatus>>, id: u32, handle: JoinHandle<()>) {
    let mut my_status = status.lock().unwrap();
    my_status.jobs_2_handles.insert(id, handle);
}
