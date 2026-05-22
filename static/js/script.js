const songCards = document.querySelectorAll(".song-card");

const recentList = document.getElementById("recent-list");

let recentlyPlayed =
  JSON.parse(localStorage.getItem("recentSongs")) || [];

function renderRecentSongs() {

  recentList.innerHTML = "";

  recentlyPlayed.forEach(song => {

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

songCards.forEach(card => {

  card.addEventListener("click", () => {

    const songName = card.dataset.song;

    recentlyPlayed =
      recentlyPlayed.filter(song => song !== songName);

    recentlyPlayed.unshift(songName);

    recentlyPlayed = recentlyPlayed.slice(0, 10);

    localStorage.setItem(
      "recentSongs",
      JSON.stringify(recentlyPlayed)
    );

    renderRecentSongs();

  });

});

renderRecentSongs();