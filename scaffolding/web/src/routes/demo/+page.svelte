<script lang="ts">
  import CardDislay from "$lib/components/CardDisplay.svelte";
  import GemToken from "$lib/components/GemToken.svelte";
  import Bank from "$lib/components/Bank.svelte";
  import Noble from "$lib/components/Noble.svelte";
  import NobleDetail from "$lib/components/NobleDetails.svelte";
  import GemTokenSmall from "$lib/components/GemTokenSmall.svelte"; 
  import VDivider from "$lib/components/VerticalDivider.svelte";
  import HDivider from "$lib/components/HorizontalDivider.svelte";
  import Player from "$lib/components/Player.svelte";

  import { onMount } from "svelte";

  import { turnNumber } from "$lib/stores/replayStore"; 

  function nextMove() {
    turnNumber.update(n => n + 1);
  }

  function prevMove() {
    turnNumber.update(n => n - 1);
  }

  function updateMoveInput(move: number) {
    turnNumber.set(move);
    //refreshBoard();
  }
  export async function gotoMove(move: number) {
    fetch("/replay/goto", 
      {
        method : "POST", 
        body : JSON.stringify({"move_index": move }),  
        headers: {"Content-type": "application/json; charset=UTF-8"}
      }
    ).then((r) => r.json())
     .then(r => {updateMoveInput(r.success.move_index)});
  }

  onMount(() => {
    turnNumber.set(0);
    turnNumber.subscribe(value => {
      gotoMove(value);
      console.log("turnNumber", value);
    });
  });

</script>

<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>

<div class="top-bar">
  <button on:click={prevMove}>{"<"}</button>
  <input type="number"  id="moveInput" value={$turnNumber} />
  <button on:click={nextMove}>{">"}</button>
</div>

<div class="game">
  <div class="game-inner">
    <VDivider/>
    <Bank>
      <GemToken tokenName={"gold"} numRemaining={3} />
      <GemToken tokenName={"emerald"} numRemaining={6} />
      <GemToken tokenName={"diamond"} numRemaining={2} />
      <GemToken tokenName={"onyx"} numRemaining={5} />
      <GemToken tokenName={"ruby"} numRemaining={1} />
      <GemToken tokenName={"sapphire"} numRemaining={2} />
    </Bank>
    <VDivider/>
    <div>
      <CardDislay/>
    </div>
    <VDivider/>
    <div class="nobles">
      <Noble>
        <NobleDetail number={3} gem_name={"emerald"} />
        <NobleDetail number={3} gem_name={"diamond"} />
        <NobleDetail number={3} gem_name={"ruby"} />
      </Noble>
      <Noble>
        <NobleDetail number={4} gem_name={"emerald"} />
        <NobleDetail number={4} gem_name={"ruby"} />
      </Noble>
      <Noble>
        <NobleDetail number={3} gem_name={"onyx"} />
        <NobleDetail number={3} gem_name={"sapphire"} />
        <NobleDetail number={3} gem_name={"ruby"} />
      </Noble>
      <Noble>
        <NobleDetail number={4} gem_name={"emerald"} />
        <NobleDetail number={4} gem_name={"ruby"} />
      </Noble>
      <Noble>
        <NobleDetail number={3} gem_name={"emerald"} />
        <NobleDetail number={3} gem_name={"diamond"} />
        <NobleDetail number={3} gem_name={"ruby"} />
      </Noble>
    </div>
    <VDivider/>
    
  </div>

  <HDivider/>

  <div class="players">
    <Player avatar={0} name="pashneal">
      <GemTokenSmall tokenName={"gold"} numRemaining={3} />
      <GemTokenSmall tokenName={"emerald"} numRemaining={6} cardCount={1}/>
      <GemTokenSmall tokenName={"diamond"} numRemaining={2} cardCount={3}/>
      <GemTokenSmall tokenName={"onyx"} numRemaining={5} cardCount={2}/>
      <GemTokenSmall tokenName={"ruby"} numRemaining={1} />
      <GemTokenSmall tokenName={"sapphire"} numRemaining={2} />
    </Player>
    <Player avatar={1} name="amos">
      <GemTokenSmall tokenName={"gold"} numRemaining={3} />
      <GemTokenSmall tokenName={"emerald"} numRemaining={0} />
      <GemTokenSmall tokenName={"diamond"} numRemaining={2} />
      <GemTokenSmall tokenName={"onyx"} numRemaining={5} />
      <GemTokenSmall tokenName={"ruby"} numRemaining={1} />
      <GemTokenSmall tokenName={"sapphire"} numRemaining={2} />
    </Player>
    <Player avatar={2} name="izzie">
      <GemTokenSmall tokenName={"gold"} numRemaining={3} />
      <GemTokenSmall tokenName={"emerald"} numRemaining={6} />
      <GemTokenSmall tokenName={"diamond"} numRemaining={2} />
      <GemTokenSmall tokenName={"onyx"} numRemaining={5} />
      <GemTokenSmall tokenName={"ruby"} numRemaining={1} />
      <GemTokenSmall tokenName={"sapphire"} numRemaining={2} />
    </Player>
    <Player avatar={3} name="kiera">
      <GemTokenSmall tokenName={"gold"} numRemaining={2} />
      <GemTokenSmall tokenName={"emerald"} numRemaining={6} />
      <GemTokenSmall tokenName={"diamond"} numRemaining={2} />
      <GemTokenSmall tokenName={"onyx"} numRemaining={5} />
      <GemTokenSmall tokenName={"ruby"} numRemaining={1} />
      <GemTokenSmall tokenName={"sapphire"} numRemaining={2} />
    </Player>
  </div>

</div>


<style>
  .game {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }
  .game-inner {
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 70rem;
    height: 40rem;
  }

  .nobles {
    display: flex;
    flex-direction: column;
    align-items: top;
    gap: 5%;
    width: 20%;
    height: 70%;
  }

  .players {
    flex-direction : row;
    display: flex;
  }

</style>
