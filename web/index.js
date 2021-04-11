import * as rustyShitBox from "rusty-shit-box";

const application = new rustyShitBox.Application();

let time = Date.now();
const render = () => {
    const dt = Date.now() - time;

    application.render();
    window.requestAnimationFrame(render);

    time = Date.now();
}

render();