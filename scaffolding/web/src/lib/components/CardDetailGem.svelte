<script lang="ts"> 
  import emerald from "$lib/assets/emerald.svg";
  import sapphire from "$lib/assets/sapphire.svg";
  import diamond from "$lib/assets/diamond.svg";
  import ruby from "$lib/assets/ruby.svg";
  import onyx from "$lib/assets/onyx.svg";


  export let gem_name : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx";
  export let number : number;

  let gems = {
    "emerald": emerald,
    "sapphire": sapphire,
    "diamond": diamond,
    "ruby": ruby,
    "onyx": onyx
  }
</script>

<div class="gem">
  {number}
  <img src={gems[gem_name]} alt="Gem" draggable="false"/>
</div>



<style>
  .gem {
    width: 30px;
    height: 30px;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    position: relative;
  }
   
  img{
    width: 100%;
    height: 100%;
  }
</style>
