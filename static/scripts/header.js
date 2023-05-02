function showmenu() {
	navbar.style.left = "0px";
    blocker.style.display = "block";
    document.body.style.overflowY = "hidden";
}

function hidemenu() {
    navbar.style.left = "-250px";
    blocker.style.display = "none";
    document.body.style.overflowY = "auto";
}

blocker.onclick = hidemenu;
