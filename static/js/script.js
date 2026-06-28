console.log("RustTune loaded");

// Theme toggle functionality
const initTheme = () => {
  const themeToggle = document.getElementById("theme-toggle");
  const html = document.documentElement;
  
  // Check for saved theme preference or default to dark mode
  const savedTheme = localStorage.getItem("theme") || "dark";
  
  // Apply saved theme on load
  if (savedTheme === "light") {
    html.classList.add("light-mode");
    themeToggle.textContent = "☀️";
  } else {
    html.classList.remove("light-mode");
    themeToggle.textContent = "🌙";
  }
  
  // Toggle theme on button click
  themeToggle.addEventListener("click", () => {
    const isLightMode = html.classList.toggle("light-mode");
    const newTheme = isLightMode ? "light" : "dark";
    localStorage.setItem("theme", newTheme);
    themeToggle.textContent = isLightMode ? "☀️" : "🌙";
    console.log(`Theme switched to ${newTheme} mode`);
  });
};

document.addEventListener("DOMContentLoaded", () => {
  initTheme();
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