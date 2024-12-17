<script lang="ts">
import { T } from '@threlte/core'
import { Grid, OrbitControls, interactivity } from '@threlte/extras'
import { onMount } from 'svelte';
import { spring } from 'svelte/motion'

interactivity();

export let coords:any[];
export let color:string[];
export let radii:any[];

console.log(coords);

onMount(() => {
  console.log("Molviewer mounted");
})

</script>

<T.PerspectiveCamera
  makeDefault
  position={[10, 10, 10]}
>
  <OrbitControls />
</T.PerspectiveCamera>

<T.DirectionalLight
  position={[3, 10, 7]}
  intensity={Math.PI}
/>
<T.AmbientLight intensity={0.3} />

<T.Group>
  {#each coords as coord, i}
  <T.Mesh position.x={coords[i][0]} position.y={coords[i][1]} position.z={coords[i][2]}>
    <T.SphereGeometry args={radii[i]} />
    <T.MeshStandardMaterial color={'#' + color[i]} toneMapped={false}/>
  </T.Mesh>
  {/each}
</T.Group>
