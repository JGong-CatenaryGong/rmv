<script>
import { T } from '@threlte/core'
import { Outlines } from '@threlte/extras'

export let color;
export let toneMapped;
export let material_type;

</script>
{#if material_type == 'standard'}
  <T.MeshStandardMaterial color={color} toneMapped={toneMapped}/>
{:else if material_type == 'toon'}
  <T.MeshToonMaterial color={color} toneMapped={toneMapped}/>
  <Outlines color="black" thickness={0.02} />
{:else if material_type == 'shine'}
  <T.MeshStandardMaterial 
  color={color} 
  toneMapped={toneMapped} 
  shininess={100} 
  specular={"0x444444"}
  roughness={0.1}
  />
{/if}