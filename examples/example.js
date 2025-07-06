// Example usage of clock-timer WebAssembly module

// Import the wasm module
// Note: In a real project, you would use:
// import { Timer, Stopwatch } from 'clock-timer';
// This example assumes you've built the wasm with wasm-pack and have the output in ../dist

async function initTimerExample() {
  // Dynamically import the wasm module
  const { Timer } = await import('../dist/clock_timer.js');

  console.log("Timer Example");

  try {
    // Create a timer for 10 seconds
    const timer = new Timer(0, 0, 10);

    console.log(`Timer created with duration: ${timer.duration} seconds`);
    console.log(`Hours: ${timer.hours}, Minutes: ${timer.minutes}, Seconds: ${timer.seconds}`);

    console.log("Starting timer...");

    // Start the timer
    await timer.start();

    console.log("Timer completed!");
  } catch (error) {
    console.error("Timer error:", error);
  }
}

async function initStopwatchExample() {
  // Dynamically import the wasm module
  const { Stopwatch } = await import('../dist/clock_timer.js');

  console.log("Stopwatch Example");

  // Create a new stopwatch
  const stopwatch = new Stopwatch();

  console.log("Starting stopwatch...");
  stopwatch.start();

  // Run for 5 seconds
  await new Promise(resolve => setTimeout(resolve, 5000));

  // Stop the stopwatch
  const elapsed = stopwatch.stop();
  console.log(`Stopwatch stopped at ${elapsed} seconds`);

  // Reset the stopwatch
  stopwatch.reset();
  console.log("Stopwatch reset");
}

// Run the examples
async function runExamples() {
  await initTimerExample();
  console.log("\n-------------------\n");
  await initStopwatchExample();
}

// This would normally be called when the page loads
// For Node.js environments, you can just call it directly
runExamples().catch(console.error);
