const centerimage = document.querySelector(".center-image");
const focusImage = (image) => {
    i = images.indexOf(image);
    centerimage.src = image
}

centerimage.onclick = (e) => {
    if (e.offsetX < centerimage.width / 2) {
        prevImage();
    } else {
        nextImage();
    }
}

window.onkeydown = (e) => {
    if (e.key === "ArrowRight") {
        nextImage();
    } else if (e.key === "ArrowLeft") {
        prevImage();
    }
}

const images = Array.from(document.querySelectorAll(".images img"), (image) => image.src)

let i = 0;
const nextImage = () => {
    i = (i + 1) % images.length;
    focusImage(images[i]);
}

const prevImage = () => {
    i = (i - 1)
    if (i < 0) 
        i = images.length - 1;
    focusImage(images[i]);
}