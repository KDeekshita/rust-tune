console.log("RustTune loaded");

const spinner = document.getElementById("spinner");
const errorBanner = document.getElementById("error-banner");
const errorMessage = document.getElementById("error-message");

let lastAction = null;

function showSpinner() {
  spinner.classList.remove("hidden");
}

function hideSpinner() {
  spinner.classList.add("hidden");
}

function showError(message) {
  errorMessage.textContent = message;
  errorBanner.classList.remove("hidden");
}

function hideError() {
  errorBanner.classList.add("hidden");
}

async function withFeedback(action) {
  lastAction = action;
  hideError();
  showSpinner();
  try {
    await action();
  } catch (err) {
    showError(err.message || "Something went wrong. Please try again.");
  } finally {
    hideSpinner();
  }
}

function retryLastAction() {
  if (lastAction) {
    withFeedback(lastAction);
  }
}

async function handlePlay() {
  await withFeedback(async () => {
    // Simulate async music load — replace with real fetch when backend is ready
    await new Promise((resolve) => {
      setTimeout(() => {
        // Uncomment to test error state:
        // reject(new Error("Failed to load track. Check your connection."));
        resolve();
      }, 1500);
    });
    console.log("Playback started");
  });
}
