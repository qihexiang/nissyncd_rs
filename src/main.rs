use notify_debouncer_mini::{
    new_debouncer_opt, Config,
    notify::{self, PollWatcher}, notify::{ErrorKind as NotifyErrorKind, RecursiveMode, Result}, Debouncer,
};
use std::env;
use std::io::ErrorKind as IoErrorKind;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<()> {
    let yp_directory = env::var("YP_DIRECTORY").unwrap_or("/var/yp".to_string());
    let notify_config = notify::Config::default().with_poll_interval(Duration::from_secs(2));
    let (tx, rx) = channel();
    let mut debouncer: Debouncer<PollWatcher> = new_debouncer_opt(Config::default().with_timeout(Duration::from_secs(2)).with_notify_config(notify_config), tx)?;

    let watcher = debouncer.watcher();

    for filename in ["/etc/passwd", "/etc/group", "/etc/shadow", "/etc/gshadow"] {
        match watcher.watch(Path::new(filename), RecursiveMode::NonRecursive) {
            Ok(_) => println!("{} watched", filename),
            Err(err) => match err.kind {
                NotifyErrorKind::Io(err) => match err.kind() {
                    IoErrorKind::NotFound => println!(
                        "No such file {}, it may related to your distribution",
                        filename
                    ),
                    _ => panic!("Unable to watch file: {}, reason: \n{:?}", filename, err),
                },
                _ => panic!("Filed to watch file {}, error: {:?}", filename, err),
            },
        }
    }

    for event in rx {
        match event {
            Ok(_) => {
                println!("Files changed!");
                Command::new("make")
                .current_dir(&yp_directory)
                .spawn()
                .expect("failed to execute rebuild process, is make command and the yp directory exist?")
                .wait()
                .expect("failed to wait for the rebuild process");
                println!("Rebuild finished!")
            }
            Err(err) => println!("Something is wrong: {:?}", err),
        }
    }

    Ok(())
}
