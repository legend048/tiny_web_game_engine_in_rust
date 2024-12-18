import init, { key_pressed, update_speed, get_fps } from './out/game_engine.js';

const canvas = document.getElementById("my_canvas");
const gl = canvas.getContext("webgl");

if (!gl) {
    console.error("WebGL is not supported");
} else {
    const vertexShaderSource = `
        attribute vec2 a_position; // Input vertex positions
        void main() {
            gl_Position = vec4(a_position, 0, 1); // Map to clip space
        }
    `;

    const fragmentShaderSource = `
        precision mediump float;
        uniform vec4 u_color; // Input color
        void main() {
            gl_FragColor = u_color; // Set fragment color
        }
    `;

    const createShader = (type, source) => {
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);

        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            console.error("Shader compilation error:", gl.getShaderInfoLog(shader));
            gl.deleteShader(shader);
            return null;
        }
        return shader;
    };

    const vertexShader = createShader(gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = createShader(gl.FRAGMENT_SHADER, fragmentShaderSource);

    const program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        console.error("Program linking error:", gl.getProgramInfoLog(program));
        gl.deleteProgram(program);
    } else {
        gl.program = program;
        gl.useProgram(program);

        const resolutionUniformLocation = gl.getUniformLocation(program, "u_resolution");
        gl.uniform2f(resolutionUniformLocation, canvas.width, canvas.height);
    }
}


window.addEventListener("keydown", (event) => {
    let keyCode = 0;
    switch (event.key) {
        case "ArrowUp": keyCode = 3; break;
        case "ArrowDown": keyCode = 4; break;
        case "ArrowLeft": keyCode = 1; break;
        case "ArrowRight": keyCode = 2; break;
        case " ": keyCode = 5; break;
    }

    if (keyCode !== 0) {
        key_pressed(keyCode);
    }
});

function change_screen_color(red, green, blue, alpha) {
    const canvas = document.getElementById("my_canvas");
    const gl = canvas.getContext("webgl");

    if (!gl) {
        console.error("WebGL is not supported");
        return;
    }
    gl.clearColor(red, green, blue, alpha);
    gl.clear(gl.COLOR_BUFFER_BIT);
}

window.change_screen_color = change_screen_color;

function js_draw_rectangle(x, y, width, height, red, green, blue, alpha) {
    const canvas = document.getElementById("my_canvas");
    const gl = canvas.getContext("webgl");

    if (!gl) {
        console.error("WebGL not supported");
        return;
    }

    const canvasWidth = canvas.width;
    const canvasHeight = canvas.height;

    const toClipSpace = (coord, size) => (coord / size) * 2 - 1;

    const x1 = toClipSpace(x, canvasWidth);
    const y1 = toClipSpace(canvasHeight - (y + height), canvasHeight);
    const x2 = toClipSpace(x + width, canvasWidth);
    const y2 = toClipSpace(canvasHeight - y, canvasHeight);

    // console.log(`Drawing rect at (${x}, ${y}) -> NDC: (${x1}, ${y1}), (${x2}, ${y2})`);

    const vertices = new Float32Array([
        x1, y1,
        x2, y1,
        x1, y2,
        x1, y2,
        x2, y1,
        x2, y2
    ]);

    if (!gl.program) {
        console.error("WebGL program not initialized");
        return;
    }
    gl.useProgram(gl.program);

    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

    const positionAttributeLocation = gl.getAttribLocation(gl.program, "a_position");
    gl.enableVertexAttribArray(positionAttributeLocation);
    gl.vertexAttribPointer(positionAttributeLocation, 2, gl.FLOAT, false, 0, 0);

    const colorUniformLocation = gl.getUniformLocation(gl.program, "u_color");
    gl.uniform4f(colorUniformLocation, red, green, blue, alpha);

    gl.drawArrays(gl.TRIANGLES, 0, 6);
}
window.js_draw_rectangle = js_draw_rectangle;


function update_score(number) {
    // console.log("Score:", number);

    const scoreElement = document.getElementById("score");
    if (scoreElement) {
        scoreElement.textContent = `Score: ${number}`;
    }
}

window.update_score = update_score;

window.js_draw_text = (text, x, y, size, r, g, b, a) => {
    const ctx = canvas.getContext("2d");
    ctx.font = `${size}px Arial`;
    ctx.fillStyle = `rgba(${r * 255}, ${g * 255}, ${b * 255}, ${a})`;
    ctx.fillText(text, x, y);
};

window.js_draw_sprite = (x, y, width, height, src) => {
    const img = new Image();
    img.src = src;
    img.onload = () => {
        gl.drawImage(img, x, y, width, height);
    };
};

window.play_sound = (src) => {
    const audio = new Audio(src);
    audio.play();
};


init().then(() => {
    console.log("WASM initialized");

    // js_draw_rectangle(100, 100, 50, 50, 1.0, 0.0, 0.0, 1.0);

    // js_draw_rectangle(200, 200, 50, 50, 0.0, 1.0, 0.0, 1.0);

    function updateFPSDisplay() {
        const fpsElement = document.getElementById("fps");
        if (fpsElement) {
            const fps = get_fps();
            fpsElement.textContent = `FPS: ${fps.toFixed(2)}`;
        }
    }

    setInterval(updateFPSDisplay, 250)

    function render() {
        // console.log("Batched Rectangles:", window.batched_rectangles);
    
        if (window.batched_rectangles) {
            window.batched_rectangles.forEach(rect => {
                const { x, y, width, height, r, g, b, a } = rect;
                js_draw_rectangle(x, y, width, height, r, g, b, a)
                
    
            });
        }
    
        requestAnimationFrame(render);
    }
    
    requestAnimationFrame(render);
});

document.getElementById("speed_btn").addEventListener("click", (event) => {
    const currentSpeed = parseFloat(event.target.getAttribute("data-speed"));
    update_speed(currentSpeed);

    const newSpeed = currentSpeed === 0.5 ? 1.0 : 0.5;
    event.target.setAttribute("data-speed", newSpeed);
    event.target.textContent = newSpeed === 0.5 ? "Increase Speed" : "Normal Speed";
});

