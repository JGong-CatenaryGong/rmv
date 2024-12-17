<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { readTextFile } from "@tauri-apps/plugin-fs";
  import { getCurrentWindow } from "@tauri-apps/api/window";
 
  import { FileBox, BookOpenText, CircleCheckBig, FolderOpen, Box } from 'lucide-svelte';
 
  import { GradientButton } from 'flowbite-svelte';
  import { Toast } from 'flowbite-svelte';
  import { Li, List, Card } from 'flowbite-svelte';

  import { slide } from 'svelte/transition';
  import { onMount } from 'svelte';

  import Molview from "./components/molview.svelte";

  // struct MolInfo {
  //   formula: String,
  //   no_atoms: usize,
  //   atoms: Vec<usize>,
  //   coordinates: Vec<[f64; 3]>,
  //   molecular_weight: f64,
  //   connections: (Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<[f64;4]>>),
  //   draw_info: (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>),
  // }

  let mol_and_info = $state("");
  let mol = $state({
    name: "",
    atoms: [],
    coordinates: [],
  });
  let mol_info = $state({
    formula: "",
    no_atoms: 0,
    atoms: [],
    coordinates: [],
    molecular_weight: 0.0,
    connections: [[], [], []],
    draw_info: [[], [], [], []],
  })

  let toastStatus = $state(false);
  let molStatus = $state(false);
  let counter = 6;

  function timeout() {
    if (--counter > 0) return setTimeout(timeout, 1000);
    toastStatus = false;
  }

  const onOpenFiles = async () => {
    try {
    const selected = await open({
        multiple: false,
      });

      if (selected && selected.length > 0) {
        const path = selected[0];
        console.log("Selected path:", path);
      }

      mol_and_info = await invoke("load_mol", { filename: selected });
      mol = JSON.parse(mol_and_info[0]);
      mol_info = JSON.parse(mol_and_info[1]);
      console.log("Selected file:", selected);

      toastStatus = true;
      console.log("toastStatus: ", toastStatus);

      molStatus = true;
      console.log("molStatus: ", molStatus);

      timeout();
    } catch (error) {
      console.log("Failed to open file");
    }
  }

  let width = $state(0);
  let height = $state(0);

  async function listenWindowSize() {
    let size = await getCurrentWindow().innerSize();
    width = size.width;
    height = size.height;
    console.log("Window size:", width, height);
  }

  onMount(() => {
    console.log("onMount");
    getCurrentWindow().onResized(listenWindowSize);
    // listenWindowSize();
  })

  let type = $state("state");
</script>

<main class="container p-4">
  <GradientButton 
  color="purpleToBlue" 
  type="button" 
  class="btn-icon btn-lg variant-filled float-right absolute bottom-4 right-4" 
  onclick={onOpenFiles}
  >
    <FileBox />
  </GradientButton>
  <Toast 
  class="float-left absolute bottom-4 left-4" 
  dismissable={false} 
  transition={slide} 
  bind:toastStatus
  >
  <CircleCheckBig slot="icon" />
  Molecule file loaded
  </Toast>
  {#if molStatus}
  <Card class="p-4">
    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
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
    </div>
  </Card>
  <Card class="p-4">
    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
      <Box class="w-8 h-8 mb-3 text-gray-500 dark:text-gray-400"/>
      Molecular Structure
    </h5>
    <Molview 
    type={type}
    coords={mol_info.coordinates}
    atoms={mol_info.atoms}
    color={mol_info.draw_info[0]}
    radii={mol_info.draw_info[1]}
    />
  </Card>
    {:else}
    <div class="h-screen flex items-center justify-center">
      <h5 class="font-bold text-gray-400 dark:text-white hover:text-gray-500 dark:text-white">
        <FolderOpen class="size-24" onclick={onOpenFiles}/>
        Please load a molecule file in .xyz or .out
      </h5>
    </div>
  {/if}
  
</main>

