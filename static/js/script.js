console.log("RustTune loaded");

document.addEventListener("DOMContentLoaded", () => {
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

  // Recently played songs
  const songCards = document.querySelectorAll(".song-card");

  const recentList = document.getElementById("recent-list");

  let recentlyPlayed = JSON.parse(localStorage.getItem("recentSongs")) || [];

  function renderRecentSongs() {
    recentList.innerHTML = "";

    recentlyPlayed.forEach((song) => {
      const songItem = document.createElement("li");

      songItem.classList.add("recent-card");

      songItem.innerHTML = `
        <div class="song-cover">🎵</div>

        <div class="song-meta">
          <p class="song-title">${song}</p>
          <p class="song-artist">Recently Played</p>
        </div>
      `;

      recentList.appendChild(songItem);
    });
  }

  songCards.forEach((card) => {
    card.addEventListener("click", () => {
      const songName = card.dataset.song;

      recentlyPlayed = recentlyPlayed.filter((song) => song !== songName);

      recentlyPlayed.unshift(songName);

      recentlyPlayed = recentlyPlayed.slice(0, 10);

      localStorage.setItem("recentSongs", JSON.stringify(recentlyPlayed));

      renderRecentSongs();
    });
  });

  renderRecentSongs();
});
