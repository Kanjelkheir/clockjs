pub mod timer {

    use std::{io::Write, thread, time::Duration};


    #[derive(Clone, Copy, Debug)]

    pub struct TimerStruct {

        pub duration: u32,

        pub hours: u32,

        pub minutes: u32,

        pub seconds: u32,

    }


    impl TimerStruct {

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


pub use timer::TimerStruct;

pub mod stopwatch {
    use std::{io::Write,thread, time::Duration};

    #[derive(Clone, Debug)]
    pub enum StopwatchStatus {
        Stopped,
        Running,
    }

    #[derive(Debug, Clone)]
    pub struct StopwatchStruct<T>
    where T: Fn(u32),
    {
        pub current_time: u32,
        pub status: StopwatchStatus,
        pub operation_on_stop: T,
    }

    impl<T> StopwatchStruct<T>
    where T: Fn(u32),
    {
        pub fn new(operation_on_stop: T) -> StopwatchStruct<T> {
            StopwatchStruct {
                current_time: 0,
                status: StopwatchStatus::Running,
                operation_on_stop,
            }
        }

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


            }    
        }
    }
}
