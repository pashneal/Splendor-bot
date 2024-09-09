<script>
</script>

<div class="display">
  <slot></slot>
</div>

<style>
  .display {
    display: flex;
    justify-content: center;
    object-fit: contain;
    gap: 1rem;
    row-gap: 1rem;
    max-width: 100%;
    max-height: 30%;
  }

  @media (max-width: 640px) {
    .display {
      max-width: 480px;
      max-height: 20%;
    }
  }
</style>
