<script lang="ts">
	import { goto } from '$app/navigation';
	import ImageFrame from '$lib/componnents/ImageFrame.svelte';
	import type { GameFrame } from '$lib/plugin';
	import get_current from '$lib/plugin/commands/get_current';
	import get_moves from '$lib/plugin/commands/get_moves';
	import _swap, {
		swap_columns as _swap_columns,
		swap_rows as _swap_rows
	} from '$lib/plugin/commands/swap';
	import _have_won from '$lib/plugin/commands/have_won';
	import { current_matrix } from '$lib/stores/current_party';
	import { appWindow } from '@tauri-apps/api/window';
	import { matrix } from 'mathjs';
	import { onDestroy, onMount } from 'svelte';
	import _reset from '$lib/plugin/commands/reset';

	let columns: number;
	let have_won = false;
	let rows: number;
	let moves = 0;
	let current: GameFrame | undefined = undefined;

	let target: GameFrame | undefined = undefined;

	async function refresh() {
		try {
			const d = await get_current();
			if (d != null) {
				current_matrix.set(d);
				rows = d.size()[0];
				columns = d.size()[1];
				moves = await get_moves();
				have_won = await _have_won();
				await appWindow.emit('refresh-images');
			} else {
				goto('/');
			}
		} catch (error) {
			goto('/');
		}
	}

	async function swap() {
		if (current != undefined && target != undefined) {
			await _swap(current, target);
			current = undefined;
			target = undefined;
			await refresh();
		}
	}

	onMount(async () => {
		await refresh();
	});
	onDestroy(() => {
		current_matrix.set(matrix());
	});

	let current_row: number | undefined = undefined;
	let target_row: number | undefined = undefined;

	async function swap_rows() {
		if (current_row != undefined && target_row != undefined) {
			await _swap_columns(current_row, target_row);
			current_row = undefined;
			target_row = undefined;
			await refresh();
		}
	}

	async function reset() {
		await _reset();
		goto('/');
	}

	let current_column: number | undefined = undefined;
	let target_column: number | undefined = undefined;

	async function swap_columns() {
		if (current_column != undefined && target_column != undefined) {
			await _swap_rows(current_column, target_column);
			current_column = undefined;
			target_column = undefined;
			await refresh();
		}
	}
	$: moves_ = moves <= 1 ? 'move' : 'moves';
</script>

{#if have_won}
	<h2>You won with {moves} {moves_}</h2>
	<button
		on:click={async () => {
			await reset();
		}}>A new one</button
	>
{/if}
<table>
	<tr>
		<td class:have_won>
			{moves}
		</td>
		{#each Array(columns).keys() as c}
			<td>
				<button
					on:click={async () => {
						if (!have_won) {
							if (current_column == undefined) {
								current_column = c;
							} else if (target_column == undefined) {
								target_column = c;
							}
							await swap_columns();
						}
					}}
				>
					{c + 1}
				</button>
			</td>
		{/each}
	</tr>
	{#each Array(rows).keys() as x}
		<tr>
			<td>
				<button
					on:click={async () => {
						if (!have_won) {
							if (current_row == undefined) {
								current_row = x;
							} else if (target_row == undefined) {
								target_row = x;
							}
							await swap_rows();
						}
					}}
				>
					{x + 1}
				</button>
			</td>
			{#each Array(columns).keys() as y}
				<td>
					<button
						on:click={async () => {
							if (!have_won) {
								if (current == undefined) {
									current = {
										x,
										y
									};
								} else if (target == undefined) {
									target = {
										x,
										y
									};
								}
								await swap();
							}
						}}
					>
						<ImageFrame {x} {y} />
					</button>
				</td>
			{/each}
		</tr>
	{/each}
</table>

<style>
	.have_won {
		color: red;
	}
</style>
