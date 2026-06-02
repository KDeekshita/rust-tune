console.log("RustTune loaded");

/**
 * Format an integer number of seconds into a "M:SS" display string.
 * The backend returns duration_secs as a plain number (e.g. 373);
 * this helper converts it to "6:13" for display in the UI.
 *
 * @param {number} secs - Total duration in seconds
 * @returns {string} Formatted duration, e.g. "6:13"
 */
function formatDuration(secs) {
  const m = Math.floor(secs / 60);
  const s = String(secs % 60).padStart(2, "0");
  return `${m}:${s}`;
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

  // Populate song list from /api/songs.
  // duration_secs is a plain integer from the API; formatDuration renders it as "M:SS".
  const songList = document.querySelector(".song-list");
  if (songList) {
    fetch("/api/songs")
      .then((res) => res.json())
      .then((songs) => {
        songList.innerHTML = songs
          .map(
            (song) => `
          <li class="song-card">
            <div class="song-cover">🎵</div>
            <div class="song-meta">
              <p class="song-title">${song.title}</p>
              <p class="song-artist">${song.artist}</p>
            </div>
            <span class="song-duration">${formatDuration(song.duration_secs)}</span>
          </li>`
          )
          .join("");
      })
      .catch((err) => console.error("Failed to load songs:", err));
  }
});