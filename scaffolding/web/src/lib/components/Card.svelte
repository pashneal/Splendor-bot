<script lang="ts">
  import emerald_card from "$lib/assets/emerald_card.svg";
  import sapphire_card from "$lib/assets/sapphire_card.svg";
  import diamond_card from "$lib/assets/diamond_card.svg";
  import ruby_card from "$lib/assets/ruby_card.svg";
  import onyx_card from "$lib/assets/onyx_card.svg";
  export let card_name : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx";

  let cards = {
    "emerald": emerald_card,
    "sapphire": sapphire_card,
    "diamond": diamond_card,
    "ruby": ruby_card,
    "onyx": onyx_card
  }
</script>

<div class="card">
  <img src={cards[card_name]} alt="Card" draggable="false"/>
  <div class="card-details"> 
    <slot></slot>
  </div>
</div>

<style>

  .card {
    max-width: 20%;
    max-height: 15%;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    position: relative;
  }
  
  img{
    max-width: 100%;
    max-height: 100%;
  }

  .card-details {
    top: 0%;
    left: 0;
    right: 0;
    max-width: 90%;
    height: 50%;
    margin-inline: auto;
    position: absolute;
    font-size : 0.7rem;
  }

</style>
