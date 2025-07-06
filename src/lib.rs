#[cfg(target_arch = "wasm32")]
pub mod wasm;

/// Module for countdown timer functionalities.

pub mod timer {
    use std::{io::Write, thread, time::Duration};

    pub trait TimerTrait {
        fn new(hours: u32, minutes: u32, seconds: u32) -> Result<TimerStruct, &'static str>;
        fn start_timer<W: Write>(&self, writer: &mut W);
    }

    /// Represents a countdown timer.
    #[derive(Clone, Copy, Debug)]
    pub struct TimerStruct {
        /// The total duration of the timer in seconds.
        pub duration: u32,
        /// The initial hours component of the timer.
        pub hours: u32,
        /// The initial minutes component of the timer.
        pub minutes: u32,
        /// The initial seconds component of the timer.
        pub seconds: u32,
    }

    impl TimerTrait for TimerStruct {
        /// Creates a new `TimerStruct` instance.
        ///
        /// # Arguments
        ///
        /// * `hours` - The hours component of the timer duration.
        /// * `minutes` - The minutes component of the timer duration.
        /// * `seconds` - The seconds component of the timer duration.
        ///
        /// # Returns
        ///
        /// * `Ok(TimerStruct)` if the total duration calculated from `hours`, `minutes`,
        ///   and `seconds` is greater than 0.
        /// * `Err("Duration need to be 1 or more seconds.")` if the total duration is 0.
        ///
        /// # Examples
        ///
        /// ```
        /// use your_crate_name::TimerStruct; // Replace your_crate_name with your actual crate name
        ///
        /// let timer = TimerStruct::new(0, 1, 30).expect("Failed to create timer"); // 1 minute 30 seconds
        /// let invalid_timer = TimerStruct::new(0, 0, 0); // This will return an Err
        /// ```
        fn new(hours: u32, minutes: u32, seconds: u32) -> Result<TimerStruct, &'static str> {
            let duration = (hours * 3600) + (minutes * 60) + seconds;

            if duration == 0 {
                return Err("Duration need to be 1 or more seconds.");
            }

            Ok(TimerStruct {
                duration,
                hours,
                minutes,
                seconds,
            })
        }

        /// Starts the countdown timer.
        ///
        /// The timer will print the remaining time to the provided writer every second,
        /// overwriting the previous line. When the timer reaches 0, it prints the final
        /// `0:0:0` with a newline and stops.
        ///
        /// # Arguments
        ///
        /// * `writer` - A mutable reference to any type that implements the `std::io::Write`
        ///              trait (e.g., `&mut std::io::Stdout`).
        ///
        /// # Examples
        ///
        /// ```
        /// use clock-timer::TimerStruct; // Replace your_crate_name with your actual crate name
        /// use std::io::{self, stdout};
        ///
        /// let timer = TimerStruct::new(0, 0, 5).unwrap(); // 5-second timer
        /// let mut writer = stdout();
        /// timer.start_timer(&mut writer);
        /// println!("Timer finished!");
        /// ```
        fn start_timer<W: Write>(&self, writer: &mut W) {
            let mut current_duration = self.duration;
            let one_second = Duration::from_secs(1);

            loop {
                // Calculate display components from the current total duration
                let display_hours = current_duration / 3600;
                let remaining_seconds_after_hours = current_duration % 3600;
                let display_minutes = remaining_seconds_after_hours / 60;
                let display_seconds = remaining_seconds_after_hours % 60;

                let time_display_string =
                    format!("{}:{}:{}", display_hours, display_minutes, display_seconds);

                if current_duration == 0 {
                    // If duration is 0, this is the final display. Print with a newline and break.
                    writeln!(writer, "{}", time_display_string).unwrap();
                    break;
                } else {
                    // For all other durations, print with a carriage return to overwrite the line.
                    write!(writer, "{}\r", time_display_string).unwrap();
                    writer.flush().unwrap(); // Ensure the output is flushed immediately
                }

                thread::sleep(one_second);
                current_duration -= 1;
            }
        }
    }
}

/// Re-exports `TimerStruct` from the `timer` module for easier access.
pub use timer::TimerStruct;

/// Module for stopwatch functionalities.
pub mod stopwatch {
    #[cfg(not(target_arch = "wasm32"))]
    use ctrlc;
    use std::{
        io::Write,
        process,
        sync::{
            Arc,
            atomic::{AtomicU32, Ordering},
        },
        thread,
        time::Duration,
    };

    pub trait StopwatchTrait<T>
    where
        T: Fn(u32) + std::marker::Send + Copy + 'static,
    {
        fn new(operation_on_stop: T) -> StopwatchStruct<T>;
        fn start_stopwatch<W: Write>(&mut self, writer: &mut W);
    }

    /// Represents the current status of the stopwatch.
    #[derive(Clone, Debug)]
    pub enum StopwatchStatus {
        /// The stopwatch is currently stopped.
        Stopped,
        /// The stopwatch is currently running.
        Running,
    }

    /// Represents a stopwatch that measures elapsed time.
    ///
    /// It takes a generic type `T` which must be a closure that accepts a `u32`
    /// (the final `current_time` when the stopwatch stops).
    #[derive(Debug, Clone)]
    pub struct StopwatchStruct<T>
    where
        T: Fn(u32) + std::marker::Send + Copy + 'static,
    {
        /// The current elapsed time in seconds.
        pub current_time: u32,
        /// The current status of the stopwatch (Running or Stopped).
        pub status: StopwatchStatus,
        /// A closure that will be executed when the stopwatch is stopped.
        /// It receives the final `current_time` as an argument.
        pub operation_on_stop: T,
    }

    impl<T> StopwatchStruct<T>
    where
        T: Fn(u32) + std::marker::Send + Copy + 'static,
    {
        /// Creates a new `StopwatchStruct` instance.
        ///
        /// # Arguments
        ///
        /// * `operation_on_stop` - A closure that will be called when the stopwatch status
        ///   is set to `Stopped`. It receives the total elapsed time in seconds as its argument.
        ///
        /// # Returns
        ///
        /// A new `StopwatchStruct` initialized with `current_time` at 0 and `status` as `Running`.
        ///
        /// # Examples
        ///
        /// ```
        /// use clock-timer::stopwatch::{StopwatchStruct, StopwatchStatus}; // Replace clock-timer
        ///
        /// let mut stopwatch = StopwatchStruct::new(|time| {
        ///     println!("Stopwatch stopped at {} seconds.", time);
        /// });
        /// ```
        pub fn new(operation_on_stop: T) -> StopwatchStruct<T> {
            StopwatchStruct {
                current_time: 0,
                status: StopwatchStatus::Running,
                operation_on_stop,
            }
        }

        /// Starts the stopwatch.
        ///
        /// The stopwatch will increment its `current_time` every second and print the elapsed time
        /// to the provided writer, overwriting the previous line.
        ///
        /// The timer can be stopped in two ways:
        /// 1.  Pressing `Ctrl+C`. This will execute the `operation_on_stop` closure and exit the process.
        /// 2.  Programmatically by setting the `status` field to `StopwatchStatus::Stopped`. This will
        ///     stop the loop and execute the `operation_on_stop` closure.
        ///
        /// # Arguments
        ///
        /// * `writer` - A mutable reference to any type that implements the `std::io::Write`
        ///              trait (e.g., `&mut std::io::Stdout`).
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use your_crate_name::stopwatch::{StopwatchStruct, StopwatchStatus}; // Replace your_crate_name
        /// use std::{io::stdout, thread, time::Duration};
        ///
        /// // This stopwatch will be stopped by another thread after 5 seconds.
        /// let mut stopwatch = StopwatchStruct::new(|time| {
        ///     println!("\nStopwatch finished at {} seconds!", time);
        /// });
        ///
        /// let mut stopwatch_clone = stopwatch.clone();
        /// thread::spawn(move || {
        ///     thread::sleep(Duration::from_secs(5));
        ///     stopwatch_clone.status = StopwatchStatus::Stopped;
        /// });
        ///
        /// stopwatch.start_timer(&mut stdout());
        /// println!("Stopwatch loop ended.");
        /// ```
        pub fn start_stopwatch<W: Write>(&mut self, writer: &mut W) {
            // Share the current time with the Ctrl-C handler using an Arc<AtomicU32>.
            // This is necessary because the handler has a 'static lifetime and needs
            // access to the time, which is being mutated in the loop.
            let shared_time = Arc::new(AtomicU32::new(self.current_time));
            let time_for_handler = shared_time.clone();

            // The operation_on_stop closure has the `Copy` trait, so we can create a
            // copy to move into the 'static Ctrl-C handler.
            let op_on_stop = self.operation_on_stop;

            // Set the Ctrl-C handler. This closure is executed when the user presses Ctrl-C.
            #[cfg(not(target_arch = "wasm32"))]
            ctrlc::set_handler(move || {
                // Load the current elapsed time from the shared atomic variable.
                let final_time = time_for_handler.load(Ordering::SeqCst);
                // Print a newline to avoid the shell prompt overwriting the final time.
                println!();
                // Execute the user-provided closure with the final time.
                (op_on_stop)(final_time);
                // Exit the process.
                process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");

            loop {
                // Check for a programmatic stop condition (e.g., set by `stop_timer`).
                if let StopwatchStatus::Stopped = self.status {
                    break;
                }

                let current_seconds = shared_time.load(Ordering::SeqCst);

                let hours = current_seconds / 3600;
                let minutes = (current_seconds % 3600) / 60;
                let seconds = current_seconds % 60;

                let output_format = format!("{}:{}:{}", hours, minutes, seconds);

                // Write the formatted time. The carriage return `\r` moves the cursor
                // to the beginning of the line, so the next write overwrites the current one.
                write!(writer, "{}\r", output_format).unwrap();
                writer.flush().unwrap();

                thread::sleep(Duration::from_secs(1));

                // Atomically increment the time for thread-safety.
                shared_time.fetch_add(1, Ordering::SeqCst);
            }

            // This block is only reached on a programmatic stop. Ctrl-C exits the process directly.
            // Update the struct's time to the final value from the shared atomic.
            self.current_time = shared_time.load(Ordering::SeqCst);

            // Print a final newline to ensure the shell prompt doesn't overwrite the last display.
            writeln!(writer).unwrap();

            // Execute the on-stop operation.
            (self.operation_on_stop)(self.current_time);
        }
    }
}
