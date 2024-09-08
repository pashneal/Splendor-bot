<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Gem from "$lib/components/CardDetailGem.svelte";
</script>

<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>


<div class="cards">
  <Card card_name={"emerald"}>
    <Gem gem_name={"emerald"} number={7}/>
    <Gem gem_name={"ruby"} number={2}/>
    <Gem gem_name={"diamond"} number={1}/>
    <Gem gem_name={"ruby"} number={2}/>
  </Card>
  <Card card_name={"ruby"}/>
  <Card card_name={"sapphire"}/>
  <Card card_name={"onyx"}/>
</div>
<div class="cards">
  <Card card_name={"onyx"}/>
  <Card card_name={"diamond"}/>
  <Card card_name={"emerald"}/>
  <Card card_name={"sapphire"}/>
</div>
<div class="cards">
  <Card card_name={"ruby"}/>
  <Card card_name={"emerald"}/>
  <Card card_name={"diamond"}/>
  <Card card_name={"onyx"}/>
</div>

<style>
  .cards {
    display: flex;
    justify-content: center;
    gap: 1rem;
    row-gap: 1rem;
  }
</style>
