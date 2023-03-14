const getGames = async () => {
    const response = await fetch("https://SteamAPI.williambreander.repl.co/featured")
    const data = await response.json();
    return data;
}
getGames().then(console.log)