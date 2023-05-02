darkmode.onclick = e => {
    if (e.target.checked) {
        window.localStorage.setItem("dark", "false")
    } else {
        window.localStorage.setItem("dark", "true")
    }
}

if (window.localStorage.getItem("dark") !== "true") {
    darkmode.checked = true
} else {
    darkmode.checked = false
}