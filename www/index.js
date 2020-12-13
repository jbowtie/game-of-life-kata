import {Grid} from "game-of-life-kata";

const pre = document.getElementById("game-of-life-canvas");
const grid = Grid.new_sample();

const renderLoop = () => {
    // uncomment this to pause between frames
    //debugger;
    pre.textContent = grid.render();
    grid.tick();
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
