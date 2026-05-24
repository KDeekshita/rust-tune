console.log("RustTune loaded");

/**
 * Triggers and displays the global error component
 * @param {string} message - The custom error message text to show the user
 * @param {function} retryCallback - Optional function to run if they click "Try Again"
 */
function showErrorMessage(message, retryCallback = null) {
    const container = document.getElementById('error-container');
    const textElement = document.getElementById('error-message-text');
    const retryBtn = document.getElementById('error-retry-btn');

    if (!container || !textElement || !retryBtn) return;

    textElement.textContent = message;

    // Configure retry button action
    if (retryCallback && typeof retryCallback === 'function') {
        retryBtn.style.display = 'inline-block';

        const newRetryBtn = retryBtn.cloneNode(true);
        retryBtn.parentNode.replaceChild(newRetryBtn, retryBtn);
        
        newRetryBtn.addEventListener('click', () => {
            hideErrorMessage();
            retryCallback();
        });
    } else {
        retryBtn.style.display = 'none';
    }
    container.classList.remove('hidden');
}

function hideErrorMessage() {
    const container = document.getElementById('error-container');
    if (container) {
        container.classList.add('hidden');
    }
}

document.addEventListener("DOMContentLoaded", () => {
  console.log("RustTune volume controller initialized");

  const muteBtn = document.getElementById("mute-btn");
  const volumeSlider = document.getElementById("volume-slider");

  let savedVolume = parseFloat(volumeSlider.value) || 0.8;

  function updateIcon(volumeValue) {
    if (volumeValue == 0) {
      muteBtn.textContent = "🔇";
    } else if (volumeValue >= 0.5) {
      muteBtn.textContent = "🔊";
    } else {
      muteBtn.textContent = "🔉";
    }
  }

  volumeSlider.addEventListener("input", (e) => {
    const val = parseFloat(e.target.value);
    
    if (val > 0) {
      savedVolume = val;
    }
    
    updateIcon(val);
  });

  muteBtn.addEventListener("click", (e) => {
    e.stopPropagation(); 

    const currentVal = parseFloat(volumeSlider.value);

    if (currentVal > 0) {
      savedVolume = currentVal;
      volumeSlider.value = 0;
      updateIcon(0);
    } else {
      volumeSlider.value = savedVolume;
      updateIcon(savedVolume);
    }
  });
});