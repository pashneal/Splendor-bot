<script lang="ts">
  import emerald from "$lib/assets/token_emerald.svg";
  import sapphire from "$lib/assets/token_sapphire.svg";
  import diamond from "$lib/assets/token_diamond.svg";
  import ruby from "$lib/assets/token_ruby.svg";
  import onyx from "$lib/assets/token_onyx.svg";
  import gold from "$lib/assets/token_gold.svg";


  export let tokenName : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx" | "gold";
  export let numRemaining : number;

  let tokens = {
    "emerald": emerald,
    "sapphire": sapphire,
    "diamond": diamond,
    "ruby": ruby,
    "onyx": onyx,
    "gold": gold
  }

</script>

<div class="token-container">

  <div class="token">
    <img src={tokens[tokenName]} alt="Token" draggable="false"/>
  </div>

  {#each Array(numRemaining) as _,i}
    <div class="token-float-{i}">
      <img src={tokens[tokenName]} alt="Token" draggable="false"/>
    </div>
  {/each}

  <div class="num-remaining">
    {numRemaining}
  </div>
</div>


<style>
  .token-container {
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    text-align:center;
    width: 100px;
    height: 50px;
    border-radius: 20px;
    border: 1px dashed black;
    display: flex;
    flex-direction: row;
    position: relative;
  }

  img{
    width:40px;
    height:40px;
    margin: 5px;
  }

  .num-remaining {
    font-size: 30px;
    font-family: 'Zero';
    margin: auto;
  }

  .token-float-0 {
    position: absolute;
  }
  .token-float-1 {
    position: absolute;
    top: -2px;
    left: -2px;
  }
  .token-float-2 {
    position: absolute;
    top: -4px;
    left: -4px;
  }
  .token-float-3 {
    position: absolute;
    top: -6px;
    left: -6px;
  }
  .token-float-4 {
    position: absolute;
    top: -8px;
    left: -8px;
  }
  .token-float-5 {
    position: absolute;
    top: -10px;
    left: -10px;
  }
  .token-float-6 {
    position: absolute;
    top: -12px;
    left: -12px;
  }

</style>

