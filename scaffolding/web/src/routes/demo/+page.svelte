<script lang="ts">
  import CardDislay from "$lib/components/CardDisplay.svelte";
  import GemToken from "$lib/components/GemToken.svelte";
  import Bank from "$lib/components/Bank.svelte";
</script>

<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>

<div class="game">
  <Bank>
    <GemToken tokenName={"gold"} numRemaining={3} />
    <GemToken tokenName={"emerald"} numRemaining={6} />
    <GemToken tokenName={"diamond"} numRemaining={2} />
    <GemToken tokenName={"onyx"} numRemaining={5} />
    <GemToken tokenName={"ruby"} numRemaining={1} />
    <GemToken tokenName={"sapphire"} numRemaining={2} />
  </Bank>
  <div>
    <CardDislay/>
  </div>
</div>

<style>
  .game {
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 80%;
    height: 80%;
    gap: 5%;
  }



</style>
