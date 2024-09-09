<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Gem from "$lib/components/CardDetailGem.svelte";
  import CardDisplay from "$lib/components/CardDisplay.svelte";
</script>

<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>


<div class="cards">
  <CardDisplay>
    <Card card_name={"emerald"}>
      <Gem gem_name={"emerald"} number={7}/>
      <Gem gem_name={"ruby"} number={2}/>
      <Gem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"ruby"}>
      <Gem gem_name={"ruby"} number={2}/>
      <Gem gem_name={"sapphire"} number={3}/>
      <Gem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"sapphire"}>
      <Gem gem_name={"ruby"} number={2}/>
      <Gem gem_name={"sapphire"} number={3}/>
      <Gem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"onyx"}/>
  </CardDisplay>

  <CardDisplay>
    <Card card_name={"onyx"}/>
    <Card card_name={"diamond"}/>
    <Card card_name={"emerald"}/>
    <Card card_name={"sapphire"}/>
  </CardDisplay>

  <CardDisplay>
    <Card card_name={"ruby"}/>
    <Card card_name={"emerald"}/>
    <Card card_name={"diamond"}/>
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

  @media (max-width: 640px) {
    .cards {
      max-width: 480px;
    }
  }

  @media (max-height: 640px) {
    .cards {
      max-height: 480px;
    }
  }

</style>
