import * as playermod from "./player/mod.js";

export function init() {
	e2.canvas = document.getElementById("main-canvas");
	e2.canvas.width = window.innerWidth;
	e2.canvas.height = window.innerHeight;

	playermod.init();
	// tilemap.init();
}

export function draw() {
	playermod.draw();
	// tilemap.draw();
}
