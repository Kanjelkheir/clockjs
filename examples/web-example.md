# rust-ticker Web Example

This example demonstrates how to use the `rust-ticker` library in a web browser.

## Timer Example

A 5-second countdown timer that displays the remaining time in HH:MM:SS format.

### Features:
- Start button to begin the countdown
- Reset button to reset the timer to its initial state
- Visual display of the remaining time

## Stopwatch Example

A simple stopwatch that counts up from zero.

### Features:
- Start button to begin timing
- Stop button to pause the stopwatch
- Reset button to reset the stopwatch to zero
- Visual display of the elapsed time

## Code Example

```javascript
// Import the Timer and Stopwatch classes
import { Timer, Stopwatch } from 'rust-ticker';

// Create a timer for 5 seconds
const timer = new Timer(0, 0, 5);

// Start the timer
timer.start().then(() => {
    console.log('Timer completed!');
});

// Create a stopwatch
const stopwatch = new Stopwatch();

// Start the stopwatch
stopwatch.start();

// Stop the stopwatch after some time
setTimeout(() => {
    const elapsed = stopwatch.stop();
    console.log(`Elapsed time: ${elapsed} seconds`);

    // Reset the stopwatch
    stopwatch.reset();
}, 3000);
```

## Implementation Details

The web example uses ES modules to import the `rust-ticker` library:

```javascript
import { Timer, Stopwatch } from '../dist/web/clockjs.js';
```

### Timer Implementation

The timer is initialized with a duration of 5 seconds and updates the display every second as it counts down.

### Stopwatch Implementation

The stopwatch starts from zero and updates the display every 100ms to show the elapsed time.

## Styling

The example includes a clean, responsive user interface with:
- Container elements for each demo component
- Clear visual separation between demos
- Consistent button styling
- Monospace font for time displays