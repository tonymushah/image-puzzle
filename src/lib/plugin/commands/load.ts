import invoke from '../invoke';

export default async function load(path: string) {
	await invoke<void>('load', {
		path
	});
}

export async function load_party(path: string) {
	await invoke<void>('load_party', {
		path
	});
}
