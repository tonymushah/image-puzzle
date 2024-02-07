import invoke from "../invoke";

export default async function have_won(): Promise<boolean> {
    return await invoke<boolean>("have_won");
}