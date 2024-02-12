import invoke from '../invoke';

export async function invert_rows() {
	await invoke('invert_rows');
}

export async function invoke_cols() {
	await invoke('invert_cols');
}
