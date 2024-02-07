import type { GameFrame } from "..";
import invoke from "../invoke";

export default async function swap(current: GameFrame, target: GameFrame) {
    await invoke<void>("swap", {
        args: {
            current,
            target
        }
    });
}

export async function swap_rows(current: number, target: number) {
    await invoke<void>("swap_rows", {
        args: {
            current,
            target
        }
    });
}

export async function swap_columns(current: number, target: number) {
    await invoke<void>("swap_columns", {
        args: {
            current,
            target
        }
    })
}