<script lang="ts">
	import {onMount} from "svelte";
	import {invoke} from "@tauri-apps/api";
	import {exit} from "@tauri-apps/api/process";
	import {repositionApp, setAppHeight} from "./lib/utils";

	type InputKeyEvent = KeyboardEvent & { currentTarget: EventTarget & HTMLInputElement }
	type Option = {
		value: string,
		label: string
	}

	let main: HTMLElement;
	let options: HTMLElement[] = [];
	let observer: ResizeObserver;
	let data: Option[] = [];
	let shownData = [];
	let searchValue = "";
	let selectIndex = -1;

	onMount(async () => {
		shownData = data = await invoke<Option[]>("get_options");
		observer = new ResizeObserver(elems => {
			setAppHeight(elems[0].borderBoxSize[0].blockSize)
		});
		observer.observe(main);
		await repositionApp();
	});

	const cancelEvent = () => false;

	function chooseOption(index: number) {
		if (index < 0) { index = 0 }
		else if (index >= shownData.length - 1) { index = shownData.length - 1 }
		selectIndex = index
		options[selectIndex].scrollIntoView({
			block: "nearest"
		})
	}

	async function sendValue(index: number) {
		await invoke("stdout", { value: shownData[selectIndex].value })
		await exit(0)
	}

	async function selectKey(e: InputKeyEvent) {
		switch (e.code) {
			case 'ArrowUp':
				chooseOption(selectIndex - 1)
				e.preventDefault()
				break
			case 'ArrowDown':
				chooseOption(selectIndex + 1)
				e.preventDefault()
				break
			case 'Enter':
				await sendValue(selectIndex)
				break
		}
	}

	function search(e: InputKeyEvent) {
		if (e.code == 'ArrowUp' || e.code == 'ArrowDown' || e.code == 'Enter') {
			return;
		}
		selectIndex = -1
		shownData = data.filter( d => d.label.toLowerCase().match(searchValue.toLowerCase()))
	}

	function focus(e: HTMLElement) {
		e.focus()
	}

	function setHoverIndex(i: number) {
		return () => {
			chooseOption(i)
		};
	}

	function selectMouse(i: number) {
		return () => {
			sendValue(i)
		}
	}


</script>

<main bind:this={main}>
	<input type="text" placeholder="Search..." bind:value={searchValue}
		   on:keydown={selectKey} on:keyup={search} on:change={search} use:focus />
	<div id="options" on:selectstart={cancelEvent}>
		{#each shownData as d, i}
			<div class="option" class:selected={selectIndex === i}
				 on:keydown={selectMouse(i)}
				 on:click={selectMouse(i)}
				 on:mouseenter={setHoverIndex(i)}
				 on:selectstart={cancelEvent}
				 bind:this={options[i]}>{d.label}</div>
		{/each}
		{#if !shownData.length}
			<div class="option">(no data)</div>
		{/if}
	</div>
</main>

<style>
	main {
		width: 100%;
		position: relative;
	}

	input[type="text"] {
		width: 100%;
		font-size: 130%;
		padding: 25px;
	}

	#options {
		width: 100%;
		border-radius: 15px;
		border: 1px solid #333;
		padding: 15px;
		background-color: rgba(0, 0, 0, 0.8);
		max-height: 400px;
		overflow: auto;
	}

	.option {
		font-size: 110%;
		padding: 15px;
		border-radius: 15px;
		cursor: pointer;
		&.selected {
			background: rgba(100, 100, 100, 0.8);
		}
	}
</style>
