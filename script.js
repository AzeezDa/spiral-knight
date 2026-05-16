import init, { draw, get_max_knights } from "./pkg/spiral_knight.js"

const DEFAULT_SIZE = 100;
const MIN_SIZE = 100;
const MAX_SIZE = 1500;
const DEFAULT_FIRST_KNIGHTS = [["1", "2", "#000000"], ["1", "2", "#FF0000"]];
const DEFAULT_REST_KNIGHTS = ["0", "0", "#000000"];
const STATUS_DONE = "<p data-tooltip=\"The drawing shown is according to the settings\">Status: Done</p>";
const STATUS_NOT_DRAWN = "<p data-tooltip=\"Press the Draw button to update the drawing\">Status: Not drawn</p>";

let size_input = () => document.getElementById("size input");
let status_label = () => document.getElementById("status");
let size = DEFAULT_SIZE;

function update() {
    if (document.getElementById("auto draw").checked) {
        do_draw()
    } else {
        status_label().innerHTML = STATUS_NOT_DRAWN;
    }
}

function do_draw() {
    draw(size)
    status_label().innerHTML = STATUS_DONE
}

init().then(() => {
    // Set up knight settings
    for (let index = 1; index <= get_max_knights(); index++) {
        let name = `Knight ${index}`;
        let dx = DEFAULT_REST_KNIGHTS[0];
        let dy = DEFAULT_REST_KNIGHTS[1];
        let colour = DEFAULT_REST_KNIGHTS[2];
        if (index == 1 || index == 2) {
            dx = DEFAULT_FIRST_KNIGHTS[index - 1][0];
            dy = DEFAULT_FIRST_KNIGHTS[index - 1][1];
            colour = DEFAULT_FIRST_KNIGHTS[index - 1][2];
        }
        document.getElementById("knights").innerHTML += `<div class=\"grid container\">
            <p class="center">${name}</p>
            <input type="number" id="${name} dx" min="0" value=${dx}></input>
            <input type="number" id="${name} dy" min="0" value=${dy}></input>
            <input type="color" id="${name} colour" min="0" value=${colour}></input>
            </div>`;

    }
    update()

    // Set up listeners
    document.getElementById("draw").onclick = do_draw;
    size_input().onchange = () => { size = Number(size_input().value); update() }

    for (let index = 1; index <= get_max_knights(); index++) {
        document.getElementById(`Knight ${index} dx`).onchange = update
        document.getElementById(`Knight ${index} dy`).onchange = update
        document.getElementById(`Knight ${index} colour`).onchange = update
    }

    document.getElementById("output").onwheel = (e) => {
        if (e.shiftKey) {
            let new_size = Math.min(Math.max(Math.trunc(size + e.deltaY / 10), MIN_SIZE), MAX_SIZE);
            if (new_size != size) {
                size = new_size
                size_input().value = size;
                update();
            }
        }
    };

    document.body.addEventListener("keydown", function(e) {
        if (e.key == "z" && size != MIN_SIZE) {
            size = MIN_SIZE
            size_input().value = size;
            update();
        }

        if (e.key == "Z" && size != MAX_SIZE) {
            size = MAX_SIZE
            size_input().value = size;
            update();
        }

        if (e.key == "r") {
            size = DEFAULT_SIZE
            size_input().value = DEFAULT_SIZE
            for (let index = 1; index <= get_max_knights(); index++) {
                let name = `Knight ${index}`;
                let dx = DEFAULT_REST_KNIGHTS[0];
                let dy = DEFAULT_REST_KNIGHTS[1];
                let colour = DEFAULT_REST_KNIGHTS[2];
                if (index == 1 || index == 2) {
                    dx = DEFAULT_FIRST_KNIGHTS[index - 1][0];
                    dy = DEFAULT_FIRST_KNIGHTS[index - 1][1];
                    colour = DEFAULT_FIRST_KNIGHTS[index - 1][2];
                }
                document.getElementById(`${name} dx`).value = dx;
                document.getElementById(`${name} dy`).value = dy;
                document.getElementById(`${name} colour`).value = colour;
            }
            update();
        }

    });
});
