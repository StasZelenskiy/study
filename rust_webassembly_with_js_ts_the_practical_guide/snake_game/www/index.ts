import init, { Direction, World, InitOutput, GameStatus } from "snake_game";
import { rand } from "./utils/rand";

init().then((wasm: InitOutput) => {
    const CELL_SIZE = 40;
    const WORLD_WIDTH = 16;
    const SNAKE_SPAWN_IDS = rand(WORLD_WIDTH * WORLD_WIDTH);
    const POINTS_TO_WIN = 10;

    const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDS, POINTS_TO_WIN);
    const worldWidth = world.width();

    const gameTileGrassImage = <HTMLImageElement> document.getElementById('game-tile-grass');
    const gameTileAppleImage = <HTMLImageElement> document.getElementById('game-tile-apple');
    const gameControlBtn = document.getElementById("game-control-btn");
    const gameStatus = document.getElementById("game-status");
    const points = document.getElementById("points");

    gameControlBtn.addEventListener("click", () => {
        const status = world.game_status();
        if (status === undefined) {
            gameControlBtn.textContent = "Playing...";
            world.start_game();
            play();
        } else {
            location.reload();
        }
    });

    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    document.addEventListener("keydown", (e) => {
        switch (e.code) {
            case "ArrowUp":
                world.change_snake_dir(Direction.Up);
                break;
            case "ArrowRight":
                world.change_snake_dir(Direction.Right);
                break;
            case "ArrowDown":
                world.change_snake_dir(Direction.Down);
                break;
            case "ArrowLeft":
                world.change_snake_dir(Direction.Left);
                break;
            default:
                break;
        }
    });

    const drawWorld = () => {
        ctx.beginPath();
        ctx.drawImage(gameTileGrassImage, 0, 0, canvas.width, canvas.height);
        ctx.stroke();
    };

    const drawReward = () => {
        const idx = world.reward_cell();
        const col = idx % worldWidth;
        const row = Math.floor(idx / worldWidth);

        ctx.beginPath();
        ctx.drawImage(gameTileAppleImage, col * CELL_SIZE - 1, row * CELL_SIZE - 1, CELL_SIZE + 2, CELL_SIZE + 2);
        ctx.stroke();
    }

    const drawSnake = () => {
        const snakeCellPtr = world.snake_cells();
        const snakeLength = world.snake_length();
        const snakeCells = new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLength);
        snakeCells
            .slice()
            .reverse()
            .forEach((cellIdx, i) => {
                const col = cellIdx % worldWidth;
                const row = Math.floor(cellIdx / worldWidth);

                ctx.fillStyle = i === snakeCells.length - 1 ? "#7878db" : "#000000";

                ctx.beginPath();
                ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            });
        ctx.stroke();
    };

    const drawGameStatus = () => {
        gameStatus.textContent = world.game_status_text();
        points.textContent = world.points().toString();
    };

    const paint = () => {
        drawWorld();
        drawSnake();
        drawReward();
        drawGameStatus();
    };

    const play = () => {
        const status = world.game_status();
        if (status == GameStatus.Lost || status == GameStatus.Won) {
            gameControlBtn.textContent = "Replay";
            return;
        }
        
        const fps = 10;
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            world.step();
            paint();
            requestAnimationFrame(play);
        }, 1000 / fps);
    };

    paint();
});
