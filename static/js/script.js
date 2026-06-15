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

// Audio Player
const audio   = document.getElementById('player');
const playBtn = document.getElementById('btn-play');
const prevBtn = document.getElementById('btn-prev');
const nextBtn = document.getElementById('btn-next');

let songs      = [];
let currentIdx = 0;

async function loadSongs() {
  const res = await fetch('/api/songs');
  songs     = await res.json();
  if (songs.length > 0) setSong(0);
}

function setSong(idx) {
  currentIdx = idx;
  audio.src  = songs[idx].url;
  audio.load();
}

playBtn.addEventListener('click', () => {
  audio.paused ? audio.play() : audio.pause();
});

audio.addEventListener('play',  () => { playBtn.textContent = '⏸'; });
audio.addEventListener('pause', () => { playBtn.textContent = '▶'; });

prevBtn.addEventListener('click', () => {
  setSong((currentIdx - 1 + songs.length) % songs.length);
  audio.play();
});

nextBtn.addEventListener('click', () => goNext());
audio.addEventListener('ended',   () => goNext());

function goNext() {
  setSong((currentIdx + 1) % songs.length);
  audio.play();
}

loadSongs();