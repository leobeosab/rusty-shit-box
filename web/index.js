import * as rustyShitBox from "rusty-shit-box";

const application = new rustyShitBox.Application();
console.log(application);

let rotation = 0.0;

let time = Date.now();
const render = () => {
    const dt = Date.now() - time;

    application.render(rotation);
    window.requestAnimationFrame(render);

    rotation += 0.0007;

    time = Date.now();
}

render();