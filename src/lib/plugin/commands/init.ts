import type { GameDimension } from "..";
import invoke from "../invoke";

export type GameInitArgs = {
    image_path: string,
    game_path: string,
    size: GameDimension
}

export default async function init(args: GameInitArgs) {
    await invoke<void>("init", {
        args
    });
}