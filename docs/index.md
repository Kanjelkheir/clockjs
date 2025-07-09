# Clock Timer WASM Demo

This demo demonstrates the clock-timer WebAssembly module running in the browser.

## Demo Components

### Timer
- 10-second countdown timer
- Shows remaining time in HH:MM:SS format
- Notifies when the timer completes

Example Code:
```javascript
const timer = new Timer(0, 0, 10); // 10 second timer
timer.start().then(() => {
  console.log('Timer completed!');
});
```

### Stopwatch
- Counts up from zero
- Can be started, stopped, and reset
- Shows elapsed time in HH:MM:SS format

Example Code:
```javascript
const stopwatch = new Stopwatch();
stopwatch.start();
// Later:
const elapsed = stopwatch.stop();
console.log(`Elapsed time: ${elapsed} seconds`);
stopwatch.reset();
```

## WebAssembly Integration

The demo imports the WASM module:

```javascript
// In a real project, you would use:
// import { Timer, Stopwatch } from 'clock-timer';
// For this demo, we're assuming the module is built to ../dist

async function init() {
    try {
        // Dynamically import the wasm module
        const clockTimer = await import('../dist/clock_timer.js');
        const { Timer, Stopwatch } = clockTimer;
        
        // Rest of the implementation...
    } catch (error) {
        console.error("Failed to initialize WASM module:", error);
        // Error handling...
    }
}
```

## User Interface

The demo includes a clean, responsive user interface with:
- Display boxes for both Timer and Stopwatch
- Control buttons (Start, Stop, Reset)
- Real-time updating displays
- Example code snippets for reference

## Building the WASM Module

Make sure you've built the WASM module with wasm-pack:

```bash
cd clock-timer && wasm-pack build --target web --out-dir dist
```
