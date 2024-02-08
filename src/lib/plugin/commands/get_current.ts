import invoke from "../invoke";
import * as math from "mathjs";

type GetCurrentRes = [string[], number, number];

export default async function get_current(): Promise<math.Matrix | null> {
    const res = await invoke<GetCurrentRes | null>("get_current");
    if (res !== null) {
        const [data, cols,] = res;
        const matrix: string[][] = [];
        for (let i = 0; i < data.length; i += cols) {
            const d = data.slice(i, i + cols);
            matrix.push(d);
        }
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        /* @ts-expect-error */
        return math.matrix(matrix);
    } else {
        return null;
    }
}