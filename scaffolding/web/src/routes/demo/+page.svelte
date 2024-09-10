<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import CardDetailGem from "$lib/components/CardDetailGem.svelte";
  import CardDisplay from "$lib/components/CardDisplay.svelte";
</script>

<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>


<div class="cards">
  <CardDisplay>
    <Card card_name={"emerald"} points={5}>
      <CardDetailGem gem_name={"emerald"} number={7}/>
      <CardDetailGem gem_name={"ruby"} number={5}/>
      <CardDetailGem gem_name={"diamond"} number={6}/>
    </Card>
    <Card card_name={"ruby"} points={7}>
      <CardDetailGem gem_name={"ruby"} number={2}/>
      <CardDetailGem gem_name={"sapphire"} number={3}/>
      <CardDetailGem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"sapphire"} points={6}>
      <CardDetailGem gem_name={"ruby"} number={2}/>
      <CardDetailGem gem_name={"sapphire"} number={3}/>
      <CardDetailGem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"onyx"} points={4}/>
  </CardDisplay>

  <CardDisplay>
    <Card card_name={"onyx"} points={2}/>
    <Card card_name={"diamond"} points={2}/>
    <Card card_name={"emerald"} points={3}/>
    <Card card_name={"sapphire"} points={2}/>
  </CardDisplay>

  <CardDisplay>
    <Card card_name={"ruby"} points={2}/>
    <Card card_name={"emerald"} points={0}/>
    <Card card_name={"diamond"} points={1}/>
    <Card card_name={"onyx"}/>
  </CardDisplay>
</div>

<style>
  .cards {
    max-width: 800px;
    max-height: 600px;
    margin-inline: auto;
    left: 0;
    right: 0;
  }


</style>
