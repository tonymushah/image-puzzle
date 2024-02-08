import { matrix } from "mathjs";
import { writable } from "svelte/store";

export const current_matrix = writable(matrix("dense"));