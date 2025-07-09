// Node.js example for rust-ticker
import { Timer, Stopwatch } from "rust-ticker";

// Create an async function to use await with our timer
async function runExamples() {
  try {
    // Example 1: Using Timer
    console.log("Example 1: Creating a 5-second timer");
    const timer = new Timer(0, 0, 5); // 5 seconds
    console.log("Starting timer...");
    await timer.start();
    console.log("Timer completed!");

    // Example 2: Using Stopwatch
    console.log("\nExample 2: Using a stopwatch");
    const stopwatch = new Stopwatch();

    // Start the stopwatch
    console.log("Starting stopwatch...");
    await stopwatch.start();

    // Wait for 3 seconds
    console.log("Waiting for 3 seconds...");
    await new Promise((resolve) => setTimeout(resolve, 3000));

    // Stop the stopwatch
    const elapsed = await stopwatch.stop();
    console.log(`Stopwatch stopped at ${elapsed} seconds`);

    // Reset the stopwatch
    console.log("Resetting stopwatch...");
    await stopwatch.reset();

    console.log("\nExamples completed!");
  } catch (error) {
    console.error("Error running examples:", error);
  }
}

// Run the examples
runExamples();
