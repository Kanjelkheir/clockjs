# ClockJS - JavaScript Timer & Stopwatch Library

A high-performance timer and stopwatch library built with Rust and compiled to WebAssembly.

## Installation

```bash
npm install clockjs
```

## Usage

### Timer

The Timer provides countdown functionality:

```javascript
import { Timer } from 'clockjs';

// Create a timer for 10 seconds
const timer = new Timer(0, 0, 10);

console.log(`Total duration: ${timer.duration} seconds`);
console.log(`Components: ${timer.hours}h ${timer.minutes}m ${timer.seconds}s`);

// Start the timer - returns a Promise that resolves when timer completes
timer.start().then(() => {
  console.log('Timer completed!');
});
```

### Stopwatch

The Stopwatch provides elapsed time measurement:

```javascript
import { Stopwatch } from 'clockjs';

// Create a new stopwatch
const stopwatch = new Stopwatch();

// Start the stopwatch
stopwatch.start();

// After some time, stop the stopwatch
setTimeout(() => {
  const elapsed = stopwatch.stop();
  console.log(`Elapsed time: ${elapsed} seconds`);
  
  // Reset if needed
  stopwatch.reset();
}, 5000);
```

## Module Formats

### ES Modules (webpack, Rollup, etc.)

```javascript
import { Timer, Stopwatch } from 'clockjs';

// Use Timer and Stopwatch directly
const timer = new Timer(0, 0, 10);
```

### CommonJS (Node.js)

```javascript
const { Timer, Stopwatch } = require('clockjs');

// Use Timer and Stopwatch directly
const timer = new Timer(0, 0, 10);
```

### Browser

```html
<script type="module">
  import { Timer, Stopwatch } from 'https://unpkg.com/clockjs/dist/web/clockjs.js';
  
  // Use Timer and Stopwatch directly
  const timer = new Timer(0, 0, 10);
</script>
```

## API Reference

### Timer

```javascript
// Create a timer for 1 hour, 2 minutes, 30 seconds
const timer = new Timer(1, 2, 30);

// Properties
timer.hours     // => 1
timer.minutes   // => 2
timer.seconds   // => 30
timer.duration  // => 3750 (total seconds)

// Methods
timer.start()   // Returns a Promise that resolves when the timer completes
```

### Stopwatch

```javascript
// Create a stopwatch
const stopwatch = new Stopwatch();

// Properties
stopwatch.current_time  // => Number of seconds elapsed
stopwatch.is_running    // => Boolean indicating if stopwatch is running

// Methods
stopwatch.start()       // Start the stopwatch
stopwatch.stop()        // Stop the stopwatch and return elapsed time
stopwatch.reset()       // Reset the stopwatch to 0
```

## Performance

Built with Rust and WebAssembly for optimal performance in time-sensitive applications.

### Advanced Usage

If you need to ensure the WASM module is fully loaded before using it:

```javascript
import { init, Timer } from 'clockjs';

// You can wait for initialization to complete if needed
await init();

// Then use the Timer and Stopwatch as usual
const timer = new Timer(0, 0, 10);
```

The library automatically initializes in the background, so this is only needed in special cases.

## License

MIT