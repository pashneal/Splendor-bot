<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Gem from "$lib/components/CardDetailGem.svelte";
  import Row from "$lib/components/CardRow.svelte";
  import CardStack from "./CardStack.svelte"; 
  import { cards , deckCounts} from "$lib/stores/replayStore";

</script>

<div class="cards">
  {#each $cards.reverse() as row, i}
    <Row>
      {#each row as card}
        <Card card_name={card.gem} points={card.points}>
          {#each card.cost.entries() as [gemName, num]}
            <Gem gem_name={gemName} number={num}/>
          {/each}
        </Card>
      {/each}
      <CardStack count={$deckCounts[i]}/>
    </Row>
  {/each}
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
