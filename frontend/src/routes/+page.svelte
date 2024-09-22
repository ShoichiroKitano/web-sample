<script lang=ts>
  import { onMount } from 'svelte';

  let samples = [];
  let count = 10;

  async function get_samples() {
    const response = await fetch('http://localhost:8080/samples');
    const json = await response.json();
    samples = json.samples;
  }

  async function post_new_sample_and_reload() {
    await fetch('http://localhost:8080/samples', {
      headers: { 'Content-Type': 'application/json' },
      method: 'POST',
      body: JSON.stringify({name: `name${count}`, status: count}),
    });
    count++;
    await get_samples();
  }

  onMount(get_samples);
</script>

<h1> Sampleを表示するよ </h1>
{#each samples as s (s.id) }
  <div> name = {s.name} </div>
  <div> status = {s.status} </div>
{/each}

<button on:click={post_new_sample_and_reload}> クリック </button>
