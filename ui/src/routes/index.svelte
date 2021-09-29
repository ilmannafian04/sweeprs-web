<script lang="ts">
  let code = '';
  let joinHandler = () => {
    console.log(code);
  };
</script>

<h1>Sweeprs</h1>
<button>Create</button>
<span>- OR -</span>
<form on:submit|preventDefault={joinHandler}>
  <input bind:value={code} />
  <button type="submit">Join</button>
</form>
