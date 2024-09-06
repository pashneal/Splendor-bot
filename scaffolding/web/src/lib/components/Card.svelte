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
 200px <!--adiv class="card-details"> -->
    <!--<p>testing testing</p>-->
    <!--<p>testing testing</p>-->
    <!--<p>testing testing</p>-->
  <!--</div>-->
</div>

<style>
  .card {
    width: 13%;
    height: 15%;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
  }
  
  img{
    width: 100%;
    height: 100%;
  }

  .card-details {
    max-width: 60%;
    bottom: 40%;
    max-height: 60%;
    position: relative;
    padding: 10%;
    margin-inline: auto;
    font-size : 0.7rem;
  }
</style>
