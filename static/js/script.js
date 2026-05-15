// RustTune music player
(() => {
  const audio = document.getElementById("audio");
  const playBtn = document.getElementById("play-btn");
  const prevBtn = document.getElementById("prev-btn");
  const nextBtn = document.getElementById("next-btn");
  const progressBar = document.getElementById("progress-bar");
  const progressFill = document.getElementById("progress-fill");
  const currentTimeEl = document.getElementById("current-time");
  const durationEl = document.getElementById("duration");
  const trackTitleEl = document.getElementById("track-title");
  const trackArtistEl = document.getElementById("track-artist");
  const songCards = Array.from(document.querySelectorAll("#song-list .song-card"));

  let currentIndex = -1;
  let isSeeking = false;

  const fmt = (s) => {
    if (!isFinite(s) || s < 0) return "0:00";
    const m = Math.floor(s / 60);
    const sec = Math.floor(s % 60).toString().padStart(2, "0");
    return `${m}:${sec}`;
  };

  const setPlayIcon = (playing) => {
    playBtn.textContent = playing ? "⏸" : "▶";
    playBtn.setAttribute("aria-label", playing ? "Pause" : "Play");
  };

  const highlightActive = () => {
    songCards.forEach((c, i) => c.classList.toggle("active", i === currentIndex));
  };

  const loadTrack = (index, autoplay = true) => {
    if (index < 0 || index >= songCards.length) return;
    const card = songCards[index];
    currentIndex = index;
    audio.src = card.dataset.src;
    trackTitleEl.textContent = card.dataset.title || "Unknown";
    trackArtistEl.textContent = card.dataset.artist || "Unknown";
    highlightActive();
    if (autoplay) audio.play().catch((e) => console.warn("Playback blocked:", e));
  };

  const togglePlay = () => {
    if (currentIndex === -1) {
      loadTrack(0, true);
      return;
    }
    if (audio.paused) audio.play();
    else audio.pause();
  };

  // Controls
  playBtn.addEventListener("click", togglePlay);
  prevBtn.addEventListener("click", () => {
    if (songCards.length === 0) return;
    const i = currentIndex <= 0 ? songCards.length - 1 : currentIndex - 1;
    loadTrack(i, true);
  });
  nextBtn.addEventListener("click", () => {
    if (songCards.length === 0) return;
    const i = (currentIndex + 1) % songCards.length;
    loadTrack(i, true);
  });

  // Song list selection
  songCards.forEach((card, i) => {
    card.addEventListener("click", () => {
      if (i === currentIndex) togglePlay();
      else loadTrack(i, true);
    });
  });

  // Audio events
  audio.addEventListener("play", () => setPlayIcon(true));
  audio.addEventListener("pause", () => setPlayIcon(false));
  audio.addEventListener("ended", () => {
    if (songCards.length === 0) return;
    loadTrack((currentIndex + 1) % songCards.length, true);
  });
  audio.addEventListener("loadedmetadata", () => {
    durationEl.textContent = fmt(audio.duration);
  });
  audio.addEventListener("timeupdate", () => {
    if (isSeeking || !isFinite(audio.duration)) return;
    const pct = (audio.currentTime / audio.duration) * 100;
    progressFill.style.width = `${pct}%`;
    currentTimeEl.textContent = fmt(audio.currentTime);
    progressBar.setAttribute("aria-valuenow", String(Math.floor(pct)));
  });

  // Progress bar seek (click + drag)
  const seekFromEvent = (e) => {
    const rect = progressBar.getBoundingClientRect();
    const x = (e.touches ? e.touches[0].clientX : e.clientX) - rect.left;
    const ratio = Math.max(0, Math.min(1, x / rect.width));
    progressFill.style.width = `${ratio * 100}%`;
    if (isFinite(audio.duration)) {
      currentTimeEl.textContent = fmt(ratio * audio.duration);
      if (!isSeeking) audio.currentTime = ratio * audio.duration;
      else audio._pendingSeek = ratio * audio.duration;
    }
  };

  progressBar.addEventListener("mousedown", (e) => {
    isSeeking = true;
    seekFromEvent(e);
  });
  document.addEventListener("mousemove", (e) => {
    if (isSeeking) seekFromEvent(e);
  });
  document.addEventListener("mouseup", () => {
    if (!isSeeking) return;
    isSeeking = false;
    if (audio._pendingSeek != null) {
      audio.currentTime = audio._pendingSeek;
      audio._pendingSeek = null;
    }
  });

  // Keyboard support on progress bar
  progressBar.addEventListener("keydown", (e) => {
    if (!isFinite(audio.duration)) return;
    if (e.key === "ArrowRight") {
      e.preventDefault();
      audio.currentTime = Math.min(audio.duration, audio.currentTime + 5);
    } else if (e.key === "ArrowLeft") {
      e.preventDefault();
      audio.currentTime = Math.max(0, audio.currentTime - 5);
    }
  });

  // Spacebar toggles play/pause when not in an input
  document.addEventListener("keydown", (e) => {
    if (e.code !== "Space") return;
    const t = e.target;
    if (t && (t.tagName === "INPUT" || t.tagName === "TEXTAREA" || t.isContentEditable)) return;
    e.preventDefault();
    togglePlay();
  });

  console.log("RustTune player ready");
})();