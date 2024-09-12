<script lang="ts">
  import Avatar from "./Avatar.svelte"; 
  import Bank from "./Bank.svelte";
  import GemTokenSmall from "./GemTokenSmall.svelte";

  export let name;
  export let avatar: number = 0;
</script>

<div class="player-container">
  <Avatar avatar={avatar} name={name} />
  <div class="gems-container">
      <GemTokenSmall tokenName={"gold"} numRemaining={3} />
      <GemTokenSmall tokenName={"emerald"} numRemaining={6} />
      <GemTokenSmall tokenName={"diamond"} numRemaining={2} />
      <GemTokenSmall tokenName={"onyx"} numRemaining={5} />
      <GemTokenSmall tokenName={"ruby"} numRemaining={1} />
      <GemTokenSmall tokenName={"sapphire"} numRemaining={2} />
  </div>
</div>

<style>
  .player-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 25%;
    height: 20%;
  }
  .gems-container {
    width: 200px;
    flex-direction: row;
    flex-wrap: wrap;
    display: flex;
    height: 100%;
    gap: 2%;
  }

</style>

