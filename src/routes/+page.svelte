<script lang="ts">
	import { goto } from '$app/navigation';
	import type { GameDimension } from '$lib/plugin';
	import get_current from '$lib/plugin/commands/get_current';
	import init from '$lib/plugin/commands/init';
	import load, { load_party } from '$lib/plugin/commands/load';
	import { dialog } from '@tauri-apps/api';
	import { re } from 'mathjs';
	import { onMount } from 'svelte';

	onMount(async () => {
		const res = await get_current();
		if (res != null) {
			goto('/game');
		}
	});
	enum SizeMode {
		Square,
		Rect
	}
	let error: Error | undefined = undefined;
	let image_path: string | undefined = undefined;
	let game_path: string | undefined = undefined;
	let size: GameDimension | undefined = undefined;
	let size_mode = SizeMode.Square;
	const init_game = async function () {
		if (image_path != undefined && game_path != undefined && size != undefined) {
			try {
				if (typeof size == 'number') {
					const _size = String(size);
					await init({
						image_path,
						game_path,
						size
					});
				} else {
					await init({
						image_path,
						game_path,
						size
					});
				}

				goto('/game');
			} catch (error_) {
				if (error_ instanceof Error) {
					error = error_;
				} else if (typeof error_ == 'string') {
					error = new Error(error_);
				} else {
					error = new Error('Unknow Error', {
						cause: error_
					});
				}
			}
		} else {
			error = new Error('SomeData is missing');
		}
	};
	const load_game = async function () {
		const res = await dialog.open({
			title: 'Load a game',
			multiple: false,
			directory: true,
			recursive: false
		});
		if (typeof res == 'string') {
			await load(res);
			goto('/game');
		}
	};
	const load_party_ = async function () {
		const res = await dialog.open({
			title: 'Load a game',
			multiple: false,
			directory: true,
			recursive: false
		});
		if (typeof res == 'string') {
			await load_party(res);
			goto('/game');
		}
	}
	$: {
		switch (size_mode) {
			case SizeMode.Rect:
				size = {
					columns: 4,
					lines: 3
				};
				break;
			case SizeMode.Square:
				size = 3;
				break;
			default:
				break;
		}
	}
	$: {
		console.log(size);
	}
</script>

<h1>Image Puzzle</h1>

<p>
	Want to <button
		on:click={async () => {
			await load_game();
		}}>load a game save</button
	> or <button
		on:click={async () => {
			await load_party_();
		}}>load a party save</button
	>
</p>

{#if error != undefined}
	<div class="error">
		<p>{error.name} : {error.message}</p>
	</div>
{/if}

<div class="image-path">
	<p>
		{#if image_path != undefined}
			Image choosen: {image_path}
		{:else}
			No image is choosen
		{/if}
	</p>
	<button
		on:click={async () => {
			const res = await dialog.open({
				title: 'Choose a directory',
				multiple: false,
				directory: false
			});
			if (typeof res == 'string') {
				image_path = res;
			}
		}}
		type="button"
	>
		Pickup an image
	</button>
</div>

<div class="game-path">
	<p>
		{#if game_path != undefined}
			Directory choosen: {game_path}
		{:else}
			No directory is choosen
		{/if}
	</p>
	<button
		on:click={async () => {
			const res = await dialog.open({
				title: 'Choose an image',
				multiple: false,
				directory: true,
				recursive: false
			});
			if (typeof res == 'string') {
				game_path = res;
			}
		}}
		type="button"
	>
		Pick a directory for the game
	</button>
</div>

<div class="game-size">
	<p>Game size</p>
	<select
		on:change={({ target }) => {
			if (target != null) {
				if (target.value == 0) {
					size_mode = SizeMode.Square;
				} else if (target.value == 1) {
					size_mode = SizeMode.Rect;
				}
				console.log('target');
			}
		}}
	>
		<option value="0">Square</option>
		<option value="1">Rect</option>
	</select>
	{#if size_mode == SizeMode.Rect}
		<div>
			<p>Columns</p>
			<input
				on:change={({ target }) => {
					if (typeof size == 'object') {
						size.columns = Number(target.value);
					}
				}}
				type="number"
			/>
		</div>
		<div>
			<p>Lines</p>
			<input
				on:change={({ target }) => {
					if (typeof size == 'object') {
						size.lines = Number(target.value);
					}
				}}
				type="number"
			/>
		</div>
	{:else if size_mode == SizeMode.Square}
		<div>
			<p>Size</p>
			<input
				on:change={({ target }) => {
					if (typeof size == 'number') {
						size = Number(target.value);
					}
				}}
				type="number"
			/>
		</div>
	{/if}
</div>

<button on:click={init_game} class="start-game"> Start game </button>

<style>
	.start-game {
		margin: 12px;
	}
	.error {
		color: red;
	}
</style>
