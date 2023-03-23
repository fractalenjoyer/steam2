const getFeatured = async () => {
    const response = await fetch("https://steamapi.fractalenjoyer.repl.co/featured")
    const data = await response.json();
    return data;
}
getFeatured().then(console.log)
