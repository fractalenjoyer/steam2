const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

const showGame = (id) => {
    modalContent.src = `/game/${id}`;
    modal.style.display = "flex";
}

modal.onclick = (e) => {
	if (e.target.classList.contains("modal")) {
		document.querySelector(".modal").style.display = "none";
	}
};