import type { GameFrame } from "..";
import invoke from "../invoke";

export default async function get_image(frame: GameFrame): Promise<ArrayBuffer> {
    return await invoke<ArrayBuffer>("get_image", {
        frame
    });
}