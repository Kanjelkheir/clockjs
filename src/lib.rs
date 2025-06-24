//! # Rust Time Utility Library
//!
//! This library provides two main utilities for time management:
//! a `TimerStruct` for countdown timers and a `StopwatchStruct` for measuring elapsed time.


/// Module for countdown timer functionalities.
pub mod timer {
    use std::{io::Write, thread, time::Duration};

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

    impl TimerStruct {
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
        pub fn new(hours: u32, minutes: u32, seconds: u32) -> Result<TimerStruct, &'static str> {
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
        /// use your_crate_name::TimerStruct; // Replace your_crate_name with your actual crate name
        /// use std::io::{self, stdout};
        ///
        /// let timer = TimerStruct::new(0, 0, 5).unwrap(); // 5-second timer
        /// let mut writer = stdout();
        /// timer.start_timer(&mut writer);
        /// println!("Timer finished!");
        /// ```
        pub fn start_timer<W: Write>(&self, writer: &mut W) {
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
    use std::{io::Write,thread, time::Duration};

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
    where T: Fn(u32),
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
    where T: Fn(u32),
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
        /// use your_crate_name::stopwatch::{StopwatchStruct, StopwatchStatus}; // Replace your_crate_name
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
        /// to the provided writer, overwriting the previous line. The loop continues indefinitely
        /// until the `status` field of the `StopwatchStruct` is manually set to `StopwatchStatus::Stopped`
        /// from outside this function (e.g., from another thread or an external signal).
        ///
        /// # Arguments
        ///
        /// * `writer` - A mutable reference to any type that implements the `std::io::Write`
        ///              trait (e.g., `&mut std::io::Stdout`).
        ///
        /// # Examples
        ///
        /// ```
        /// use your_crate_name::stopwatch::{StopwatchStruct, StopwatchStatus}; // Replace your_crate_name
        /// use std::{io::stdout, thread, time::Duration};
        ///
        /// let mut stopwatch = StopwatchStruct::new(|time| {
        ///     println!("\nStopwatch finished at {} seconds!", time);
        /// });
        ///
        /// let mut writer = stdout();
        ///
        /// // To stop the stopwatch, you would typically change its status from another thread
        /// // or based on some external event. For demonstration, we'll do it after a delay.
        /// let mut stopwatch_clone = stopwatch.clone(); // Clone to move into another thread
        /// thread::spawn(move || {
        ///     thread::sleep(Duration::from_secs(5));
        ///     stopwatch_clone.status = StopwatchStatus::Stopped; // Stop after 5 seconds
        /// });
        ///
        /// stopwatch.start_timer(&mut writer);
        /// println!("Stopwatch loop ended.");
        /// ```
        pub fn start_timer<W: Write>(&mut self, writer: &mut W) {
            loop {
                match self.status {
                    StopwatchStatus::Stopped => {
                        (self.operation_on_stop)(self.current_time);
                        break;
                    },
                    StopwatchStatus::Running => (),
                }
 
                let hours = self.current_time / 3600;
                let mut remaining = self.current_time % 3600;
                let minutes = remaining / 60;
                remaining %= 60;
                
                let output_format = format!("{}:{}:{}", hours, minutes, remaining);
       
                if self.current_time == 0 {
                    write!(writer, "{}", output_format).unwrap();
                }

                thread::sleep(Duration::from_secs(1));

                self.current_time += 1;

                write!(writer, "{}\r", output_format).unwrap();

                writer.flush().unwrap();
            }    
        }

        pub fn stop_timer<T>(seconds: u32, reference: &mut StopwatchStruct<T>)
        where T: Fn(u32), 
        {
            let duration = Duration::from_secs(seconds);
            reference.status = StopwatchStatus::Stopped; 
        }
    }
}
