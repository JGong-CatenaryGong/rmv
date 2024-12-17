<script lang="ts">
  import { Canvas } from '@threlte/core'
  import Scene from './scene.svelte';

  export let type:string

  export let coords:any[];
  export let atoms:number[];
  export let color:string[];
  export let radii:number[];
</script>

<Canvas>
  <Scene
    coords={coords}
    color={color}
    radii={radii}
  />
</Canvas>
