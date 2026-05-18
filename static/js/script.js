console.log("RustTune loaded");

const loading = document.getElementById("loading");
const error = document.getElementById("error");

window.showLoading = function () {
    loading.style.display = "block";
    error.style.display = "none";

    setTimeout(() => {
        loading.style.display = "none";
    }, 2000);
};

window.retryMusic = function () {
    error.style.display = "none";
    alert("Retrying...");
};