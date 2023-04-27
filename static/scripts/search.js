const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

const showGame = (id) => {
    modalContent.src = `/game/${id}`;
    modal.style.display = "flex";
}

// would rather replace this with in-html event listener for clarity
modal.onclick = (e) => {
	e.target.style.display = "none";
};
