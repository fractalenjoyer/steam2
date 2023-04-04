const getFeatured = async () => {
	const response = await fetch(
		"https://steamapi.fractalenjoyer.repl.co/featured"
	);
	const data = await response.json();
	return data;
};

const moreFeatured = async () => {
	const response = await fetch(
		"https://steamapi.fractalenjoyer.repl.co/morefeatured"
	);
	const data = await response.json();
	let games = new Games(
		[...data.featured_win, ...data.featured_mac, ...data.featured_linux],
		30
	);
	return games;
};

class Game {
	constructor(game) {
		this.raw = game;
		this.id = game.id;
		this.name = game.name;
		this.img = game.large_capsule_image;
		this.price = game.final_price.toString();
		if (this.price === "0") {
			this.price = "Free";
		} else {
			this.price = `€${this.price.slice(0, -2)}.${this.price.slice(-2)}`;
		}
	}
	async getDetails() {
		if (this.details) return this.details;
		const response = await fetch(
			`https://steamapi.fractalenjoyer.repl.co/getapp?id=${this.id}`
		);
		const data = await response.json();
		this.details = data[this.id]?.data;
		return this.details;
	}
}

class Games {
	constructor(games, length = 9) {
		this.games = Array.from(
			{ length: Math.min(games.length, length) },
			(_, i) => {
				const game = games[i];
				return new Game(
					game,
					game.id,
					game.name,
					game.final_price,
					game.large_capsule_image
				);
			}
		);
	}

	placeCards(element) {
		this.games.forEach((game) => {
			element?.appendChild(createCard(game));
		});
	}
}

class Featured {
	constructor(data) {
		this.specials = new Games(data.specials.items);
		this.topSellers = new Games(data.top_sellers.items);
		this.newReleases = new Games(data.new_releases.items);
		this.comingSoon = new Games(data.coming_soon.items);
	}
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

getFeatured().then((data) => {
	const featured = new Featured(data);
	featured.specials.placeCards(document.querySelector(".specials"));
	featured.topSellers.placeCards(document.querySelector(".top-sellers"));
	featured.newReleases.placeCards(document.querySelector(".new"));
	featured.comingSoon.placeCards(document.querySelector(".upcoming"));
});

const modal = document.querySelector(".modal");
const modalContent = document.querySelector(".modal-content");

const showGame = (game) => {
	game.getDetails().then((data) => {
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

const search = document.querySelector("#search");
search.onkeyup = (e) => {
	if (e.key === "Enter") {
		window.location = `pages/search.html?q=${e.target.value}`;
	}
}

// moreFeatured().then((games) => {
//     games.placeCards(document.querySelector(".specials"))
// })