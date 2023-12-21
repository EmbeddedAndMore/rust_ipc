use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use nix::{
    sys::{wait::{wait,waitpid, WaitStatus}, signal, signal::Signal},
    unistd::{fork, ForkResult, Pid}
};

use tokio::signal::{unix::{signal, SignalKind}};
use tokio::runtime::Builder;
fn main() {


    unsafe {
        match fork().expect("failed to create child process") {
            ForkResult::Parent { child }=>{
                println!("Child process created, {}",child);
                // println!("Wait before sleep");
                // sleep(Duration::from_secs(3));
                // println!("sleep finished.");
                // signal::kill(child, Signal::SIGINT).unwrap();
                let rt = Builder::new_multi_thread().enable_io().build().unwrap();
                rt.block_on(async
                    {
                        let mut sigchld = signal(SignalKind::child()).unwrap();

                        match sigchld.recv().await {
                            None => {}
                            Some(sig) => {
                                match wait().unwrap(){
                                    WaitStatus::Exited(pid, exit_code) => {
                                        println!("Received SIGCHLD signal {:?}, {}.", pid, exit_code)
                                    }
                                    WaitStatus::Signaled(_, _, _) => {}
                                    WaitStatus::Stopped(_, _) => {}
                                    WaitStatus::PtraceEvent(_, _, _) => {}
                                    WaitStatus::PtraceSyscall(_) => {}
                                    WaitStatus::Continued(_) => {}
                                    WaitStatus::StillAlive => {}
                                }

                            }
                        }
                    }
                )
            }
            ForkResult::Child =>{
                println!("child created");

                sleep(Duration::from_secs(2));
                println!("finishing child");
                // let rt = Builder::new_multi_thread().enable_io().build().unwrap();

                // rt.block_on(async
                //     {
                //         let mut sigint = signal(SignalKind::interrupt()).unwrap();
                //
                //         match sigint.recv().await {
                //             None => {}
                //             Some(sig) => {
                //                 println!("Received SIGINT signal")
                //             }
                //         }
                //     }
                // )

            }
        }
    }
}
