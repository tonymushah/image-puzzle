import { invoke as i } from "@tauri-apps/api";
import type { InvokeArgs } from "@tauri-apps/api/tauri";

const pluginName = "image-puzzle";

export default async function invoke<T = unknown>(command: string, args?: InvokeArgs): Promise<T> {
    return await i<T>(`plugin:${pluginName}|${command}`, args);
}