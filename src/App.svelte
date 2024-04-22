<script lang="ts">
	import {onMount} from "svelte";
	import {repositionApp, setAppHeight} from "./lib/utils";

	type InputKeyEvent = KeyboardEvent & { currentTarget: EventTarget & HTMLInputElement }

	let main: HTMLElement;
	let options: HTMLElement[] = [];
	let observer: ResizeObserver;

	onMount(() => {
		observer = new ResizeObserver(elems => {
			setAppHeight(elems[0].borderBoxSize[0].blockSize)
		});
		observer.observe(main);
		repositionApp();
	})

	const data = [
		{label: "California"},
		{label: "North Carolina"},
		{label: "North Dakota"},
		{label: "South Carolina"},
		{label: "South Dakota"},
		{label: "Michigan"},
		{label: "Tennessee"},
		{label: "Nevada"},
		{label: "New Hampshire"},
		{label: "New Jersey"},
	];

	let shownData = data;
	let searchValue = "";
	let selectIndex = -1;

	const cancelEvent = () => false;

	function chooseOption(index: number) {
		if (index < 0) { index = 0 }
		else if (index >= shownData.length - 1) { index = shownData.length - 1 }
		console.info(index)
		selectIndex = index
		options[selectIndex].scrollIntoView({
			block: "nearest"
		})
	}
	function select(e: InputKeyEvent) {
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
				// TODO: close app and return value
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


</script>

<main bind:this={main}>
	<input type="text" placeholder="Search..." bind:value={searchValue}
		   on:keydown={select} on:keyup={search} on:change={search} use:focus />
	<div id="options" on:selectstart={cancelEvent}>
		{#each shownData as d, i}
			<div class="option" class:selected={selectIndex === i}
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
