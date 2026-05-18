const playBtn = document.getElementById("playBtn");
const loading = document.getElementById("loading");
const errorBox = document.getElementById("error");
const retryBtn = document.getElementById("retryBtn");

function simulateRequest() {
  loading.classList.remove("hidden");
  errorBox.classList.add("hidden");

  setTimeout(() => {
    loading.classList.add("hidden");

    const success = Math.random() > 0.5;

    if (!success) {
      errorBox.classList.remove("hidden");
    } else {
      alert("Music Played Successfully 🎵");
    }
  }, 2000);
}

playBtn.addEventListener("click", simulateRequest);

retryBtn.addEventListener("click", simulateRequest);