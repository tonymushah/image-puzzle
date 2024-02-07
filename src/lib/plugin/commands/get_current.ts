import invoke from "../invoke";
import math from "mathjs";

type GetCurrentRes = [string[], number, number];

export default async function get_current(): Promise<math.Matrix> {
    const [data, cols,] = await invoke<GetCurrentRes>("get_current");
    const matrix: string[][] = [];
    for (let i = 0; i < length; i += cols) {
        matrix.push(data.slice(i, i + cols));
    }
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    /* @ts-expect-error */
    return math.matrix(matrix);
}