const cache = {};

const showGame = (id) => {
    if (cache[id]) {
        launchModal(cache[id]);
        return;
    }
    fetch(`/api/appdetails/${id}`).then((data) => {
        data.json().then((game) => {
            cache[id] = game[id]?.data;
            launchModal(game[id]?.data)
        })
    })
}

const launchModal = (game) => {
    modal.style.display = "flex";
    console.log(game);

    let div = `<img src="${game.header_image}" alt="${game.name}"/>`;

    game.screenshots.forEach((screenshot) => {
        div += `<img src="${screenshot.path_full}" alt="${game.name}" style="width: 40vw"/>`;
    });

    div += `
    <h3>${game.name}</h3>
    <p>${game.is_free ? "Free" : game.price}</p>
    <p>${game.about_the_game}</p>`;
    modalContent.innerHTML = div;
}

const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

modal.onclick = (e) => {
	if (e.target.classList.contains("modal")) {
		document.querySelector(".modal").style.display = "none";
	}
};