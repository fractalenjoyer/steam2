const search = async (term) => {
    let response = await fetch(`https://steamapi.fractalenjoyer.repl.co/search?q=${term}`);
    let data = await response.json();
    return data;
}

const searchTerm = window.location.search.match(/(?<=\?q\=).*/)[0];
const results = document.querySelector(".results");
search(searchTerm).then((data) => {
    results.innerText = JSON.stringify(data, null, 2)
});