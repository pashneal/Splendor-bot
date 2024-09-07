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

<div class="card-detail-container">
    <img src={gems[gem_name]} alt="Gem" draggable="false"/>
  <div class="pip-container"> 
  </div>
</div>



<style>
  .card-detail-container {
    width: 100%;
    height: 15%;
    position : relative;
    background-color: white;
    border-radius: 1vw;
  }

  img{
    width: 30%;
    height: 150%;
    position: absolute;
    left: -5%;
    top: -25%;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
  }

</style>
