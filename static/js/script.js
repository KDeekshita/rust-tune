console.log("RustTune loaded");
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