const getFeatured = async () => {
    const response = await fetch("https://steamapi.fractalenjoyer.repl.co/featured")
    const data = await response.json();
    return data;
}

class Game {
    constructor(game) {
        this.raw = game
        this.id = game.id;
        this.name = game.name;
        this.img = game.large_capsule_image;
        this.price = game.final_price.toString()
        if (this.price === "0") {
            this.price = "Free";
        } else {
            this.price = `€${this.price.slice(0, -2)}.${this.price.slice(-2)}`
        }
    }
    async getDetails() {
        const response = await fetch(`https://steamapi.fractalenjoyer.repl.co/getapp?id=${this.id}`);
        const data = await response.json();
        return data[this.id]?.data;
    }
}

class Games {
    constructor(games, length = 9) {
        this.games = Array.from({length: Math.min(games.length, length)}, (x, i) => {
            const game = games[i];
            return new Game(game, game.id, game.name, game.final_price, game.large_capsule_image);
        })
    }

    placeCards(element) {
        this.games.forEach(game => {
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
    card.onclick = (e) => {
        game.getDetails().then(data => {
            console.log(data)
        });
    }
    card.innerHTML = `
        <img src="${game.img}" alt="${game.name}"/>
        <h3>${game.name}</h3>
        <p>${game.price}</p>
    `;
    return card;
}

getFeatured().then(data => {
    console.log(data);
    const featured = new Featured(data);
    featured.specials.placeCards(document.querySelector(".specials"));
    featured.topSellers.placeCards(document.querySelector(".top-sellers"));
    featured.newReleases.placeCards(document.querySelector(".new"));
    featured.comingSoon.placeCards(document.querySelector(".upcoming"));
})