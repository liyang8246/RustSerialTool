<script>
	import ConnConfig from './panel/ConnConfig.svelte';
  import InputConfig from './panel/InputConfig.svelte';
  import OutputConfig from './panel/OutputConfig.svelte';
  import OutputArea from './panel/OutputArea.svelte';
  import InputArea from './panel/InputArea.svelte';
</script>

<div class="bg-gray-100 flex w-screen h-screen p-4">
  <div class="bg-white rounded-lg w-full h-full">
    <div class="flex w-full h-full">
      <div class="flex w-4/12 min-w-52 max-w-64 flex-col items-center justify-evenly ">
        <ConnConfig></ConnConfig>
        <div class="w-full h-1 bg-gray-100"></div>
        <OutputConfig></OutputConfig>
        <div class="w-full h-1 bg-gray-100"></div>
        <InputConfig></InputConfig>
      </div>
      <div class="w-1.5 h-full bg-gray-100"></div>
      <div class="flex flex-col w-full items-center">
        <OutputArea></OutputArea>
        <div class="w-full h-1 bg-gray-100"></div>
        <InputArea></InputArea>
      </div>
    </div>
  </div>
</div>