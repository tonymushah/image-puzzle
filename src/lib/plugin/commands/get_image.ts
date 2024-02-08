import type { GameFrame } from "..";
import invoke from "../invoke";

export default async function get_image(frame: GameFrame): Promise<Int8Array> {
    return await invoke<Int8Array>("get_image", {
        frame
    });
}

export async function get_image_by_path(path: string): Promise<Int8Array> {
    return await invoke<Int8Array>("get_image_by_path", {
        path
    });
}