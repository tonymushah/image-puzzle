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
	import _transpose from '$lib/plugin/commands/transpose';
	import {
		invert_rows as _invert_rows,
		invoke_cols as _invert_cols
	} from '$lib/plugin/commands/invert';
	import save_party from '$lib/plugin/commands/save_party';

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
	async function transpose() {
		if (!have_won) {
			await _transpose();
			await refresh();
		}
	}
	async function invert_rows() {
		if (!have_won) {
			await _invert_rows();
			await refresh();
		}
	}
	async function invert_cols() {
		if (!have_won) {
			await _invert_cols();
			await refresh();
		}
	}
	$: isSquare = columns == rows;
	$: moves_ = moves <= 1 ? 'move' : 'moves';
</script>

{#if have_won}
	<h2>You won with {moves} {moves_}</h2>
	<button
		on:click={async () => {
			await reset();
		}}>A new one</button
	>
{:else}
	<button
		on:click={async () => {
			await reset();
		}}>A new one</button
	>
{/if}

<div class="controls">
	<button type="button" on:click={save_party}>Save Party</button>
	{#if isSquare}
		<button type="button" on:click={transpose}>Transpose</button>
	{/if}
	<button type="button" on:click={invert_cols}>Invert Columns</button>
	<button type="button" on:click={invert_rows}>Invert Rows</button>
</div>

<table>
	<tr>
		<td class:have_won>
			{moves}
		</td>
		{#each Array(columns).keys() as c}
			<td>
				<button
					class:selected={current_column == c || target_column == c}
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
					class:selected={target_row == x || current_row == x}
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
						class:selected={(current?.x == x && current.y == y) || (target?.x == x && target.y == y)}
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
	.controls {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
	}
	.controls > button{
		padding: 10px;
	}
	.selected {
		background-color: antiquewhite;
	}
</style>
