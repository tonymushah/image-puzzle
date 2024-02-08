<script lang="ts">
	import get_image, { get_image_by_path } from "$lib/plugin/commands/get_image";

	import type { UnlistenFn } from "@tauri-apps/api/event";
	import { appWindow } from "@tauri-apps/api/window";
	import { onDestroy, onMount } from "svelte";
    let img: HTMLImageElement;
    
    export let x: number;
    export let y: number;

    async function load() {
        const buf = await get_image({
            x,
            y
        });
        const data = btoa(buf.reduce((data, byte) => data + String.fromCharCode(byte), ''));
        img.src = `data:image/*;base64,${data}`;
        console.log("refreshed image")
        //crate.src = src;
    }

    const unlisten: UnlistenFn[] = [];
    onMount(async () => {
        await load();
        unlisten.push(await appWindow.listen("refresh-images", () => {
            load();
        }));
    });
    onDestroy(() => {
        unlisten.forEach((u) => u());
    })
</script>

<img bind:this={img} alt=""/>