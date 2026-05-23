console.log("RustTune loaded");


  
function toggleLike(btn) {
  const liked = btn.classList.toggle("song-card-grid__like--liked");
  btn.textContent = liked ? "♥" : "♡";
  btn.setAttribute("aria-pressed", String(liked));
  const title = btn.closest(".song-card-grid").querySelector(".song-card-grid__title").textContent;
  btn.setAttribute("aria-label", `${liked ? "Unlike" : "Like"} ${title}`);
}

function playSong(btn) {
  const card = btn.closest(".song-card-grid");
  document.querySelectorAll(".song-card-grid--playing")
    .forEach(c => c.classList.remove("song-card-grid--playing"));
  card.classList.add("song-card-grid--playing");
  document.getElementById("now-title").textContent = card.querySelector(".song-card-grid__title").textContent;
  document.getElementById("now-artist").textContent = card.querySelector(".song-card-grid__genre").textContent + " • RustTune";
}

document.querySelectorAll(".song-card-grid__like").forEach(btn => {
  btn.addEventListener("click", () => toggleLike(btn));
});

document.querySelectorAll(".song-card-grid__play-btn").forEach(btn => {
  btn.addEventListener("click", () => playSong(btn));
});