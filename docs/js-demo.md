# Rust Ticker JS Demo

This demo demonstrates the rust-ticker functionality implemented in JavaScript.

## Demo Components

### Timer
- 10-second countdown timer
- Shows remaining time in HH:MM:SS format
- Notifies when the timer completes

### Stopwatch
- Counts up from zero
- Can be started, stopped, and reset
- Shows elapsed time in HH:MM:SS format

## JavaScript Implementation

The demo uses vanilla JavaScript to implement both Timer and Stopwatch functionality:

### Timer Class
```javascript
class Timer {
    constructor(hours, minutes, seconds) {
        this.hours = hours;
        this.minutes = minutes;
        this.seconds = seconds;
        this.duration = hours * 3600 + minutes * 60 + seconds;
    }

    start() {
        return new Promise((resolve) => {
            let remaining = this.duration;
            const interval = setInterval(() => {
                remaining--;
                console.log(
                    `Timer: ${Math.floor(remaining / 3600)}:${Math.floor((remaining % 3600) / 60)}:${remaining % 60}`,
                );
                if (remaining <= 0) {
                    clearInterval(interval);
                    resolve();
                }
            }, 1000);
        });
    }
}
```

### Stopwatch Class
```javascript
class Stopwatch {
    constructor() {
        this.current_time = 0;
        this.is_running = false;
        this.interval = null;
    }

    start() {
        if (this.is_running) return;
        this.is_running = true;
        this.interval = setInterval(() => {
            this.current_time++;
            console.log(
                `Stopwatch: ${Math.floor(this.current_time / 3600)}:${Math.floor((this.current_time % 3600) / 60)}:${this.current_time % 60}`,
            );
        }, 1000);
    }

    stop() {
        if (!this.is_running) return this.current_time;
        this.is_running = false;
        clearInterval(this.interval);
        return this.current_time;
    }

    reset() {
        this.stop();
        this.current_time = 0;
    }
}
```

## Usage Example

```javascript
// Create and use a timer
const timer = new Timer(0, 0, 10); // 10 second timer
timer.start().then(() => {
    console.log('Timer completed!');
});

// Create and use a stopwatch
const stopwatch = new Stopwatch();
stopwatch.start();
// Later:
const elapsed = stopwatch.stop();
console.log(`Elapsed time: ${elapsed} seconds`);
stopwatch.reset();
```

## User Interface

The demo includes a clean, responsive user interface with:
- Display boxes for both Timer and Stopwatch
- Control buttons (Start, Stop, Reset)
- Real-time updating displays
- Console output capture for monitoring events