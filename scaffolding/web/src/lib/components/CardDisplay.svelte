<script>
  import Card from "$lib/components/Card.svelte";
  import Gem from "$lib/components/CardDetailGem.svelte";
  import Row from "$lib/components/CardRow.svelte";
  import CardStack from "./CardStack.svelte"; 
</script>

<div class="cards">
  <Row>
    <Card card_name={"emerald"} points={5}>
      <Gem gem_name={"emerald"} number={7}/>
      <Gem gem_name={"ruby"} number={5}/>
      <Gem gem_name={"diamond"} number={6}/>
    </Card>
    <Card card_name={"ruby"} points={7}>
      <Gem gem_name={"ruby"} number={2}/>
      <Gem gem_name={"sapphire"} number={3}/>
      <Gem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"sapphire"} points={6}>
      <Gem gem_name={"ruby"} number={2}/>
      <Gem gem_name={"sapphire"} number={3}/>
      <Gem gem_name={"diamond"} number={1}/>
    </Card>
    <Card card_name={"onyx"} points={4}/>
    <CardStack count={31}/>
  </Row>

  <Row>
    <Card card_name={"onyx"} points={2}/>
    <Card card_name={"diamond"} points={2}/>
    <Card card_name={"emerald"} points={3}/>
    <Card card_name={"sapphire"} points={2}/>
    <CardStack count={0}/>
  </Row>

  <Row>
    <Card card_name={"ruby"} points={2}/>
    <Card card_name={"emerald"} points={0}/>
    <Card card_name={"diamond"} points={1}/>
    <Card card_name={"onyx"}/>
    <CardStack count={4}/>
  </Row>
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
