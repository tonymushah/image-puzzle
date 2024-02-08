<script lang="ts">
	import { goto } from '$app/navigation';
	import ImageFrame from '$lib/componnents/ImageFrame.svelte';
	import type { GameFrame } from '$lib/plugin';
	import get_current from '$lib/plugin/commands/get_current';
	import get_moves from '$lib/plugin/commands/get_moves';
	import _swap from '$lib/plugin/commands/swap';
	import { current_matrix } from '$lib/stores/current_party';
	import { appWindow } from '@tauri-apps/api/window';
	import { matrix } from 'mathjs';
	import { onDestroy, onMount } from 'svelte';

	let columns: number;
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
                await appWindow.emit("refresh-images");
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
			refresh();
		}
	}

	onMount(async () => {
		await refresh();
	});
    onDestroy(() => {
        current_matrix.set(matrix());
    })
</script>

<p>Moves : {moves}</p>

{#each Array(rows).keys() as x}
	<div>
		{#each Array(columns).keys() as y}
			<button
				on:click={async () => {
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
				}}
			>
				<ImageFrame {x} {y} />
			</button>
		{/each}
	</div>
{/each}
