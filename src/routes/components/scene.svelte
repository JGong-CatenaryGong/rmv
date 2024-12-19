<script lang="ts">
import { T } from '@threlte/core'
import { Grid, OrbitControls, interactivity } from '@threlte/extras'
import { onMount } from 'svelte';
import { spring } from 'svelte/motion';
import * as THREE from 'three';
import Meshmaterial from './meshmaterial.svelte';

interactivity();

export let type:string;
export let coords:any[];
export let color:string[];
export let radii:any[];

export let bond_coords:any[][];
export let bond_lengths:any[][];
export let bond_width:number;
export let cov_bond_info:any[][];
export let vdw_bond_info:any[][];
export let display_vdw:string;

export let material_type:string;

function calcQuaternion(x:number, y:number, z:number) {
  let quaternion = new THREE.Quaternion();
  let up = new THREE.Vector3(0,1,0);
  quaternion.setFromUnitVectors(up, new THREE.Vector3(x,y,z))
  // console.log(quaternion.x, quaternion.y, quaternion.z, quaternion.w)
  return [quaternion.x, quaternion.y, quaternion.z, quaternion.w];
}

let bond_coords_flat:any[];
let bond_lengths_flat:any[];
$: [bond_coords_flat, bond_lengths_flat] = cov_bond_info

let vdw_coords_flat:any[];
let vdw_lengths_flat:any[];
$: [vdw_coords_flat, vdw_lengths_flat] = vdw_bond_info
// $: bond_coords_flat = bond_coords.flat(1).filter(x => x[3] !==0 && x[3] !==0.5);
// $: bond_lengths_flat = bond_lengths.flat(1).filter((_, idx) => bond_coords.flat(1)[idx][3] !==0 && bond_coords.flat(1)[idx][3] !==0.5);

onMount(() => {
  // console.log(coords);
  // console.log(radii);
  console.log("Molviewer mounted");

  // filter_bonds(bond_coords, bond_lengths);
  
  // console.log(bond_coords_flat);
  // console.log(bond_lengths_flat);
})

</script>

<T.PerspectiveCamera
  makeDefault
  position={[12, 12, 12]}
>
  <OrbitControls />
</T.PerspectiveCamera>

<T.DirectionalLight
  position={[3, 10, 7]}
  intensity={Math.PI}
/>
<T.AmbientLight intensity={0.3} />

<T.Group>
  {#if type !== 'stick'}
    {#each coords as coord, i}
    <T.Mesh position.x={coord[0]} position.y={coord[1]} position.z={coord[2]}>
      <T.SphereGeometry args={[radii[i] / 100, 60, 60]} />
      <Meshmaterial material_type={material_type} color={'#' + color[i]} toneMapped={false}/>
      <!-- <T.MeshStandardMaterial color={'#' + color[i]} toneMapped={false}/> -->
    </T.Mesh>
    {/each}
  {:else if type === 'stick'}
    {#each coords as coord, i}
    <T.Mesh position.x={coord[0]} position.y={coord[1]} position.z={coord[2]}>
      <T.SphereGeometry args={[bond_width, 20, 20]} />
      <Meshmaterial material_type={material_type} color={'#FFFFFF'} toneMapped={false}/>
    </T.Mesh>
    {/each}
  {/if}
  {#if type === 'cpk' || type === 'stick'}
    {#each bond_coords_flat as bond, i}
    <!-- bond[4,5,6]s are orientation vectors -->
    <T.Mesh 
    position.x={bond[0]} 
    position.y={bond[1]} 
    position.z={bond[2]} 
    quaternion={calcQuaternion(bond[4], bond[5], bond[6])}
    >
      <T.CylinderGeometry args={[bond_width, bond_width, bond_lengths_flat[i], 32]} />
      <Meshmaterial material_type={material_type} color={'#FFFFFF'} toneMapped={false}/>
    </T.Mesh>
    {/each}
    {#if display_vdw == "vdw"}
      {#each vdw_coords_flat as bond, i}
      <T.Mesh 
      position.x={bond[0]} 
      position.y={bond[1]} 
      position.z={bond[2]} 
      quaternion={calcQuaternion(bond[4], bond[5], bond[6])}
      >
        <T.CylinderGeometry args={[bond_width, bond_width, vdw_lengths_flat[i], 32]}/>
        <Meshmaterial material_type={material_type} color={'#87ceeb'} toneMapped={false}/>
      </T.Mesh>
      {/each}
    {/if}
  {/if}
</T.Group>
