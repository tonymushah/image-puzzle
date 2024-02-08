import invoke from "../invoke";

export default async function load(path: string) {
    await invoke<void>("load", {
        path
    });
}