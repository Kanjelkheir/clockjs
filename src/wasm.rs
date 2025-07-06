use crate::timer::{TimerStruct, TimerTrait};

use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    console_error_panic_hook::set_once();
}

// Create a wrapper for TimerStruct that can be used in JavaScript
#[wasm_bindgen]
pub struct Timer {
    inner: TimerStruct,
}

#[wasm_bindgen]
impl Timer {
    /// Creates a new Timer instance
    #[wasm_bindgen(constructor)]
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Result<Timer, JsValue> {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        match TimerStruct::new(hours, minutes, seconds) {
            Ok(timer) => Ok(Timer { inner: timer }),
            Err(e) => Err(JsValue::from_str(e)),
        }
    }

    /// Gets the total duration in seconds
    #[wasm_bindgen(getter)]
    pub fn duration(&self) -> u32 {
        self.inner.duration
    }

    /// Gets the hours component
    #[wasm_bindgen(getter)]
    pub fn hours(&self) -> u32 {
        self.inner.hours
    }

    /// Gets the minutes component
    #[wasm_bindgen(getter)]
    pub fn minutes(&self) -> u32 {
        self.inner.minutes
    }

    /// Gets the seconds component
    #[wasm_bindgen(getter)]
    pub fn seconds(&self) -> u32 {
        self.inner.seconds
    }

    /// Starts the timer and returns a Promise that resolves when the timer completes
    pub fn start(&self) -> js_sys::Promise {
        let duration = self.inner.duration;

        // Create a Promise that will resolve when the timer completes
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let window = web_sys::window().expect("should have a window in this context");

            // Clone necessary values for the closure
            let resolve_fn = resolve.clone();

            // Create a recursive setTimeout function to handle the countdown
            fn create_timeout(
                window: &web_sys::Window,
                remaining: u32,
                callback: &js_sys::Function,
                resolve_fn: &js_sys::Function,
            ) {
                if remaining == 0 {
                    // Timer completed, resolve the promise with the final time
                    let _ = resolve_fn.call0(&JsValue::NULL);
                    return;
                }

                // Calculate display components
                let hours = remaining / 3600;
                let minutes = (remaining % 3600) / 60;
                let seconds = remaining % 60;

                // Log current time to console
                web_sys::console::log_1(&JsValue::from_str(&format!(
                    "Timer: {}:{}:{}",
                    hours, minutes, seconds
                )));

                // Create closure for the next timeout
                let window_clone = window.clone();
                let callback_clone = callback.clone();
                let resolve_clone = resolve_fn.clone();
                let next_remaining = remaining - 1;

                let next_callback = Closure::once_into_js(move || {
                    create_timeout(
                        &window_clone,
                        next_remaining,
                        &callback_clone,
                        &resolve_clone,
                    );
                });

                // Set timeout for 1 second
                let _ = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        next_callback.as_ref().unchecked_ref(),
                        1000,
                    )
                    .expect("failed to set timeout");
            }

            // Start the timeout chain
            let callback = js_sys::Function::new_no_args("");
            create_timeout(&window, duration, &callback, &resolve_fn);
        });

        promise
    }
}

// Create a wrapper for StopwatchStruct that can be used in JavaScript
#[wasm_bindgen]
pub struct Stopwatch {
    current_time: u32,
    running: bool,
    interval_id: Option<i32>,
}

#[wasm_bindgen]
impl Stopwatch {
    /// Creates a new Stopwatch instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Stopwatch {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        Stopwatch {
            current_time: 0,
            running: false,
            interval_id: None,
        }
    }

    /// Gets the current elapsed time in seconds
    #[wasm_bindgen(getter)]
    pub fn current_time(&self) -> u32 {
        self.current_time
    }

    /// Checks if the stopwatch is currently running
    #[wasm_bindgen(getter)]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Starts the stopwatch
    pub fn start(&mut self) -> Result<(), JsValue> {
        if self.running {
            return Ok(());
        }

        self.running = true;
        let window = web_sys::window().expect("should have a window in this context");

        // Create closure for the interval
        let closure = {
            let mut time = self.current_time;

            Closure::wrap(Box::new(move || {
                time += 1;

                // Calculate display components
                let hours = time / 3600;
                let minutes = (time % 3600) / 60;
                let seconds = time % 60;

                // Log current time to console
                web_sys::console::log_1(&JsValue::from_str(&format!(
                    "Stopwatch: {}:{}:{}",
                    hours, minutes, seconds
                )));
            }) as Box<dyn FnMut()>)
        };

        // Set interval for 1 second
        let interval_id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                1000,
            )
            .expect("failed to set interval");

        // Store the interval ID so we can clear it later
        self.interval_id = Some(interval_id);

        // Forget the closure so it remains valid for the interval
        closure.forget();

        Ok(())
    }

    /// Stops the stopwatch and returns the elapsed time
    pub fn stop(&mut self) -> u32 {
        if !self.running {
            return self.current_time;
        }

        self.running = false;

        // Clear the interval if it exists
        if let Some(interval_id) = self.interval_id.take() {
            let window = web_sys::window().expect("should have a window in this context");
            window.clear_interval_with_handle(interval_id);
        }

        self.current_time
    }

    /// Resets the stopwatch to zero
    pub fn reset(&mut self) {
        self.stop();
        self.current_time = 0;
    }
}
