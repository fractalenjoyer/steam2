const search = async (term) => {
    let response = await fetch(`https://steamapi.fractalenjoyer.repl.co/search?q=${term}`);
    let data = await response.json();
    return data;
}

const getDetails = async (game) => {
    const response = await fetch(
        `https://steamapi.fractalenjoyer.repl.co/getapp?id=${game.id}`
    );
    const data = await response.json();
    game.details = data[game.id]?.data;
    return game.details;
}

const createCard = (game) => {
	const card = document.createElement("div");
	card.classList.add("card");
	card.onclick = (e) => showGame(game);
	card.innerHTML = `
        <img src="${game.img}" alt="${game.name}"/>
        <h3>${game.name}</h3>
        <p>${game.price}</p>
    `;
	return card;
};

const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

const showGame = (game) => {
	getDetails(game).then((data) => {
		console.log(data);
		modal.style.display = "flex";

		let div = `<img src="${data.header_image}" alt="${data.name}"/>`;

		data.screenshots.forEach((screenshot) => {
			div += `<img src="${screenshot.path_full}" alt="${game.name}" style="width: 40vw"/>`;
		});

		div += `
        <h3>${data.name}</h3>
        <p>${data.is_free ? "Free" : game.price}</p>
        <p>${data.about_the_game}</p>`;
        modalContent.innerHTML = div;
	});
};

modal.onclick = (e) => {
	if (e.target.classList.contains("modal")) {
		document.querySelector(".modal").style.display = "none";
	}
};

const searchTerm = window.location.search.match(/(?<=\?q\=).*/)[0];
const results = document.querySelector(".results");
search(searchTerm).then((data) => {
    data.forEach((game) => {
        results.appendChild(createCard(game));
    });
});