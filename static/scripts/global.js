darkmode.onclick = e => {
    if (e.target.checked) {
        window.localStorage.setItem("lightmode", "true")
    } else {
        window.localStorage.setItem("lightmode", "false")
    }
}

if (window.localStorage.getItem("lightmode") === "true") {
    darkmode.checked = true
} else {
    darkmode.checked = false
}