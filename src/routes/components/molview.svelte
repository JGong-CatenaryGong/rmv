<script lang="ts">
  import { Canvas } from '@threlte/core'
  import Scene from './scene.svelte';
  import { Li, List, Card, GradientButton, Drawer, Button, CloseButton, A } from 'flowbite-svelte';
  import { FileBox, BookOpenText, CircleCheckBig, FolderOpen, Box } from 'lucide-svelte';

  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';

  export let mol_info:any;

  // color={mol_info.draw_info[0]}
  // radii={mol_info.draw_info[1]}

  // let coords=$state(mol_info.coordinates);
  // let color=$state(mol_info.draw_info[0]);
  // let radii=$state(mol_info.draw_info[3]);

  function countTrues(arr: boolean[][]) {
    return arr.flat(1).filter(x => x === true).length;
  }

  import { Label, Select, Range, Checkbox } from 'flowbite-svelte';
    import { onMount } from 'svelte';

  let type = "cpk";
  let display_types = [
    { value: "cpk", name: "CPK" },
    { value: "vdw", name: "Van der Waals Spheres" },
    { value: "cov", name: "Covalent Spheres" },
    { value: "stick", name: "Stick" },
  ];

  let bond_width_range = 0.1;
  let display_vdw = "cov";
  let vdw_types = [
    { value: "vdw", name: "Yes" },
    { value: "cov", name: "No" },
  ];

  let material_type = "standard";
  let material_types = [
    { value: "standard", name: "Standard" },
    { value: "toon", name: "Cartoon" },
    { value: "shine", name: "Shine" },
  ];


</script>
<div class="flex h-dvh w-screen bottom-4 top-4 right-4 left-4">
  <Label class="absolute top-4 right-4 z-50">
    Display Mode
    <Select class="mt-2" items={display_types} bind:value={type} />
  </Label>
  {#if type === "cpk" || type === "stick"} 
  <Label class="absolute top-24 right-4 z-50">
    Bond Width
    <Range id="range-steps" min="0" max="0.5" bind:value={bond_width_range} step="0.05" />
  </Label>
  <Label class="absolute top-40 right-4 z-50">
    Display Weak Contacts
    <Select class="mt-2" items={vdw_types} bind:value={display_vdw} />
  </Label>
  {/if}
  <Label class="absolute top-64 right-4 z-50">
    Materials Mode
    <Select class="mt-2" items={material_types} bind:value={material_type} />
  </Label>
  <div class="grow max-w-lg">
    <Table>
      <caption>
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
          <BookOpenText class="w-8 h-8 mb-3 text-gray-500 dark:text-gray-400"/>
          Geometric Information
        </h5>
      </caption>
      <TableHead>
        <TableHeadCell>Property</TableHeadCell>
        <TableHeadCell>Value</TableHeadCell>
      </TableHead>
      <TableBody>
        <TableBodyRow>
          <TableBodyCell>Formula</TableBodyCell>
          <TableBodyCell>{mol_info.formula}</TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
          <TableBodyCell>MW</TableBodyCell>
          <TableBodyCell>{mol_info.molecular_weight}</TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
          <TableBodyCell>Number of atoms</TableBodyCell>
          <TableBodyCell>{mol_info.no_atoms}</TableBodyCell>
        </TableBodyRow>
        <TableBodyRow>
          <TableBodyCell>Number of Bonds</TableBodyCell>
          <TableBodyCell>{countTrues(mol_info.connections[1]) / 2}</TableBodyCell>
        </TableBodyRow>
      </TableBody>
    </Table>
    {#if Array.isArray(mol_info.calculated_info) && mol_info.calculated_info.length !== 0}
    <Table>
      <caption>
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
          Calculated Information
        </h5>
      </caption>
      <TableHead>
        <TableHeadCell>Property</TableHeadCell>
        <TableHeadCell>Value</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each mol_info.calculated_info as info}
        <TableBodyRow>
          <TableBodyCell>{info[0]}</TableBodyCell>
          <TableBodyCell>{info[1]}</TableBodyCell>
        </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
    {/if}

    <!-- <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
      <BookOpenText class="w-8 h-8 mb-3 text-gray-500 dark:text-gray-400"/>
      Molecular Information
    </h5>
    <div class="font-normal text-gray-700 dark:text-gray-400 leading-tight">
      <List tag="ul" class="space-y-1 text-gray-500 dark:text-gray-400">
        <Li>
          <b>Chemcial Formula</b> {mol_info.formula}
        </Li>
        <Li>
          <b>MW</b> {mol_info.molecular_weight}
        </Li>
      </List>
    </div> -->
  </div>
  <!-- <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white absolute top-4 right-4 float-right">
    <Box class="w-8 h-8 mb-3 text-gray-500 dark:text-gray-400"/>
    Molecular Structure
  </h5> -->
  <div class="flex-auto left-4">
    <Canvas>
      <Scene
      type={type}
      coords={mol_info.coordinates}
      color={mol_info.draw_info[0]}
      radii={type === "cpk" ? mol_info.draw_info[3] : 
             type === "vdw" ? mol_info.draw_info[1] : 
             type === "cov" ? mol_info.draw_info[2] : []}
      bond_coords={mol_info.connections[2]}
      bond_lengths={mol_info.connections[3]}
      cov_bond_info={mol_info.bond_info}
      vdw_bond_info={mol_info.vdw_info}
      display_vdw={display_vdw}
      material_type={material_type}
      bind:bond_width={bond_width_range}
      />
    </Canvas>
  </div>
</div>
