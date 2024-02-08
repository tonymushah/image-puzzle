import invoke from "../invoke";

export default async function reset(): Promise<void> {
    return await invoke<void>("reset");
}