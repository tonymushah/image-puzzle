import invoke from "../invoke";

export default async function get_moves(): Promise<number> {
    return await invoke<number>("get_moves");
}